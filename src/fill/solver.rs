use crate::core::fill_rule::FillRule;
use crate::segm::shape_count::ShapeCount;
use crate::segm::segment::{Segment, SegmentFill};

pub(crate) struct FillSolver {}

impl FillSolver {
    pub(crate) fn fill(fill_rule: FillRule, is_list: bool, segments: &Vec<Segment>) -> Vec<SegmentFill> {
        if is_list {
            Self::list_fill(&segments, fill_rule)
        } else {
            Self::tree_fill(&segments, fill_rule)
        }
    }
}

impl Segment {
    #[inline]
    pub(super) fn add_and_fill(&self, sum_count: ShapeCount, fill_rule: FillRule) -> (ShapeCount, SegmentFill) {
        let new_count = sum_count.add(self.count);

        let fill: SegmentFill = match fill_rule {
            FillRule::EvenOdd => {
                (1 & new_count.subj as SegmentFill) // SUBJ_TOP
                    | ((1 & sum_count.subj as SegmentFill) << 1) // SUBJ_BOTTOM
                    | ((1 & new_count.clip as SegmentFill) << 2) // CLIP_TOP
                    | ((1 & sum_count.clip as SegmentFill) << 3) // CLIP_BOTTOM
            }
            FillRule::NonZero => {
                (new_count.subj != 0) as SegmentFill // SUBJ_TOP
                    | ((sum_count.subj != 0) as SegmentFill) << 1 // SUBJ_BOTTOM
                    | ((new_count.clip != 0) as SegmentFill) << 2 // CLIP_TOP
                    | ((sum_count.clip != 0) as SegmentFill) << 3 // CLIP_BOTTOM
            }
        };

        (new_count, fill)
    }
}