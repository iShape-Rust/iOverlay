use crate::{layout::overlay_link::OverlayLink, fill::segment::SegmentFill};
use super::fill_rule::FillRule;

pub(super) trait Filter {
    fn filter(&self, fill_rule: FillRule) -> Vec<bool>;
}

impl Filter for Vec<OverlayLink> {

    fn filter(&self, fill_rule: FillRule) -> Vec<bool> {
        match fill_rule {
            FillRule::Subject => filter_subject(self),
            FillRule::Clip => filter_clip(self),
            FillRule::Intersect => filter_intersect(self),
            FillRule::Union => filter_union(self),
            FillRule::Difference => filter_difference(self),
            FillRule::Xor => filter_xor(self),
        }
    }
}

fn filter_subject(links: &Vec<OverlayLink>) -> Vec<bool> {
    let n = links.len();
    let mut skip = vec![false; n];

    for i in 0..n {
        let fill = links[i].fill;

        // Skip edge if it it inside or not belong subject

        let is_top = fill & SegmentFill::SUBJECT_TOP == SegmentFill::SUBJECT_TOP;
        let is_bot = fill & SegmentFill::SUBJECT_BOTTOM == SegmentFill::SUBJECT_BOTTOM;

        skip[i] = is_top == is_bot;
    }

    skip
}

fn filter_clip(links: &Vec<OverlayLink>) -> Vec<bool> {
    let n = links.len();
    let mut skip = vec![false; n];

    for i in 0..n {
        let fill = links[i].fill;

        // Skip edge if it it inside or not belong clip

        let is_top = fill & SegmentFill::CLIP_TOP == SegmentFill::CLIP_TOP;
        let is_bot = fill & SegmentFill::CLIP_BOTTOM == SegmentFill::CLIP_BOTTOM;

        skip[i] = is_top == is_bot;
    }

    skip
}

fn filter_intersect(links: &Vec<OverlayLink>) -> Vec<bool> {
    let n = links.len();
    let mut skip = vec![false; n];

    for i in 0..n {
        let fill = links[i].fill;

        // Skip edge if it not from same side. If edge is inside for one polygon is ok too
        
        let is_top_subject = fill & SegmentFill::SUBJECT_TOP == SegmentFill::SUBJECT_TOP;
        let is_top_clip = fill & SegmentFill::CLIP_TOP == SegmentFill::CLIP_TOP;

        let is_bottom_subject = fill & SegmentFill::SUBJECT_BOTTOM == SegmentFill::SUBJECT_BOTTOM;
        let is_bottom_clip = fill & SegmentFill::CLIP_BOTTOM == SegmentFill::CLIP_BOTTOM;
        
        let skip_edge = !(is_top_subject && is_top_clip || is_bottom_subject && is_bottom_clip);
        
        skip[i] = skip_edge;
    }

    skip
}

fn filter_union(links: &Vec<OverlayLink>) -> Vec<bool> {
    let n = links.len();
    let mut skip = vec![false; n];

    for i in 0..n {
        let fill = links[i].fill;

        // Skip edge if it has a polygon from both sides (subject or clip). One side must be empty

        let is_top_not_empty = fill & SegmentFill::BOTH_TOP != SegmentFill::NONE;
        let is_bot_not_empty = fill & SegmentFill::BOTH_BOTTOM != SegmentFill::NONE;

        skip[i] = is_top_not_empty && is_bot_not_empty;
    }

    skip
}

fn filter_difference(links: &Vec<OverlayLink>) -> Vec<bool> {
    let n = links.len();
    let mut skip = vec![false; n];

    for i in 0..n {
        let fill = links[i].fill;

        // Skip edge if it does not have only the subject side

        let top_only_subject = fill & SegmentFill::BOTH_TOP == SegmentFill::SUBJECT_TOP;
        let bot_only_subject = fill & SegmentFill::BOTH_BOTTOM == SegmentFill::SUBJECT_BOTTOM;

        skip[i] = !(top_only_subject || bot_only_subject);
    }

    skip
}

fn filter_xor(links: &Vec<OverlayLink>) -> Vec<bool> {
    let n = links.len();
    let mut skip = vec![false; n];

    for i in 0..n {
        let fill = links[i].fill;

        // Skip edge if clip and subject share it

        let same_top = fill == SegmentFill::BOTH_TOP;
        let same_bottom = fill == SegmentFill::BOTH_BOTTOM;
        let same_side0 = fill == SegmentFill::SUBJECT_TOP | SegmentFill::CLIP_BOTTOM;
        let same_side1 = fill == SegmentFill::SUBJECT_BOTTOM | SegmentFill::CLIP_TOP;

        skip[i] = same_top || same_bottom || same_side0 || same_side1;
    }

    skip
}