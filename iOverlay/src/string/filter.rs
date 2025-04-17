use crate::geom::id_point::IdPoint;
use crate::segm::segment::{SUBJ_BOTH, SUBJ_BOTTOM, SUBJ_TOP};
use crate::string::graph::StringGraph;
use crate::string::rule::StringRule;

pub(super) struct NavigationLink {
    pub(crate) a: IdPoint,
    pub(crate) b: IdPoint,
    pub(crate) fill: u8,
}

impl NavigationLink {
    #[inline]
    pub(super) fn visit(&mut self, node_id: usize, clockwise: bool) {
        let is_a = self.a.id == node_id;
        let direct = self.a.point < self.b.point;
        let same = clockwise == direct;

        let mask = if is_a {
            if same { SUBJ_TOP } else { SUBJ_BOTTOM }
        } else {
            if same { SUBJ_BOTTOM } else { SUBJ_TOP }
        };

        self.fill &= !mask;
    }

    #[inline]
    pub(super) fn is_move_possible(&self, node_id: usize, clockwise: bool) -> bool {
        match self.fill {
            SUBJ_BOTH => return true,
            0 => return false,
            _ => {}
        }

        let is_a = self.a.id == node_id;
        let direct = self.a.point < self.b.point;
        let left = if direct {
            self.fill & SUBJ_TOP != 0
        } else {
            self.fill & SUBJ_BOTTOM != 0
        };

        is_a == (clockwise == left)
    }

    #[inline]
    pub(super) fn other(&self, node_id: usize) -> IdPoint {
        if self.a.id == node_id { self.b } else { self.a }
    }
}

impl StringGraph {
    #[inline(always)]
    pub(super) fn filter(&self, ext_rule: StringRule) -> Vec<NavigationLink> {
        match ext_rule {
            StringRule::Slice => self.filter_slice(),
        }
    }

    #[inline]
    fn filter_slice(&self) -> Vec<NavigationLink> {
        self.links
            .iter()
            .map(|link| {
                let fill = link.fill & SUBJ_BOTH;
                NavigationLink {
                    a: link.a,
                    b: link.b,
                    fill,
                }
            })
            .collect()
    }
}
