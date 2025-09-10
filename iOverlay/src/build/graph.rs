use crate::build::builder::{GraphBuilder, GraphNode};
use crate::core::link::OverlayLink;
use crate::core::solver::Solver;
use crate::geom::end::End;
use crate::i_float::int::point::IntPoint;
use crate::segm::winding::WindingCount;
use alloc::vec::Vec;
use i_key_sort::sort::key_sort::KeySort;
use i_shape::util::reserve::Reserve;

impl<C: WindingCount, N: GraphNode> GraphBuilder<C, N> {
    pub(super) fn build_nodes_and_connect_links(&mut self, solver: &Solver) {
        let n = self.links.len();
        if n == 0 {
            return;
        }

        self.build_ends(solver);
        self.nodes.reserve_capacity(self.links.len());
        self.nodes.clear();

        let mut ai = 0;
        let mut bi = 0;
        let mut a = self.links[0].a.point;
        let mut b = self.ends[0].point;
        let mut next_a_cnt = self.links.size(a, ai);
        let mut next_b_cnt = self.ends.size(b, bi);
        let mut indices = Vec::with_capacity(4);
        while next_a_cnt > 0 || next_b_cnt > 0 {
            let (a_cnt, b_cnt) = if a == b {
                (next_a_cnt, next_b_cnt)
            } else if next_a_cnt > 0 && a < b {
                (next_a_cnt, 0)
            } else {
                (0, next_b_cnt)
            };

            let node_id = self.nodes.len();

            if a_cnt > 0 {
                next_a_cnt = 0;
                for _ in 0..a_cnt {
                    unsafe { self.links.get_unchecked_mut(ai) }.a.id = node_id;
                    indices.push(ai);
                    ai += 1;
                }
                if ai < n {
                    a = unsafe { self.links.get_unchecked(ai) }.a.point;
                    next_a_cnt = self.links.size(a, ai);
                }
            }

            if b_cnt > 0 {
                next_b_cnt = 0;
                for _ in 0..b_cnt {
                    let e = unsafe { self.ends.get_unchecked(bi) };
                    indices.push(e.index);
                    unsafe { self.links.get_unchecked_mut(e.index) }.b.id = node_id;
                    bi += 1;
                }

                if bi < n {
                    b = unsafe { self.ends.get_unchecked(bi) }.point;
                    next_b_cnt = self.ends.size(b, bi);
                }
            }

            self.nodes.push(N::with_indices(indices.as_slice()));
            indices.clear();
        }
    }

    #[inline]
    fn build_ends(&mut self, solver: &Solver) {
        self.ends.clear();
        self.ends.reserve_capacity(self.links.len());
        for (i, link) in self.links.iter().enumerate() {
            self.ends.push(End {
                index: i,
                point: link.b.point,
            });
        }
        self.ends
            .sort_by_two_keys(solver.is_parallel_sort_allowed(), |e| e.point.x, |e| e.point.y);
    }
}

trait Size {
    fn size(&self, point: IntPoint, index: usize) -> usize;
}

impl Size for [OverlayLink] {
    #[inline]
    fn size(&self, point: IntPoint, index: usize) -> usize {
        let mut i = index + 1;
        while i < self.len() && self[i].a.point == point {
            i += 1;
        }

        i - index
    }
}

impl Size for Vec<End> {
    #[inline]
    fn size(&self, point: IntPoint, index: usize) -> usize {
        let mut i = index + 1;
        while i < self.len() && self[i].point == point {
            i += 1;
        }

        i - index
    }
}
