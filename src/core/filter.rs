use crate::{core::overlay_link::OverlayLink};
use crate::fill::segment::{ALL, BOTH_BOTTOM, BOTH_TOP, CLIP_BOTH, CLIP_BOTTOM, CLIP_TOP, NONE, SUBJ_BOTH, SUBJ_BOTTOM, SUBJ_TOP};
use super::overlay_rule::OverlayRule;

pub(crate) trait Filter {
    fn filter(&self, fill_rule: OverlayRule) -> Vec<bool>;
}

impl Filter for Vec<OverlayLink> {
    #[inline(always)]
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

fn filter_subject(links: &Vec<OverlayLink>) -> Vec<bool> {
    links.iter().map(|link| {
        let fill = link.fill;

        // Skip edge if it inside or not belong subject

        let subj = fill & SUBJ_BOTH;
        subj == 0 || subj == SUBJ_BOTH
    }).collect()
}

fn filter_clip(links: &Vec<OverlayLink>) -> Vec<bool> {
    links.iter().map(|link| {
        let fill = link.fill;

        // Skip edge if it inside or not belong clip

        let clip = fill & CLIP_BOTH;
        clip == 0 || clip == CLIP_BOTH
    }).collect()
}

fn filter_intersect(links: &Vec<OverlayLink>) -> Vec<bool> {
    links.iter().map(|link| {
        let fill = link.fill;

        // One side must belong to both but not two side at once

        let is_top = fill & BOTH_TOP == BOTH_TOP;
        let is_bot = fill & BOTH_BOTTOM == BOTH_BOTTOM;

        !(is_top || is_bot) || is_top && is_bot
    }).collect()
}

fn filter_union(links: &Vec<OverlayLink>) -> Vec<bool> {
    links.iter().map(|link| {
        let fill = link.fill;

        // One side must be empty

        let is_top_empty = fill & BOTH_TOP == NONE;
        let is_bot_empty = fill & BOTH_BOTTOM == NONE;

        !(is_top_empty || is_bot_empty)
    }).collect()
}

fn filter_difference(links: &Vec<OverlayLink>) -> Vec<bool> {
    links.iter().map(|link| {
        let fill = link.fill;

        // One side must belong only subject
        // Can not be subject inner edge

        let subject_inner = fill == SUBJ_BOTH;
        let top_only_subject = fill & BOTH_TOP == SUBJ_TOP;
        let bot_only_subject = fill & BOTH_BOTTOM == SUBJ_BOTTOM;

        !(top_only_subject || bot_only_subject) || subject_inner
    }).collect()
}

fn filter_inverse_difference(links: &Vec<OverlayLink>) -> Vec<bool> {
    links.iter().map(|link| {
        let fill = link.fill;

        // One side must belong only clip
        // Can not be clip inner edge

        let clip_inner = fill == CLIP_BOTH;
        let top_only_clip = fill & BOTH_TOP == CLIP_TOP;
        let bot_only_clip = fill & BOTH_BOTTOM == CLIP_BOTTOM;

        !(top_only_clip || bot_only_clip) || clip_inner
    }).collect()
}

fn filter_xor(links: &Vec<OverlayLink>) -> Vec<bool> {
    links.iter().map(|link| {
        let fill = link.fill;

        // Skip edge if clip and subject share it

        let subject_inner = fill == SUBJ_BOTH;
        let clip_inner = fill == CLIP_BOTH;
        let both_inner = fill == ALL;
        let only_top = fill == BOTH_TOP;
        let only_bottom = fill == BOTH_BOTTOM;
        let diagonal_0 = fill == CLIP_TOP | SUBJ_BOTTOM;
        let diagonal_1 = fill == CLIP_BOTTOM | SUBJ_TOP;

        subject_inner || clip_inner || both_inner || only_top || only_bottom || diagonal_0 || diagonal_1
    }).collect()
}