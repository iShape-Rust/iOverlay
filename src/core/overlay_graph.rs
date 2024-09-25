//! This module defines the graph structure that represents the relationships between the paths in
//! subject and clip polygons after boolean operations. The graph helps in extracting final shapes
//! based on the overlay rule applied.

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
/// [More information](https://ishape-rust.github.io/iShape-js/overlay/overlay_graph/overlay_graph.html) about Overlay Graph.
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
        let mut indices = Vec::with_capacity(4);
        while next_a_cnt > 0 || next_b_cnt > 0 {
            let (a_cnt, b_cnt) = if a == b {
                (next_a_cnt, next_b_cnt)
            } else if next_a_cnt > 0 && a < b {
                (next_a_cnt, 0)
            } else {
                (0, next_b_cnt)
            };

            let node_id = nodes.len();

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
            // nodes.push(OverlayNode { indices });

            nodes.push(OverlayNode::new(indices.as_slice()));
            indices.clear();
        }

        debug_assert!(nodes.len() <= n);

        Self { solver, nodes, links }
    }

    pub(crate) fn find_nearest_counter_wise_link_to(
        &self,
        target_index: usize,
        node_id: usize,
        indices: &[usize],
        visited: &[bool],
    ) -> usize {
        let target = self.link(target_index);
        let (c, a) = if target.a.id == node_id {
            (target.a.point, target.b.point)
        } else { (target.b.point, target.a.point) };

        let (mut it_index, mut best_index) = indices.first_not_visited(visited);

        let mut link_index = indices.next_link(&mut it_index, visited);

        if link_index >= self.links.len() {
            // no more links
            return best_index;
        }

        let va = a.subtract(c);
        let b = self.link(best_index).other(node_id).point;
        let mut vb = b.subtract(c);
        let mut more_180 = va.cross_product(vb) <= 0;

        while link_index < self.links.len() {
            let link = &self.links[link_index];
            let p = link.other(node_id).point;
            let vp = p.subtract(c);
            let new_more_180 = va.cross_product(vp) <= 0;

            if new_more_180 == more_180 {
                // both more 180 or both less 180
                let is_clock_wise = vp.cross_product(vb) > 0;
                if is_clock_wise {
                    best_index = link_index;
                    vb = vp;
                }
            } else if more_180 {
                // new less 180
                more_180 = false;
                best_index = link_index;
                vb = vp;
            }

            link_index = indices.next_link(&mut it_index, visited);
        }

        best_index
    }

    #[inline]
    pub(crate) fn find_left_top_link(&self, link_index: usize, visited: &[bool]) -> usize {
        let top = self.link(link_index);
        debug_assert!(top.is_direct());

        let node = self.node(top.a.id);

        match node {
            OverlayNode::Bridge(bridge) => {
                self.find_left_top_link_on_bridge(bridge)
            }
            OverlayNode::Cross(indices) => {
                self.find_left_top_link_on_indices(top, link_index, indices, visited)
            }
        }
    }

    #[inline(always)]
    fn find_left_top_link_on_indices(&self, link: &OverlayLink, link_index: usize, indices: &[usize], visited: &[bool]) -> usize {
        let mut top_index = link_index;
        let mut top = link;

        // find most top link

        for &i in indices.iter() {
            if i == link_index {
                continue;
            }
            let link = self.link(i);
            if !link.is_direct() || Triangle::is_clockwise_point(top.a.point, top.b.point, link.b.point) {
                continue;
            }

            let &is_visited = unsafe { visited.get_unchecked(i) };
            if is_visited {
                continue;
            }

            top_index = i;
            top = link;
        }

        top_index
    }

    #[inline(always)]
    fn find_left_top_link_on_bridge(&self, bridge: &[usize; 2]) -> usize {
        let l0 = self.link(bridge[0]);
        let l1 = self.link(bridge[1]);
        if Triangle::is_clockwise_point(l0.a.point, l0.b.point, l1.b.point) {
            bridge[0]
        } else {
            bridge[1]
        }
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

trait OverlayNodeIndices {
    fn first_not_visited(&self, visited: &[bool]) -> (usize, usize);
    fn next_link(&self, it_index: &mut usize, visited: &[bool]) -> usize;
}

impl OverlayNodeIndices for [usize] {
    #[inline(always)]
    fn first_not_visited(&self, visited: &[bool]) -> (usize, usize) {
        let mut it_index = 0;
        while it_index < self.len() {
            let link_index = self[it_index];
            it_index += 1;
            let &is_visited = unsafe { visited.get_unchecked(link_index) };
            if !is_visited {
                return (it_index, link_index);
            }
        }
        unreachable!("The loop should always return");
    }

    #[inline(always)]
    fn next_link(&self, it_index: &mut usize, visited: &[bool]) -> usize {
        while *it_index < self.len() {
            let link_index = self[*it_index];
            *it_index += 1;
            let &is_visited = unsafe { visited.get_unchecked(link_index) };
            if !is_visited {
                return link_index
            }
        }

        usize::MAX
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