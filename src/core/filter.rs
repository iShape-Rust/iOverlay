use crate::{core::link::OverlayLink};
use crate::segm::segment::{SegmentFill, ALL, BOTH_BOTTOM, BOTH_TOP, CLIP_BOTH, CLIP_BOTTOM, CLIP_TOP, NONE, SUBJ_BOTH, SUBJ_BOTTOM, SUBJ_TOP};
use super::overlay_rule::OverlayRule;

/// Read how to apply filter mask [doc](https://ishape-rust.github.io/iShape-js/overlay/overlay_graph/overlay_graph.html)
pub(crate) trait MaskFilter {
    fn filter(&self, fill_rule: OverlayRule) -> Vec<bool>;
}

impl MaskFilter for Vec<OverlayLink> {
    #[inline]
    fn filter(&self, overlay_rule: OverlayRule) -> Vec<bool> {
        match overlay_rule {
            OverlayRule::Subject => filter_subject(self),
            OverlayRule::Clip => filter_clip(self),
            OverlayRule::Intersect => filter_intersect(self),
            OverlayRule::Union => filter_union(self),
            OverlayRule::Difference => filter_difference(self),
            OverlayRule::Xor => filter_xor(self),
            OverlayRule::InverseDifference => { filter_inverse_difference(self) }
        }
    }
}


pub(super) trait InclusionFilterStrategy {
    fn is_included(fill: SegmentFill) -> bool;
}

pub(super) struct SubjectFilter;
pub(super) struct ClipFilter;
pub(super) struct IntersectFilter;
pub(super) struct UnionFilter;
pub(super) struct DifferenceFilter;
pub(super) struct InverseDifferenceFilter;
pub(super) struct XorFilter;
pub(super) struct FillerFilter;
pub(super) struct NoneFilter;

impl InclusionFilterStrategy for SubjectFilter {
    #[inline(always)]
    fn is_included(fill: SegmentFill) -> bool {
        fill.is_subject()
    }
}

impl InclusionFilterStrategy for ClipFilter {
    #[inline(always)]
    fn is_included(fill: SegmentFill) -> bool {
        fill.is_clip()
    }
}

impl InclusionFilterStrategy for IntersectFilter {
    #[inline(always)]
    fn is_included(fill: SegmentFill) -> bool {
        fill.is_intersect()
    }
}

impl InclusionFilterStrategy for UnionFilter {
    #[inline(always)]
    fn is_included(fill: SegmentFill) -> bool {
        fill.is_union()
    }
}

impl InclusionFilterStrategy for DifferenceFilter {
    #[inline(always)]
    fn is_included(fill: SegmentFill) -> bool {
        fill.is_difference()
    }
}

impl InclusionFilterStrategy for InverseDifferenceFilter {
    #[inline(always)]
    fn is_included(fill: SegmentFill) -> bool {
        fill.is_inverse_difference()
    }
}

impl InclusionFilterStrategy for XorFilter {
    #[inline(always)]
    fn is_included(fill: SegmentFill) -> bool {
        fill.is_xor()
    }
}

impl InclusionFilterStrategy for FillerFilter {
    #[inline(always)]
    fn is_included(fill: SegmentFill) -> bool {
        !fill.is_filler()
    }
}

impl InclusionFilterStrategy for NoneFilter {
    #[inline(always)]
    fn is_included(_: SegmentFill) -> bool {
        true
    }
}

#[inline]
fn filter_subject(links: &[OverlayLink]) -> Vec<bool> {
    links.iter().map(|link| !link.fill.is_subject()).collect()
}

#[inline]
fn filter_clip(links: &[OverlayLink]) -> Vec<bool> {
    links.iter().map(|link| !link.fill.is_clip()).collect()
}

#[inline]
fn filter_intersect(links: &[OverlayLink]) -> Vec<bool> {
    links.iter().map(|link| !link.fill.is_intersect()).collect()
}

#[inline]
fn filter_union(links: &[OverlayLink]) -> Vec<bool> {
    links.iter().map(|link| !link.fill.is_union()).collect()
}

#[inline]
fn filter_difference(links: &[OverlayLink]) -> Vec<bool> {
    links.iter().map(|link| !link.fill.is_difference()).collect()
}

#[inline]
fn filter_inverse_difference(links: &[OverlayLink]) -> Vec<bool> {
    links.iter().map(|link| !link.fill.is_inverse_difference()).collect()
}

#[inline]
fn filter_xor(links: &[OverlayLink]) -> Vec<bool> {
    links.iter().map(|link| !link.fill.is_xor()).collect()
}

trait FillFilter {
    fn is_subject(&self) -> bool;
    fn is_clip(&self) -> bool;
    fn is_intersect(&self) -> bool;
    fn is_union(&self) -> bool;
    fn is_difference(&self) -> bool;
    fn is_inverse_difference(&self) -> bool;
    fn is_xor(&self) -> bool;
    fn is_filler(&self) -> bool;
}

impl FillFilter for SegmentFill {
    #[inline(always)]
    fn is_subject(&self) -> bool {
        let fill = *self;
        let subj = fill & SUBJ_BOTH;
        subj == SUBJ_TOP || subj == SUBJ_BOTTOM
    }

    #[inline(always)]
    fn is_clip(&self) -> bool {
        let fill = *self;
        let clip = fill & CLIP_BOTH;
        clip == CLIP_TOP || clip == CLIP_BOTTOM
    }

    #[inline(always)]
    fn is_intersect(&self) -> bool {
        let fill = *self;
        let top = fill & BOTH_TOP;
        let bottom = fill & BOTH_BOTTOM;

        (top == BOTH_TOP || bottom == BOTH_BOTTOM) && fill != ALL
    }

    #[inline(always)]
    fn is_union(&self) -> bool {
        let fill = *self;
        let top = fill & BOTH_TOP;
        let bottom = fill & BOTH_BOTTOM;
        (top == 0 || bottom == 0) && fill != 0
    }

    #[inline(always)]
    fn is_difference(&self) -> bool {
        let fill = *self;
        let top = fill & BOTH_TOP;
        let bottom = fill & BOTH_BOTTOM;
        let is_not_inner_subj = fill != SUBJ_BOTH;
        (top == SUBJ_TOP || bottom == SUBJ_BOTTOM) && is_not_inner_subj
    }

    #[inline(always)]
    fn is_inverse_difference(&self) -> bool {
        let fill = *self;
        let top = fill & BOTH_TOP;
        let bottom = fill & BOTH_BOTTOM;
        let is_not_inner_clip = fill != CLIP_BOTH;
        (top == CLIP_TOP || bottom == CLIP_BOTTOM) && is_not_inner_clip
    }

    #[inline(always)]
    fn is_xor(&self) -> bool {
        let fill = *self;
        let top = fill & BOTH_TOP;
        let bottom = fill & BOTH_BOTTOM;

        let is_any_top = top == SUBJ_TOP || top == CLIP_TOP;
        let is_any_bottom = bottom == SUBJ_BOTTOM || bottom == CLIP_BOTTOM;

        // only one of it must be true
        is_any_top != is_any_bottom
    }

    #[inline(always)]
    fn is_filler(&self) -> bool {
        let fill = *self;
        fill == NONE || fill == SUBJ_BOTH || fill == CLIP_BOTH || fill == ALL
    }
}