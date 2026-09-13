use core::f64::consts::PI;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;

/// A point on a variable-width centerline.
#[derive(Debug, Clone, Copy)]
pub struct StrokeVertex<P: FloatPointCompatible> {
    pub point: P,
    pub width: P::Scalar,
}

impl<P: FloatPointCompatible> StrokeVertex<P> {
    #[inline]
    pub fn new(point: P, width: P::Scalar) -> Self {
        Self { point, width }
    }

    #[inline]
    pub(super) fn radius(&self) -> P::Scalar {
        P::Scalar::HALF * self.width.max(P::Scalar::ZERO)
    }
}

/// Round-only style for variable-width strokes.
#[derive(Debug, Clone, Copy)]
pub struct VariableStrokeStyle<T: FloatNumber> {
    /// Maximum angular step used to approximate round joins and caps, in radians.
    pub round_angle: T,
}

impl<T: FloatNumber> VariableStrokeStyle<T> {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn round_angle(mut self, angle: T) -> Self {
        self.round_angle = Self::normalize_angle(angle);
        self
    }

    #[inline]
    pub(super) fn normalized(self) -> Self {
        Self {
            round_angle: Self::normalize_angle(self.round_angle),
        }
    }

    #[inline]
    fn normalize_angle(angle: T) -> T {
        let value = angle.to_f64().clamp(0.01 * PI, 0.25 * PI);
        T::from_float(value)
    }
}

impl<T: FloatNumber> Default for VariableStrokeStyle<T> {
    fn default() -> Self {
        Self {
            round_angle: T::from_float(0.1),
        }
    }
}
