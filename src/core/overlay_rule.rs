use crate::segm::segment::{BOTH_BOTTOM, BOTH_TOP, CLIP_TOP, NONE, SegmentFill, SUBJ_TOP};

/// Defines the types of overlay/boolean operations that can be applied to shapes. For a visual description, see [Overlay Rules](https://ishape-rust.github.io/iShape-js/overlay/overlay_rules/overlay_rules.html).
/// - `Subject`: Processes the subject shape, useful for resolving self-intersections and degenerate cases within the subject itself.
/// - `Clip`: Similar to `Subject`, but for Clip shapes.
/// - `Intersect`: Finds the common area between the subject and clip shapes, effectively identifying where they overlap.
/// - `Union`: Combines the area of both subject and clip shapes into a single unified shape.
/// - `Difference`: Subtracts the area of the clip shape from the subject shape, removing the clip shape's area from the subject.
/// - `InverseDifference`: Subtracts the area of the subject shape from the clip shape, removing the subject shape's area from the clip.
/// - `Xor`: Produces a shape consisting of areas unique to each shape, excluding any parts where the subject and clip overlap.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OverlayRule {
    Subject,
    Clip,
    Intersect,
    Union,
    Difference,
    InverseDifference,
    Xor,
}

pub(crate) trait FillTopStrategy {
    fn is_fill_top(fill: SegmentFill) -> bool;
}

pub(crate) struct SubjectStrategy;
pub(crate) struct ClipStrategy;
pub(crate) struct IntersectStrategy;
pub(crate) struct UnionStrategy;
pub(crate) struct DifferenceStrategy;
pub(crate) struct InverseDifferenceStrategy;
pub(crate) struct XorStrategy;

impl FillTopStrategy for SubjectStrategy {
    #[inline(always)]
    fn is_fill_top(fill: SegmentFill) -> bool {
        fill & SUBJ_TOP == SUBJ_TOP
    }
}

impl FillTopStrategy for ClipStrategy {
    #[inline(always)]
    fn is_fill_top(fill: SegmentFill) -> bool {
        fill & CLIP_TOP == CLIP_TOP
    }
}

impl FillTopStrategy for IntersectStrategy {
    #[inline(always)]
    fn is_fill_top(fill: SegmentFill) -> bool {
        fill & BOTH_TOP == BOTH_TOP
    }
}

impl FillTopStrategy for UnionStrategy {
    #[inline(always)]
    fn is_fill_top(fill: SegmentFill) -> bool {
        fill & BOTH_BOTTOM == NONE
    }
}

impl FillTopStrategy for DifferenceStrategy {
    #[inline(always)]
    fn is_fill_top(fill: SegmentFill) -> bool {
        fill & BOTH_TOP == SUBJ_TOP
    }
}

impl FillTopStrategy for InverseDifferenceStrategy {
    #[inline(always)]
    fn is_fill_top(fill: SegmentFill) -> bool {
        fill & BOTH_TOP == CLIP_TOP
    }
}

impl FillTopStrategy for XorStrategy {
    #[inline(always)]
    fn is_fill_top(fill: SegmentFill) -> bool {
        let is_subject = fill & BOTH_TOP == SUBJ_TOP;
        let is_clip = fill & BOTH_TOP == CLIP_TOP;
        is_subject || is_clip
    }
}