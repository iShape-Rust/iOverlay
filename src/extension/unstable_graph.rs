use crate::core::overlay_graph::OverlayGraph;
use crate::core::overlay_link::OverlayLink;
use crate::core::overlay_node::OverlayNode;
use crate::core::solver::Solver;
use crate::segm::segment::{Segment, SegmentFill};

pub struct UnstableGraph {
    pub(super) solver: Solver,
    pub(super) nodes: Vec<Vec<usize>>,
    pub(super) links: Vec<OverlayLink>,
}

impl UnstableGraph {

    #[inline]
    pub(super) fn new(solver: Solver, bundle: (Vec<Segment>, Vec<SegmentFill>)) -> Self {
        let (old_nodes, links) = OverlayGraph::build_nodes_and_links(&solver, bundle);

        let nodes = old_nodes.into_iter().map(|node| match node {
            OverlayNode::Bridge(data) => data.to_vec(),
            OverlayNode::Cross(indices) => indices
        }).collect();

        Self { solver, nodes, links }
    }

    #[inline(always)]
    pub(super) fn link(&self, index: usize) -> &OverlayLink {
        unsafe { self.links.get_unchecked(index) }
    }

    #[inline(always)]
    pub(super) fn node(&self, index: usize) -> &Vec<usize> {
        unsafe { self.nodes.get_unchecked(index) }
    }
}