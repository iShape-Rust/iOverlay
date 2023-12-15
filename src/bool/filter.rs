use crate::{layout::overlay_link::OverlayLink, fill::segment::SegmentFill};
use super::overlay_rule::OverlayRule;

pub(super) trait Filter {
    fn filter(&self, fill_rule: OverlayRule) -> Vec<bool>;
}

impl Filter for Vec<OverlayLink> {

    fn filter(&self, fill_rule: OverlayRule) -> Vec<bool> {
        match fill_rule {
            OverlayRule::Subject => filter_subject(self),
            OverlayRule::Clip => filter_clip(self),
            OverlayRule::Intersect => filter_intersect(self),
            OverlayRule::Union => filter_union(self),
            OverlayRule::Difference => filter_difference(self),
            OverlayRule::Xor => filter_xor(self),
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

        // One side must belong to both but not two side at once
        
        let is_top = fill & SegmentFill::BOTH_TOP == SegmentFill::BOTH_TOP;
        let is_bot = fill & SegmentFill::BOTH_BOTTOM == SegmentFill::BOTH_BOTTOM;

        skip[i] = !(is_top || is_bot) || is_top && is_bot;
    }

    skip
}

fn filter_union(links: &Vec<OverlayLink>) -> Vec<bool> {
    let n = links.len();
    let mut skip = vec![false; n];

    for i in 0..n {
        let fill = links[i].fill;

        // One side must be empty

        let is_top_empty = fill & SegmentFill::BOTH_TOP == SegmentFill::NONE;
        let is_bot_empty = fill & SegmentFill::BOTH_BOTTOM == SegmentFill::NONE;

        skip[i] = !(is_top_empty || is_bot_empty);
    }

    skip
}

fn filter_difference(links: &Vec<OverlayLink>) -> Vec<bool> {
    let n = links.len();
    let mut skip = vec![false; n];

    for i in 0..n {
        let fill = links[i].fill;

        // One side must belong only subject
        // Can not be subject inner edge

        let subject_inner = fill == SegmentFill::SUBJECT_BOTH;
        let top_only_subject = fill & SegmentFill::BOTH_TOP == SegmentFill::SUBJECT_TOP;
        let bot_only_subject = fill & SegmentFill::BOTH_BOTTOM == SegmentFill::SUBJECT_BOTTOM;

        skip[i] = !(top_only_subject || bot_only_subject) || subject_inner;
    }

    skip
}

fn filter_xor(links: &Vec<OverlayLink>) -> Vec<bool> {
    let n = links.len();
    let mut skip = vec![false; n];

    for i in 0..n {
        let fill = links[i].fill;

        // Skip edge if clip and subject share it

        let subject_inner = fill == SegmentFill::SUBJECT_BOTH;
        let clip_inner = fill == SegmentFill::CLIP_BOTH;
        let both_inner = fill == SegmentFill::ALL;
        let only_top = fill == SegmentFill::BOTH_TOP;
        let only_bottom = fill == SegmentFill::BOTH_BOTTOM;
        let diagonal_0 = fill == SegmentFill::CLIP_TOP | SegmentFill::SUBJECT_BOTTOM;
        let diagonal_1 = fill == SegmentFill::CLIP_BOTTOM | SegmentFill::SUBJECT_TOP;

        skip[i] = subject_inner || clip_inner || both_inner || only_top || only_bottom || diagonal_0 || diagonal_1;
    }

    skip
}