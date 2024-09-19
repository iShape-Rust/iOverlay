use i_float::point::IntPoint;
use i_float::triangle::Triangle;

use crate::core::solver::Solver;
use crate::id_point::IdPoint;
use crate::segm::end::End;
use crate::segm::segment::{Segment, SegmentFill};
use crate::sort::SmartSort;

use super::{overlay_link::OverlayLink, overlay_node::OverlayNode};


/// A representation of geometric shapes organized for efficient boolean operations.
///
/// `OverlayGraph` is a core structure designed to facilitate the execution of boolean operations on shapes, such as union, intersection, and difference. It organizes and preprocesses geometric data, making it optimized for these operations. This struct is the result of compiling shape data into a form where boolean operations can be applied directly, efficiently managing the complex relationships between different geometric entities.
///
/// Use `OverlayGraph` to perform boolean operations on the geometric shapes you've added to an `Overlay`, after it has processed the shapes according to the specified fill and overlay rules.
pub struct OverlayGraph {
    pub(crate) solver: Solver,
    pub(crate) nodes: Vec<OverlayNode>,
    pub(crate) links: Vec<OverlayLink>,
}

impl OverlayGraph {
    pub(super) fn new(solver: Solver, bundle: (Vec<Segment>, Vec<SegmentFill>)) -> Self {
        let segments = bundle.0;
        let fills = bundle.1;

        if segments.is_empty() {
            return Self { solver: Default::default(), nodes: vec![], links: vec![] };
        }

        let n = segments.len();
        let mut links: Vec<OverlayLink> = segments
            .into_iter().enumerate()
            .map(|(index, segment)| {
                let fill = *unsafe { fills.get_unchecked(index) };
                OverlayLink::new(
                    IdPoint { id: 0, point: segment.x_segment.a },
                    IdPoint { id: 0, point: segment.x_segment.b },
                    fill,
                )
            }).collect();

        let mut end_bs: Vec<End> = links.iter().enumerate()
            .map(|(i, link)| End { index: i, point: link.b.point })
            .collect();

        end_bs.smart_sort_by(&solver, |a, b| a.point.cmp(&b.point));

        let mut nodes: Vec<OverlayNode> = Vec::with_capacity(n);

        let mut ai = 0;
        let mut bi = 0;
        let mut a = links[0].a.point;
        let mut b = end_bs[0].point;
        let mut next_a_cnt = links.size(a, ai);
        let mut next_b_cnt = end_bs.size(b, bi);
        while next_a_cnt > 0 || next_b_cnt > 0 {
            let (a_cnt, b_cnt) = if a == b {
                (next_a_cnt, next_b_cnt)
            } else if next_a_cnt > 0 && a < b {
                (next_a_cnt, 0)
            } else {
                (0, next_b_cnt)
            };

            let node_id = nodes.len();
            let mut indices = Vec::with_capacity(a_cnt + b_cnt);

            if a_cnt > 0 {
                next_a_cnt = 0;
                for _ in 0..a_cnt {
                    unsafe { links.get_unchecked_mut(ai) }.a.id = node_id;
                    indices.push(ai);
                    ai += 1;
                }
                if ai < n {
                    a = unsafe { links.get_unchecked(ai) }.a.point;
                    next_a_cnt = links.size(a, ai);
                }
            }

            if b_cnt > 0 {
                next_b_cnt = 0;
                for _ in 0..b_cnt {
                    let e = unsafe { end_bs.get_unchecked(bi) };
                    indices.push(e.index);
                    unsafe { links.get_unchecked_mut(e.index) }.b.id = node_id;
                    bi += 1;
                }

                if bi < n {
                    b = unsafe { end_bs.get_unchecked(bi) }.point;
                    next_b_cnt = end_bs.size(b, bi);
                }
            }

            debug_assert!(indices.len() > 1, "indices: {}", indices.len());
            nodes.push(OverlayNode { indices });
        }

        debug_assert!(nodes.len() <= n);

        Self { solver, nodes, links }
    }

    pub(crate) fn find_nearest_counter_wise_link_to(
        &self,
        target_index: usize,
        node_id: usize,
        visited: &[bool],
    ) -> usize {
        let target = self.link(target_index);
        let (c, a) = if target.a.id == node_id {
            (target.a.point, target.b.point)
        } else { (target.b.point, target.a.point) };

        let node = self.node(node_id);

        let mut best_index = usize::MAX;
        let mut b = a;
        let mut more_180 = true;

        let mut it_index = 0;
        while it_index < node.indices.len() {
            let link_index = node.indices[it_index];
            it_index += 1;
            let &is_visited = unsafe { visited.get_unchecked(link_index) };
            if !is_visited {
                best_index = link_index;
                b = self.link(best_index).other_by_node_id(node_id).point;
                more_180 = Triangle::is_cw_or_line_point(c, a, b);
                break;
            }
        }

        while it_index < node.indices.len() {
            let link_index = node.indices[it_index];
            it_index += 1;
            let &is_visited = unsafe { visited.get_unchecked(link_index) };
            if is_visited {
                continue;
            }

            let p = self.link(link_index).other_by_node_id(node_id).point;
            let new_more_180 = Triangle::is_cw_or_line_point(c, a, p);
            if new_more_180 == more_180 {
                // both more 180 or both less 180
                if Triangle::is_clockwise_point(c, b, p) {
                    best_index = link_index;
                    b = p;
                }
            } else if more_180 {
                // new less 180
                more_180 = false;
                best_index = link_index;
                b = p;
            }
        }

        best_index
    }

    #[inline(always)]
    pub(crate) fn link(&self, index: usize) -> &OverlayLink {
        unsafe { self.links.get_unchecked(index) }
    }

    #[inline(always)]
    pub(crate) fn node(&self, index: usize) -> &OverlayNode {
        unsafe { self.nodes.get_unchecked(index) }
    }
}

trait Size {
    fn size(&self, point: IntPoint, index: usize) -> usize;
}

impl Size for Vec<OverlayLink> {
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