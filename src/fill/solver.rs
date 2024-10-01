use crate::core::fill_rule::FillRule;
use crate::fill::strategy::{EvenOddStrategy, FillStrategy, NegativeStrategy, NonZeroStrategy, PositiveStrategy};
use crate::segm::segment::{Segment, SegmentFill};

pub(crate) struct FillSolver {}

impl FillSolver {

    #[inline]
    pub(crate) fn fill_with_rule(fill_rule: FillRule, is_list: bool, segments: &[Segment]) -> Vec<SegmentFill> {
        match fill_rule {
            FillRule::EvenOdd => Self::fill::<EvenOddStrategy>(is_list, segments),
            FillRule::NonZero => Self::fill::<NonZeroStrategy>(is_list, segments),
            FillRule::Positive => Self::fill::<PositiveStrategy>(is_list, segments),
            FillRule::Negative => Self::fill::<NegativeStrategy>(is_list, segments),
        }
    }

    #[inline]
    pub(crate) fn fill<F: FillStrategy>(is_list: bool, segments: &[Segment]) -> Vec<SegmentFill> {
        if is_list {
            Self::list_fill::<F>(segments)
        } else {
            Self::tree_fill::<F>(segments)
        }
    }
}