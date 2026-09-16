use super::{ArcDirection, ArcMath, ArcOptions};
use alloc::vec::Vec;
use i_float::int::angle::{Angle, Rotation};
use i_float::int::number::{int::IntNumber, wide_int::WideIntNumber};
use i_float::int::unit_vector::UnitIntVector;

/// Reusable storage and two cached rotation matrices for directed arcs.
///
/// Output contains intermediate directions in traversal order, excluding both
/// input endpoints. The caller applies the center and radius and retains the
/// original contact points to avoid rounding seams. Equal rays describe an
/// empty arc; full turns are not supported. Opposite rays describe a semicircle
/// whose side is selected by `ArcDirection`. Major arcs are supported too.
///
/// Matrices are constructed once, with [`Rotation::with_precision`]. Each build
/// measures the sweep with [`Angle::between`] and applies the appropriate matrix
/// repeatedly, without per-point CORDIC, square roots, or normalization. Steps
/// use the achieved matrix angle; the last gap can be shorter than the others.
/// Reversing a build need not produce the same intermediate directions.
///
/// Inputs should be freshly normalized directions: length at least 0.99 for
/// `i32`/`i64`, or 0.95 for `i16`. The step error budget includes rounding during
/// at most 1536 applications. `i16` also checks the endpoint geometrically;
/// its coarse storage can noticeably contract long fine arcs. For those arcs,
/// prefer `i32` or `i64`. Output length never exceeds the starting length.
///
/// ```
/// use i_float::int::vector::IntVector;
/// use i_overlay::mesh::int::arc::{ArcBuilder, ArcDirection, ArcOptions};
///
/// let from = IntVector::<i32>::new(1, 0).fast_normalize().unwrap();
/// let to = IntVector::<i32>::new(0, 1).fast_normalize().unwrap();
/// let mut builder = ArcBuilder::new(ArcOptions::default());
/// let directions = builder.build(from, to, ArcDirection::Counterclockwise);
/// assert!(!directions.is_empty());
/// let offset = directions[0].scale(1024);
/// assert!(offset.x > 0 && offset.y > 0);
/// ```
pub struct IntegerArc<I: IntNumber = i32> {
    options: ArcOptions,
    short_arc_dot: I::Wide,
    clockwise: ArcRotation<I>,
    counterclockwise: ArcRotation<I>,
    directions: Vec<UnitIntVector<I>>,
}

struct ArcRotation<I: IntNumber> {
    matrix: Rotation<I>,
    upper_step: u64,
}

impl<I: IntNumber> IntegerArc<I> {
    const MAX_ROTATIONS: u64 = 1536;
    // i32/i64: two metadata units plus component rounding at length >= 0.99.
    // i16: metadata plus component rounding, even after contraction to 0.68.
    const STEP_ERROR: u64 = if I::BITS == 16 { 1 << 18 } else { 4 };

    /// Prepares both traversal directions. The result buffer starts empty.
    pub fn new(options: ArcOptions) -> Self {
        let options = options.clamped();
        // Counting by the upper step prevents endpoint overshoot. Budget the
        // accumulated difference from the lower step in the final gap too.
        // i16 stops geometrically, so it needs only a single-step reserve.
        let reserve = if I::BITS == 16 {
            2 * Self::STEP_ERROR
        } else {
            2 * Self::STEP_ERROR * (Self::MAX_ROTATIONS + 1) + 2 * Angle::MAX_ERROR as u64
        };
        let divisor = 1u64 << options.rotation_precision;
        let available = options.max_step.bits() as u64 - reserve;
        let requested = (available * divisor / (divisor + 1)) as u32;
        let prepare = |bits| {
            let matrix = Rotation::<I>::with_precision(Angle::from_bits(bits), options.rotation_precision);
            let step = (matrix.angle().bits() as i32).unsigned_abs() as u64;
            debug_assert!(step > Self::STEP_ERROR);
            ArcRotation {
                matrix,
                upper_step: step + Self::STEP_ERROR,
            }
        };
        // A sufficient small-arc test: input norms are <=1, so an unnormalized
        // dot above cos(max_step) also bounds the normalized dot from below.
        // Add eight Q30 units to cover the full CORDIC coefficient error, then
        // round upward when narrowing to the Q28 dot scale of i16.
        let cosine = options.max_step.cos() as u32 + 8;
        let dot_bits = 2 * (I::BITS - 2);
        let short_arc_dot = if dot_bits >= 30 {
            I::Wide::from_u32(cosine) << (dot_bits - 30)
        } else {
            I::Wide::from_u32(cosine.div_ceil(1 << (30 - dot_bits)))
        };
        Self {
            options,
            short_arc_dot,
            clockwise: prepare(requested.wrapping_neg()),
            counterclockwise: prepare(requested),
            directions: Vec::new(),
        }
    }

    /// Returns the effective settings, after clamping to the supported range.
    pub fn options(&self) -> ArcOptions {
        self.options
    }

    /// Clears the previous result, retaining its capacity, and returns the
    /// intermediate directions. Settings and cached matrices remain reusable.
    pub fn build(
        &mut self,
        from: UnitIntVector<I>,
        to: UnitIntVector<I>,
        direction: ArcDirection,
    ) -> &[UnitIntVector<I>] {
        self.directions.clear();
        // Most shallow joins need no intermediate points. Avoid vectoring for
        // those arcs, while distinguishing the directed minor and major arcs.
        if cross(from, to, direction) >= I::Wide::ZERO && dot(from, to) >= self.short_arc_dot {
            return &self.directions;
        }
        let (sweep, rotation) = match direction {
            ArcDirection::Clockwise => (Angle::between(to, from), &self.clockwise),
            ArcDirection::Counterclockwise => (Angle::between(from, to), &self.counterclockwise),
        };
        if sweep.bits() == 0
            || sweep.bits() as u64 + Angle::MAX_ERROR as u64 <= self.options.max_step.bits() as u64
        {
            return &self.directions;
        }

        if I::BITS == 16 {
            // Low precision storage accumulates too much error for a fixed
            // count. Stop just before the next rotated direction reaches the
            // endpoint. Each step is <90 degrees, so cross/dot suffice.
            let mut current = from;
            for _ in 0..Self::MAX_ROTATIONS {
                let next = rotation.matrix.apply(current);
                let near_end =
                    cross(current, to, direction) >= I::Wide::ZERO && dot(current, to) >= I::Wide::ZERO;
                if cross(current, next, direction) <= I::Wide::ZERO
                    || (near_end && cross(next, to, direction) <= I::Wide::ZERO)
                {
                    return &self.directions;
                }
                self.directions.push(next);
                current = next;
            }
            debug_assert!(false, "freshly normalized directions must reach the arc endpoint");
        } else {
            // Strictly below the lower bound on the sweep: both input endpoints
            // are excluded even when the arc is an exact multiple of the step.
            let lower = sweep.bits().saturating_sub(Angle::MAX_ERROR) as u64;
            let count = lower.saturating_sub(1) / rotation.upper_step;
            debug_assert!(count < Self::MAX_ROTATIONS);
            self.directions.reserve(count as usize);
            let mut current = from;
            for _ in 0..count {
                current = rotation.matrix.apply(current);
                self.directions.push(current);
            }
        }
        &self.directions
    }
}

impl<I: IntNumber> Default for IntegerArc<I> {
    fn default() -> Self {
        Self::new(ArcOptions::default())
    }
}

impl<I: IntNumber> ArcMath<UnitIntVector<I>> for IntegerArc<I> {
    fn new(options: ArcOptions) -> Self {
        IntegerArc::new(options)
    }
    fn build(
        &mut self,
        from: UnitIntVector<I>,
        to: UnitIntVector<I>,
        direction: ArcDirection,
    ) -> &[UnitIntVector<I>] {
        IntegerArc::build(self, from, to, direction)
    }
}

fn cross<I: IntNumber>(a: UnitIntVector<I>, b: UnitIntVector<I>, direction: ArcDirection) -> I::Wide {
    let cross = a.x().to_wide() * b.y().to_wide() - a.y().to_wide() * b.x().to_wide();
    match direction {
        ArcDirection::Counterclockwise => cross,
        ArcDirection::Clockwise => -cross,
    }
}

fn dot<I: IntNumber>(a: UnitIntVector<I>, b: UnitIntVector<I>) -> I::Wide {
    a.x().to_wide() * b.x().to_wide() + a.y().to_wide() * b.y().to_wide()
}

#[cfg(test)]
mod tests {
    use super::{ArcDirection, ArcOptions, IntegerArc};
    use i_float::int::vector::IntVector;

    #[test]
    fn repeated_builds_reuse_buffer_and_rotations() {
        let from = IntVector::<i32>::new(1, 0).fast_normalize().unwrap();
        let to = IntVector::<i32>::new(0, -1).fast_normalize().unwrap();
        let mut builder = IntegerArc::new(ArcOptions {
            max_step: ArcOptions::MIN_STEP,
            ..ArcOptions::default()
        });
        builder.build(from, to, ArcDirection::Counterclockwise);
        let output_ptr = builder.directions.as_ptr();
        let output_capacity = builder.directions.capacity();
        let cw = builder.clockwise.matrix.apply(from);
        let ccw = builder.counterclockwise.matrix.apply(from);
        assert!(output_capacity > 0);
        for _ in 0..3 {
            assert!(builder.build(from, from, ArcDirection::Clockwise).is_empty());
            builder.build(from, to, ArcDirection::Counterclockwise);
            assert_eq!(builder.directions.as_ptr(), output_ptr);
            assert_eq!(builder.directions.capacity(), output_capacity);
            assert_eq!(builder.clockwise.matrix.apply(from), cw);
            assert_eq!(builder.counterclockwise.matrix.apply(from), ccw);
        }
    }
}
