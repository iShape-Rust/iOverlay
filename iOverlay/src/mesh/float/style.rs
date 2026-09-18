use crate::mesh::int::arc::ArcOptions;
use crate::mesh::int::style::{IntLineCap, IntLineJoin, IntStrokeStyle};
use crate::mesh::math::MathMode;
use alloc::rc::Rc;
use alloc::vec::Vec;
use core::f64::consts::PI;
use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_float::int::angle::Angle;
use i_float::int::number::int::IntNumber;
use i_float::int::point::IntPoint;

pub(super) fn angle_from_radians<T: FloatNumber>(radians: T) -> Angle {
    Angle::from_radians(radians).unwrap_or_else(|| {
        // Normalization clamps infinities, but preserves NaN. Use the same
        // minimum as floating-point joins and caps instead of panicking.
        Angle::from_radians(T::from_float(0.01 * PI)).expect("minimum style angle is finite")
    })
}

/// The endpoint style of a line.
#[derive(Debug, Clone)]
pub enum LineCap<P: FloatPointCompatible> {
    /// A line with a squared-off end. This is the default.
    Butt,
    /// A line with a rounded end. The line ends with a semicircular arc with a radius of 1/2 the line’s width, centered on the endpoint.
    /// Takes a parameter `Angle` in radians.
    Round(P::Scalar),
    /// A line with a squared-off end. An extended distance equal to half the line width.
    Square,
    /// Set a custom end with template points.
    Custom(Rc<[P]>),
}

/// The join style of a line.
#[derive(Debug, Clone)]
pub enum LineJoin<T: FloatNumber> {
    /// Cuts off the corner where two lines meet. This is the default.
    Bevel,
    /// Creates a sharp corner where two lines meet.
    /// The parameter is the minimum interior angle in radians, clamped to
    /// 0.01*pi..=0.99*pi (1.8..=178.2 degrees) before integer conversion.
    /// With Integer construction math, the effective minimum is at least
    /// 5 degrees, and interior angles above 175 degrees use bevel joins.
    /// Float construction math uses the converted angle without that cutoff.
    Miter(T),
    /// Creates an arc corner where two lines meet.
    /// The arc is approximated using a group of segments, where the parameter `Angle`
    /// is defined as `L / R`, with `L` being the maximum segment length and `R` being the arc radius.
    Round(T),
}

/// Defines the stroke style for outlining paths.
#[derive(Debug, Clone)]
pub struct StrokeStyle<P: FloatPointCompatible> {
    /// The width of the stroke.
    pub width: P::Scalar,
    /// The cap style at the start of the stroke.
    pub start_cap: LineCap<P>,
    /// The cap style at the end of the stroke.
    pub end_cap: LineCap<P>,
    /// The join style where two lines meet.
    pub join: LineJoin<P::Scalar>,
    /// Arithmetic for stroke construction. Integer remains the default.
    pub math: MathMode,
}

/// Defines the outline style for offsetting shapes.
#[derive(Debug)]
pub struct OutlineStyle<T: FloatNumber> {
    pub outer_offset: T,
    pub inner_offset: T,
    pub join: LineJoin<T>,
    /// Arithmetic used to construct offsets and joins.
    pub math: MathMode,
}

impl<P: FloatPointCompatible> LineCap<P> {
    pub(crate) fn normalize(self) -> Self {
        if let LineCap::Round(angle) = self {
            let a = angle.to_f64().clamp(0.01 * PI, 0.25 * PI);
            LineCap::Round(P::Scalar::from_float(a))
        } else {
            self
        }
    }
}

impl<T: FloatNumber> From<&LineJoin<T>> for IntLineJoin {
    /// Converts a floating-point join, normalizing its angle before quantization.
    fn from(join: &LineJoin<T>) -> Self {
        match join.clone().normalize() {
            LineJoin::Bevel => IntLineJoin::Bevel,
            LineJoin::Miter(a) => IntLineJoin::Miter(angle_from_radians(a)),
            LineJoin::Round(a) => IntLineJoin::Round(ArcOptions {
                max_step: angle_from_radians(a),
                ..ArcOptions::default()
            }),
        }
    }
}

impl<T: FloatNumber> LineJoin<T> {
    /// Conservative multiplier for the join's reach relative to the offset radius.
    pub(super) fn padding_factor(&self) -> f64 {
        match IntLineJoin::from(self) {
            IntLineJoin::Miter(minimum) => {
                let sin = Angle::from_bits(minimum.bits() / 2).sin() as f64;
                1.1 * (1u32 << 30) as f64 / sin
            }
            _ => 1.1,
        }
    }

    pub(crate) fn normalize(self) -> Self {
        match self {
            LineJoin::Miter(ratio) => {
                let a = ratio.to_f64().clamp(0.01 * PI, 0.99 * PI);
                LineJoin::Miter(T::from_float(a))
            }
            LineJoin::Round(angle) => {
                let a = angle.to_f64().clamp(0.01 * PI, 0.25 * PI);
                LineJoin::Round(T::from_float(a))
            }
            _ => self,
        }
    }
}

impl<P: FloatPointCompatible> StrokeStyle<P> {
    /// Creates a new `StrokeStyle` with the specified width.
    pub fn new(width: P::Scalar) -> Self {
        Self {
            width,
            ..Default::default()
        }
    }

    /// Sets the stroke width.
    pub fn width(mut self, width: P::Scalar) -> Self {
        self.width = P::Scalar::from_float(width.to_f64().max(0.0));
        self
    }

    /// Sets the cap style at the start of the stroke.
    pub fn start_cap(mut self, cap: LineCap<P>) -> Self {
        self.start_cap = cap.normalize();
        self
    }

    /// Sets the cap style at the end of the stroke.
    pub fn end_cap(mut self, cap: LineCap<P>) -> Self {
        self.end_cap = cap.normalize();
        self
    }

    /// Sets the line join style.
    pub fn line_join(mut self, join: LineJoin<P::Scalar>) -> Self {
        self.join = join.normalize();
        self
    }

    /// Selects construction arithmetic; the final boolean operation stays integer.
    pub fn math(mut self, math: MathMode) -> Self {
        self.math = math;
        self
    }

    pub(super) fn to_int<I: IntNumber>(&self, adapter: &FloatPointAdapter<P, I>) -> IntStrokeStyle<I> {
        let radius = P::Scalar::from_float(0.5 * self.width.to_f64().max(0.0));
        let cap = |cap: &LineCap<P>| match cap.clone().normalize() {
            LineCap::Butt => IntLineCap::Butt,
            LineCap::Square => IntLineCap::Square,
            LineCap::Round(a) => IntLineCap::Round(ArcOptions {
                max_step: angle_from_radians(a),
                ..ArcOptions::default()
            }),
            LineCap::Custom(points) => IntLineCap::Custom(
                points
                    .iter()
                    .map(|p| {
                        IntPoint::new(
                            adapter.round_len_to_int(p.x() * radius),
                            adapter.round_len_to_int(p.y() * radius),
                        )
                    })
                    .collect::<Vec<_>>()
                    .into(),
            ),
        };
        let radius = adapter.round_len_to_int(radius);
        IntStrokeStyle {
            width: I::from_wide(radius.to_wide() + radius.to_wide()),
            start_cap: cap(&self.start_cap),
            end_cap: cap(&self.end_cap),
            join: IntLineJoin::from(&self.join),
            math: self.math,
        }
    }

    /// Conservative distance by which the stroke can extend beyond its input bounds.
    pub(super) fn padding(&self) -> P::Scalar {
        let cap = |cap: &LineCap<P>| match cap {
            LineCap::Square => 2.0,
            LineCap::Custom(points) => {
                points
                    .iter()
                    .map(|p| {
                        let x = p.x().to_f64();
                        let y = p.y().to_f64();
                        FloatNumber::sqrt(x * x + y * y)
                    })
                    .fold(1.0_f64, f64::max)
                    * 1.1
            }
            _ => 1.1,
        };
        let r = 0.5 * self.width.to_f64().max(0.0);

        let join_factor = self.join.padding_factor();
        let start_cap_factor = cap(&self.start_cap);
        let end_cap_factor = cap(&self.end_cap);

        let factor = join_factor.max(start_cap_factor).max(end_cap_factor);
        P::Scalar::from_float(r * factor)
    }
}

impl<P: FloatPointCompatible> Default for StrokeStyle<P> {
    fn default() -> Self {
        Self {
            width: P::Scalar::from_float(1.0),
            start_cap: LineCap::Butt,
            end_cap: LineCap::Butt,
            join: LineJoin::Bevel,
            math: MathMode::Integer,
        }
    }
}

impl<T: FloatNumber> OutlineStyle<T> {
    /// Creates a new `OutlineStyle` with the specified offset.
    pub fn new(offset: T) -> Self {
        Self {
            outer_offset: offset,
            inner_offset: offset,
            ..Default::default()
        }
    }

    /// Selects construction arithmetic; boolean operations stay integer.
    pub fn math(mut self, math: MathMode) -> Self {
        self.math = math;
        self
    }

    /// Sets the offset distance.
    pub fn offset(mut self, offset: T) -> Self {
        self.outer_offset = offset;
        self.inner_offset = offset;
        self
    }

    /// Sets the outer distance.
    pub fn outer_offset(mut self, outer_offset: T) -> Self {
        self.outer_offset = outer_offset;
        self
    }

    /// Sets the inner distance.
    pub fn inner_offset(mut self, inner_offset: T) -> Self {
        self.inner_offset = inner_offset;
        self
    }

    /// Sets the line join style for the offset path.
    pub fn line_join(mut self, join: LineJoin<T>) -> Self {
        self.join = join;
        self
    }
}

impl<T: FloatNumber> Default for OutlineStyle<T> {
    fn default() -> Self {
        Self {
            outer_offset: T::from_float(1.0),
            inner_offset: T::from_float(1.0),
            join: LineJoin::Bevel,
            math: MathMode::Integer,
        }
    }
}
