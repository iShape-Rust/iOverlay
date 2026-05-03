use crate::segm::boolean::ShapeCountBoolean;
use i_float::int::point::IntPoint;

pub trait OverlayEdgeData<C = ShapeCountBoolean>: Copy + PartialEq + Send + Sync {
    #[inline(always)]
    fn reversed(self) -> Self {
        self
    }

    #[inline(always)]
    fn split(self, _ctx: EdgeDataSplit) -> (Self, Self) {
        (self, self)
    }

    fn merge(ctx: EdgeDataMerge<C, Self>) -> Self;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EdgeDataSplit {
    pub a: IntPoint,
    pub p: IntPoint,
    pub b: IntPoint,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EdgeDataMerge<C, D> {
    pub lhs_data: D,
    pub lhs_count: C,
    pub rhs_data: D,
    pub rhs_count: C,
    pub out_count: C,
}

impl<C> OverlayEdgeData<C> for ()
where
    C: Copy + Send + Sync,
{
    #[inline(always)]
    fn merge(_: EdgeDataMerge<C, Self>) -> Self {}
}
