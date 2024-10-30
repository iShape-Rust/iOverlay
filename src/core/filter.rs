use crate::{core::link::OverlayLink};
use crate::segm::segment::{Segment, SegmentFill, ALL, BOTH_BOTTOM, BOTH_TOP, CLIP_BOTH, CLIP_BOTTOM, CLIP_TOP, NONE, SUBJ_BOTH, SUBJ_BOTTOM, SUBJ_TOP};
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

pub(super) struct SegmentFilter;

impl SegmentFilter {
    #[inline]
    pub(super) fn filter(segments: &mut Vec<Segment>, fills: &mut Vec<SegmentFill>, overlay_rule: OverlayRule) {
        match overlay_rule {
            OverlayRule::Subject => Self::filter_subject(segments, fills),
            OverlayRule::Clip => Self::filter_clip(segments, fills),
            OverlayRule::Intersect => Self::filter_intersect(segments, fills),
            OverlayRule::Union => Self::filter_union(segments, fills),
            OverlayRule::Difference => Self::filter_difference(segments, fills),
            OverlayRule::InverseDifference => Self::filter_inverse_difference(segments, fills),
            OverlayRule::Xor => Self::filter_xor(segments, fills),
        }
    }

    #[inline]
    fn filter_subject(segments: &mut Vec<Segment>, fills: &mut Vec<SegmentFill>) {
        let mut iter = fills.iter();
        segments.retain(|_| iter.next().unwrap().is_subject());
        fills.retain(|fill| fill.is_subject())
    }

    #[inline]
    fn filter_clip(segments: &mut Vec<Segment>, fills: &mut Vec<SegmentFill>) {
        let mut iter = fills.iter();
        segments.retain(|_| iter.next().unwrap().is_clip());
        fills.retain(|fill| fill.is_clip())
    }

    #[inline]
    fn filter_intersect(segments: &mut Vec<Segment>, fills: &mut Vec<SegmentFill>) {
        let mut iter = fills.iter();
        segments.retain(|_| iter.next().unwrap().is_intersect());
        fills.retain(|fill| fill.is_intersect())
    }

    #[inline]
    fn filter_union(segments: &mut Vec<Segment>, fills: &mut Vec<SegmentFill>) {
        let mut iter = fills.iter();
        segments.retain(|_| iter.next().unwrap().is_union());
        fills.retain(|fill| fill.is_union())
    }

    #[inline]
    fn filter_difference(segments: &mut Vec<Segment>, fills: &mut Vec<SegmentFill>) {
        let mut iter = fills.iter();
        segments.retain(|_| iter.next().unwrap().is_difference());
        fills.retain(|fill| fill.is_difference())
    }

    #[inline]
    fn filter_inverse_difference(segments: &mut Vec<Segment>, fills: &mut Vec<SegmentFill>) {
        let mut iter = fills.iter();
        segments.retain(|_| iter.next().unwrap().is_inverse_difference());
        fills.retain(|fill| fill.is_inverse_difference())
    }

    #[inline]
    fn filter_xor(segments: &mut Vec<Segment>, fills: &mut Vec<SegmentFill>) {
        let mut iter = fills.iter();
        segments.retain(|_| iter.next().unwrap().is_xor());
        fills.retain(|fill| fill.is_xor())
    }

    #[inline]
    pub(super) fn filter_filler(segments: &mut Vec<Segment>, fills: &mut Vec<SegmentFill>) {
        let mut iter = fills.iter();
        segments.retain(|_| !iter.next().unwrap().is_filler());
        fills.retain(|fill| !fill.is_filler())
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

    fn is_filler(&self) -> bool {
        let fill = *self;
        fill == NONE || fill == SUBJ_BOTH || fill == CLIP_BOTH || fill == ALL
    }
}