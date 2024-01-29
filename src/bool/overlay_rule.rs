use crate::fill::segment::{BOTH_BOTTOM, BOTH_TOP, CLIP_BOTTOM, CLIP_TOP, NONE, SegmentFill, SUBJECT_BOTTOM, SUBJECT_TOP};

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
    pub(super) fn is_fill_top(&self, fill: SegmentFill) -> bool {
        match self {
            OverlayRule::Subject => fill & SUBJECT_TOP == SUBJECT_TOP,
            OverlayRule::Clip => fill & CLIP_TOP == CLIP_TOP,
            OverlayRule::Intersect => fill & BOTH_TOP == BOTH_TOP,
            OverlayRule::Union => fill & BOTH_BOTTOM == NONE,
            OverlayRule::Difference => fill & BOTH_TOP == SUBJECT_TOP,
            OverlayRule::Xor => {
                let is_subject = fill & BOTH_TOP == SUBJECT_TOP;
                let is_clip = fill & BOTH_TOP == CLIP_TOP;
                is_subject || is_clip
            }
        }
    }
}