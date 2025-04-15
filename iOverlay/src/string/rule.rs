use crate::segm::segment::{SegmentFill, SUBJ_BOTTOM};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StringRule {
    Slice
}

impl StringRule {
    #[inline(always)]
    pub(crate) fn is_hole(&self, fill: SegmentFill) -> bool {
        match self {
            StringRule::Slice => {
                fill & SUBJ_BOTTOM == 0
            }
        }
    }
}