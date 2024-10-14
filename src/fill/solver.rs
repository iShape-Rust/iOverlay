use crate::geom::segment::{Segment, SegmentFill};
use crate::geom::shape_count::ShapeCount;

pub(crate) trait FillStrategy {
    fn add_and_fill(this: ShapeCount, bot: ShapeCount) -> (ShapeCount, SegmentFill);
}

pub(crate) struct FillSolver;

impl FillSolver {

    #[inline]
    pub(crate) fn fill<F: FillStrategy>(is_list: bool, segments: &[Segment]) -> Vec<SegmentFill> {
        if is_list {
            Self::list_fill::<F>(segments)
        } else {
            Self::tree_fill::<F>(segments)
        }
    }
}