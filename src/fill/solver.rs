use i_float::point::IntPoint;
use crate::core::fill_rule::FillRule;
use crate::segm::shape_count::ShapeCount;
use crate::segm::segment::{Segment, CLIP_BOTTOM, CLIP_TOP, NONE, SUBJ_BOTTOM, SUBJ_TOP, SegmentFill};

pub(super) struct Handler {
    pub(super) id: usize,
    pub(super) b: IntPoint,
}

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
        let is_subj_top: bool;
        let is_subj_bottom: bool;
        let is_clip_top: bool;
        let is_clip_bottom: bool;

        let new_count = sum_count.add(self.count);

        match fill_rule {
            FillRule::EvenOdd => {
                is_subj_top = 1 & new_count.subj == 1;
                is_subj_bottom = 1 & sum_count.subj == 1;

                is_clip_top = 1 & new_count.clip == 1;
                is_clip_bottom = 1 & sum_count.clip == 1;
            }
            FillRule::NonZero => {
                is_subj_top = new_count.subj != 0;
                is_subj_bottom = sum_count.subj != 0;

                is_clip_top = new_count.clip != 0;
                is_clip_bottom = sum_count.clip != 0;
            }
        }

        let subj_top = if is_subj_top { SUBJ_TOP } else { NONE };
        let subj_bottom = if is_subj_bottom { SUBJ_BOTTOM } else { NONE };
        let clip_top = if is_clip_top { CLIP_TOP } else { NONE };
        let clip_bottom = if is_clip_bottom { CLIP_BOTTOM } else { NONE };

        let fill = subj_top | subj_bottom | clip_top | clip_bottom;

        (new_count, fill)
    }
}