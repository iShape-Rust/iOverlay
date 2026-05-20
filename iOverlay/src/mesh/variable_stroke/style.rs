use crate::mesh::style::{LineCap, LineJoin};
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;

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
        P::Scalar::from_float(0.5 * self.width.to_f64().max(0.0))
    }
}

#[derive(Debug, Clone)]
pub struct VariableStrokeStyle<P: FloatPointCompatible> {
    pub start_cap: LineCap<P>,
    pub end_cap: LineCap<P>,
    pub join: LineJoin<P::Scalar>,
}

impl<P: FloatPointCompatible> VariableStrokeStyle<P> {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn start_cap(mut self, cap: LineCap<P>) -> Self {
        self.start_cap = cap.normalize();
        self
    }

    #[inline]
    pub fn end_cap(mut self, cap: LineCap<P>) -> Self {
        self.end_cap = cap.normalize();
        self
    }

    #[inline]
    pub fn line_join(mut self, join: LineJoin<P::Scalar>) -> Self {
        self.join = join.normalize();
        self
    }
}

impl<P: FloatPointCompatible> Default for VariableStrokeStyle<P> {
    fn default() -> Self {
        Self {
            start_cap: LineCap::Butt,
            end_cap: LineCap::Butt,
            join: LineJoin::Bevel,
        }
    }
}
