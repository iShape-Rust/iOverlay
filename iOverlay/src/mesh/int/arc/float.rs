use super::{ArcDirection, ArcMath, ArcOptions};
use alloc::vec::Vec;
use core::f64::consts::TAU;
use i_float::float::number::FloatNumber;
#[cfg(test)]
use i_float::int::angle::Angle;
use i_float::int::number::{int::IntNumber, wide_int::WideIntNumber};
use i_float::int::unit_vector::UnitIntVector;

#[derive(Clone, Copy, Debug)]
struct FloatUnitVector {
    x: f64,
    y: f64,
}

impl FloatUnitVector {
    #[cfg(test)]
    fn cross(self, other: Self) -> f64 {
        self.x * other.y - self.y * other.x
    }
    #[cfg(test)]
    fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y
    }
}

/// Cached f64 rotations with reusable storage. Endpoints remain the original
/// integer contact points in the caller, never reconstructed from these rays.
/// Direction lengths are approximate and may drift slightly above one.
pub(crate) struct FloatArc<I: IntNumber> {
    step: f64,
    angle_error: f64,
    sin: f64,
    cos: f64,
    directions: Vec<UnitIntVector<I>>,
}

impl<I: IntNumber> FloatArc<I> {
    const MAX_ROTATIONS: usize = 1536;
    // A generous angular budget for atan2 and at most 1536 matrix applications.
    // The smaller cached step leaves room for this error in the final gap.
    const ANGLE_ERROR: f64 = 64.0 * Self::MAX_ROTATIONS as f64 * f64::EPSILON;
}

impl<I: IntNumber> ArcMath<UnitIntVector<I>> for FloatArc<I> {
    fn new(options: ArcOptions) -> Self {
        let options = options.clamped();
        let max_step = options.max_step.bits() as f64 * (TAU / 4294967296.0);
        // Component truncation adds at most about sqrt(2)/S angular error
        // per output ray; reserve 2/S for each end of a gap.
        let angle_error = Self::ANGLE_ERROR + 2.0 / UnitIntVector::<I>::DENOMINATOR.to_f64();
        let step = max_step - 2.0 * angle_error;
        let (sin, cos) = FloatNumber::sin_cos(step);
        Self {
            step,
            angle_error,
            sin,
            cos,
            directions: Vec::new(),
        }
    }

    fn build(
        &mut self,
        from: UnitIntVector<I>,
        to: UnitIntVector<I>,
        direction: ArcDirection,
    ) -> &[UnitIntVector<I>] {
        self.directions.clear();
        let a = crate::mesh::int::math::vector(from);
        let b = crate::mesh::int::math::vector(to);
        let cross = a.cross_product(b).to_f64();
        let dot = a.dot_product(b).to_f64();
        let scale = UnitIntVector::<I>::DENOMINATOR.to_f64();
        let from = float_vector(from);
        let sign = match direction {
            ArcDirection::Clockwise => -1.0,
            ArcDirection::Counterclockwise => 1.0,
        };
        let cross = sign * cross;
        // Conservatively skip short arcs without atan2. Account for unit-vector
        // normalization error when comparing the dot product with cos(step).
        if cross >= 0.0 && dot / (scale * scale) >= self.cos + 8.0 * f64::EPSILON {
            return &self.directions;
        }
        let mut sweep = FloatNumber::atan2(cross, dot);
        if sweep < 0.0 {
            sweep += TAU;
        }
        // The quotient is nonnegative, so truncation is equivalent to floor.
        let count = ((sweep - self.angle_error).max(0.0) / self.step) as usize;
        debug_assert!(count <= Self::MAX_ROTATIONS);
        self.directions.reserve(count);
        let sin = sign * self.sin;
        let mut current = from;
        for _ in 0..count {
            current = FloatUnitVector {
                x: self.cos * current.x - sin * current.y,
                y: sin * current.x + self.cos * current.y,
            };
            // Keep f64 rotation state between steps; quantization does not
            // accumulate. Small length drift from float rounding is accepted.
            self.directions
                .push(UnitIntVector::from_float_unchecked(current.x, current.y));
        }
        &self.directions
    }
}

fn float_vector<I: IntNumber>(direction: UnitIntVector<I>) -> FloatUnitVector {
    let scale = UnitIntVector::<I>::DENOMINATOR.to_f64();
    FloatUnitVector {
        x: direction.x().to_wide().to_f64() / scale,
        y: direction.y().to_wide().to_f64() / scale,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_arcs<I: IntNumber>() {
        let scale = UnitIntVector::<I>::DENOMINATOR;
        let unit = |angle: f64| {
            let (sin, cos) = FloatNumber::sin_cos(angle);
            UnitIntVector::<I>::from_float_unchecked(cos, sin)
        };
        let sweep = |a: UnitIntVector<I>, b: UnitIntVector<I>, sign: f64| {
            let a = float_vector(a);
            let b = float_vector(b);
            let angle = FloatNumber::atan2(sign * a.cross(b), a.dot(b));
            if angle < 0.0 { angle + TAU } else { angle }
        };
        for bits in [ArcOptions::MIN_STEP.bits(), 1 << 26, ArcOptions::MAX_STEP.bits()] {
            let max_step = bits as f64 * TAU / 4294967296.0;
            let mut arc = FloatArc::<I>::new(ArcOptions {
                max_step: Angle::from_bits(bits),
                rotation_precision: 5,
            });
            for start in [0.0, 0.3, 1.7, 3.1, 4.9] {
                for extent in [
                    0.0,
                    1e-8,
                    max_step,
                    max_step + 1e-6,
                    1.0,
                    core::f64::consts::PI,
                    TAU - 1e-8,
                ] {
                    for direction in [ArcDirection::Clockwise, ArcDirection::Counterclockwise] {
                        let sign = if direction == ArcDirection::Clockwise {
                            -1.0
                        } else {
                            1.0
                        };
                        let from = unit(start);
                        let to = unit(start + sign * extent);
                        let total = sweep(from, to, sign);
                        let mut previous = 0.0;
                        for &point in arc.build(from, to, direction) {
                            let x = point.x().to_f64() / scale.to_f64();
                            let y = point.y().to_f64() / scale.to_f64();
                            let norm = FloatNumber::sqrt(x * x + y * y);
                            let tolerance = FloatArc::<I>::ANGLE_ERROR + 4.0 / scale.to_f64();
                            assert!((norm - 1.0).abs() <= tolerance);
                            let progress = sweep(from, point, sign);
                            assert!(
                                progress > previous && progress < total,
                                "bits={}, progress={progress}, previous={previous}, total={total}",
                                I::BITS
                            );
                            assert!(progress - previous <= max_step + 1e-12);
                            previous = progress;
                        }
                        assert!(total - previous <= max_step + 1e-12);
                    }
                }
            }
            arc.build(unit(0.0), unit(3.0), ArcDirection::Clockwise);
            let capacity = arc.directions.capacity();
            let ptr = arc.directions.as_ptr();
            for _ in 0..3 {
                assert!(
                    arc.build(unit(0.0), unit(0.0), ArcDirection::Clockwise)
                        .is_empty()
                );
                arc.build(unit(0.0), unit(3.0), ArcDirection::Clockwise);
                assert_eq!(arc.directions.capacity(), capacity);
                assert_eq!(arc.directions.as_ptr(), ptr);
            }
        }
    }

    #[test]
    fn arc_bounds_and_reuse_i16() {
        check_arcs::<i16>();
    }
    #[test]
    fn arc_bounds_and_reuse_i32() {
        check_arcs::<i32>();
    }
    #[test]
    fn arc_bounds_and_reuse_i64() {
        check_arcs::<i64>();
    }
}
