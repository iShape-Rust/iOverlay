use crate::segm::segment::{SegmentFill, SUBJ_BOTTOM};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExtRule {
    Slice
}

impl ExtRule {
    #[inline(always)]
    pub(crate) fn is_hole(&self, fill: SegmentFill) -> bool {
        match self {
            ExtRule::Slice => {
                fill & SUBJ_BOTTOM == 0
            }
        }
    }
}