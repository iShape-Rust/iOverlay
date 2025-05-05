//! This module defines the graph structure that represents the relationships between the paths in
//! subject and clip polygons after boolean operations. The graph helps in extracting final shapes
//! based on the overlay rule applied.

use i_float::int::point::IntPoint;
use i_key_sort::sort::layout::BinStore;
use crate::core::solver::Solver;
use crate::geom::end::End;
use crate::util::sort::SmartBinSort;

use super::{link::OverlayLink, node::OverlayNode};


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
    #[inline]
    pub(crate) fn new(solver: Solver, links: Vec<OverlayLink>) -> Self {
        let mut m_links = links;
        let nodes = Self::build_nodes_and_connect_links(&solver, &mut m_links);
        Self { solver, nodes, links: m_links }
    }

    pub(crate) fn build_nodes_and_connect_links(solver: &Solver, links: &mut [OverlayLink]) -> Vec<OverlayNode> {
        let n = links.len();
        if n == 0 {
            return vec![];
        }

        let mut end_min = i32::MAX;
        let mut end_max = i32::MIN;
        for link in links.iter() {
            end_min = end_min.min(link.b.point.x);
            end_max = end_max.max(link.b.point.x);
        }

        let end_bs = if let Some(mut store) = BinStore::new(end_min, end_max, links.len()) {
            store.layout_bins(links.iter().map(|link|&link.b.point.x));
            store.into_sorted_by_bins_vec(
                links.len(), links.iter().enumerate()
                    .map(|(i, link)| End { index: i, point: link.b.point }), |a, b| a.point.cmp(&b.point))
        } else {
            let mut end_bs: Vec<End> = links.iter().enumerate()
                .map(|(i, link)| End { index: i, point: link.b.point })
                .collect();

            end_bs.smart_bin_sort_by(solver, |a, b| a.point.cmp(&b.point));
            end_bs
        };

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

            nodes.push(OverlayNode::new(indices.as_slice()));
            indices.clear();
        }

        nodes
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

impl OverlayGraph {
    pub fn validate(&self) {
        for node in self.nodes.iter() {
            if let OverlayNode::Cross(indices) = node {
                debug_assert!(indices.len() > 1, "indices: {}", indices.len());
                debug_assert!(self.nodes.len() <= self.links.len(), "nodes is more then links");
            }
        }
    }
}