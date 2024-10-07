use crate::core::fill_rule::FillRule;
use crate::fill::strategy::{EvenOddStrategy, EvenOddStrategyString, FillStrategy, NegativeStrategy, NegativeStrategyString, NonZeroStrategy, NonZeroStrategyString, PositiveStrategy, PositiveStrategyString};
use crate::segm::segment::{Segment, SegmentFill};

pub(crate) struct FillSolver {}

impl FillSolver {

    #[inline]
    pub(crate) fn fill_with_rule(fill_rule: FillRule, is_string: bool, is_list: bool, segments: &[Segment]) -> Vec<SegmentFill> {
        if is_string {
            match fill_rule {
                FillRule::EvenOdd => Self::fill::<EvenOddStrategyString>(is_list, segments),
                FillRule::NonZero => Self::fill::<NonZeroStrategyString>(is_list, segments),
                FillRule::Positive => Self::fill::<PositiveStrategyString>(is_list, segments),
                FillRule::Negative => Self::fill::<NegativeStrategyString>(is_list, segments),
            }
        } else {
            match fill_rule {
                FillRule::EvenOdd => Self::fill::<EvenOddStrategy>(is_list, segments),
                FillRule::NonZero => Self::fill::<NonZeroStrategy>(is_list, segments),
                FillRule::Positive => Self::fill::<PositiveStrategy>(is_list, segments),
                FillRule::Negative => Self::fill::<NegativeStrategy>(is_list, segments),
            }
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