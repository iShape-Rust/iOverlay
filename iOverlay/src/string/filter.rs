use crate::geom::id_point::IdPoint;
use crate::string::rule::StringRule;
use crate::string::graph::StringGraph;
use crate::segm::segment::{CLIP_BOTH, SUBJ_BOTH, SUBJ_BOTTOM, SUBJ_TOP};

pub(super) struct NavigationLink {
    pub(crate) a: IdPoint,
    pub(crate) b: IdPoint,
    pub(crate) fill: u8,
}

impl NavigationLink {
    #[inline]
    pub(super) fn visit(&mut self, node_id: usize, clockwise: bool) {
        let direct = self.a.point < self.b.point;
        let direction = self.a.id == node_id;
        if direction {
            if clockwise != direct {
                self.fill &= !SUBJ_BOTTOM;
            } else {
                self.fill &= !SUBJ_TOP;
            }
        } else {
            if clockwise == direct {
                self.fill &= !SUBJ_BOTTOM;
            } else {
                self.fill &= !SUBJ_TOP;
            }
        }
    }

    #[inline]
    pub(super) fn is_move_possible(&self, node_id: usize, clockwise: bool) -> bool {
        if self.fill == SUBJ_BOTH {
            return true;
        } else if self.fill == 0 {
            return false;
        }
        let direction = self.a.id == node_id;
        let left = self.is_left();
        if direction {
            clockwise == left
        } else {
            clockwise != left
        }
    }

    #[inline]
    pub(super) fn other(&self, node_id: usize) -> IdPoint {
        if self.a.id == node_id { self.b } else { self.a }
    }

    #[inline]
    pub(super) fn is_left(&self) -> bool {
        let direct = self.a.point < self.b.point;
        if direct {
            self.fill & SUBJ_TOP == SUBJ_TOP
        } else {
            self.fill & SUBJ_BOTTOM == SUBJ_BOTTOM
        }
    }
}

impl StringGraph {
    #[inline(always)]
    pub(super) fn filter(&self, ext_rule: StringRule) -> Vec<NavigationLink> {
        match ext_rule {
            StringRule::Slice => {
                self.filter_slice()
            }
        }
    }

    #[inline]
    fn filter_slice(&self) -> Vec<NavigationLink> {
        self.links.iter().map(|link| {
            let subj = link.fill & SUBJ_BOTH;
            let fill = if subj == 0 {
                0
            } else if link.fill & CLIP_BOTH == 0 {
                subj
            } else {
                SUBJ_BOTH
            };

            NavigationLink {
                a: link.a,
                b: link.b,
                fill,
            }
        }).collect()
    }
}