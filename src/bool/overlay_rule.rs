use crate::fill::segment::{BOTH_BOTTOM, BOTH_TOP, CLIP_TOP, NONE, SegmentFill, SUBJ_TOP};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OverlayRule {
    Subject,
    Clip,
    Intersect,
    Union,
    Difference,
    Xor
}

impl OverlayRule {
    pub(crate) fn is_fill_top(&self, fill: SegmentFill) -> bool {
        match self {
            OverlayRule::Subject => fill & SUBJ_TOP == SUBJ_TOP,
            OverlayRule::Clip => fill & CLIP_TOP == CLIP_TOP,
            OverlayRule::Intersect => fill & BOTH_TOP == BOTH_TOP,
            OverlayRule::Union => fill & BOTH_BOTTOM == NONE,
            OverlayRule::Difference => fill & BOTH_TOP == SUBJ_TOP,
            OverlayRule::Xor => {
                let is_subject = fill & BOTH_TOP == SUBJ_TOP;
                let is_clip = fill & BOTH_TOP == CLIP_TOP;
                is_subject || is_clip
            }
        }
    }
}