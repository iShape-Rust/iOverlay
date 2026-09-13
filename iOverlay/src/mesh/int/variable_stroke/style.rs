use crate::mesh::int::arc::ArcOptions;
use i_float::int::{
    number::{int::IntNumber, wide_int::WideIntNumber},
    point::IntPoint,
};

#[derive(Debug, Clone, Copy)]
pub struct IntStrokeVertex<I: IntNumber = i32> {
    pub point: IntPoint<I>,
    pub width: I,
}
impl<I: IntNumber> IntStrokeVertex<I> {
    pub fn new(point: IntPoint<I>, width: I) -> Self {
        Self { point, width }
    }
    pub(super) fn radius(&self) -> I {
        I::from_wide((self.width.max(I::ZERO).to_wide() + I::Wide::ONE) / I::Wide::TWO)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct IntVariableStrokeStyle {
    pub arc: ArcOptions,
}
impl Default for IntVariableStrokeStyle {
    fn default() -> Self {
        Self {
            arc: ArcOptions {
                max_step: i_float::int::angle::Angle::from_bits(68_356_528),
                ..ArcOptions::default()
            },
        }
    }
}
impl IntVariableStrokeStyle {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn arc(mut self, arc: ArcOptions) -> Self {
        self.arc = arc;
        self
    }
}
