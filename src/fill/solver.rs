use crate::segm::segment::{Segment, SegmentFill};
use crate::segm::shape_count::ShapeCount;

pub(crate) trait FillStrategy<C> {
    fn add_and_fill(this: C, bot: C) -> (C, SegmentFill);
}

pub(crate) struct FillSolver;

impl FillSolver {

    #[inline]
    pub(crate) fn fill<F: FillStrategy<C>, C: ShapeCount>(is_list: bool, segments: &[Segment<C>]) -> Vec<SegmentFill> {
        if is_list {
            Self::list_fill::<F, C>(segments)
        } else {
            Self::tree_fill::<F, C>(segments)
        }
    }
}