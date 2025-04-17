use crate::core::link::OverlayLink;
use crate::segm::segment::{SUBJ_BOTH, SUBJ_BOTTOM, SUBJ_TOP};
use crate::string::graph::StringGraph;
use crate::string::rule::StringRule;

impl OverlayLink {
    #[inline]
    pub(super) fn visit_fill(&self, fill: u8, node_id: usize, clockwise: bool) -> u8 {
        let is_a = self.a.id == node_id;
        let direct = self.a.point < self.b.point;
        let same = clockwise == direct;

        let mask = if is_a {
            if same { SUBJ_TOP } else { SUBJ_BOTTOM }
        } else {
            if same { SUBJ_BOTTOM } else { SUBJ_TOP }
        };

        fill & !mask
    }

    #[inline]
    pub(super) fn is_move_possible(&self, fill: u8, node_id: usize, clockwise: bool) -> bool {
        match fill {
            SUBJ_BOTH => return true,
            0 => return false,
            _ => {}
        }

        let is_a = self.a.id == node_id;
        let direct = self.a.point < self.b.point;
        let left = if direct {
            fill & SUBJ_TOP != 0
        } else {
            fill & SUBJ_BOTTOM != 0
        };

        is_a == (clockwise == left)
    }
}
//
// impl NavigationLink {
//
//     #[inline]
//     pub(super) fn visit_link(&mut self, node_id: usize,  link: &OverlayLink, clockwise: bool) {
//         let is_a = link.a.id == node_id;
//         let direct = link.a.point < link.b.point;
//         let same = clockwise == direct;
//
//         let mask = if is_a {
//             if same { SUBJ_TOP } else { SUBJ_BOTTOM }
//         } else {
//             if same { SUBJ_BOTTOM } else { SUBJ_TOP }
//         };
//
//         self.fill &= !mask;
//     }
//
//     #[inline]
//     pub(super) fn visit(&mut self, node_id: usize, clockwise: bool) {
//         let is_a = self.a.id == node_id;
//         let direct = self.a.point < self.b.point;
//         let same = clockwise == direct;
//
//         let mask = if is_a {
//             if same { SUBJ_TOP } else { SUBJ_BOTTOM }
//         } else {
//             if same { SUBJ_BOTTOM } else { SUBJ_TOP }
//         };
//
//         self.fill &= !mask;
//     }
//
//     #[inline]
//     pub(super) fn is_move_possible(&self, node_id: usize, clockwise: bool) -> bool {
//         match self.fill {
//             SUBJ_BOTH => return true,
//             0 => return false,
//             _ => {}
//         }
//
//         let is_a = self.a.id == node_id;
//         let direct = self.a.point < self.b.point;
//         let left = if direct {
//             self.fill & SUBJ_TOP != 0
//         } else {
//             self.fill & SUBJ_BOTTOM != 0
//         };
//
//         is_a == (clockwise == left)
//     }
//
//     #[inline]
//     pub(super) fn other(&self, node_id: usize) -> IdPoint {
//         if self.a.id == node_id { self.b } else { self.a }
//     }
// }

impl StringGraph {
    #[inline(always)]
    pub(super) fn filter(&self, ext_rule: StringRule) -> Vec<u8> {
        match ext_rule {
            StringRule::Slice => self.filter_slice(),
        }
    }

    #[inline]
    fn filter_slice(&self) -> Vec<u8> {
        self.links
            .iter()
            .map(|link| link.fill & SUBJ_BOTH)
            .collect()
    }
}
