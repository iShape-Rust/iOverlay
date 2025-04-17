use crate::core::graph::OverlayGraph;
use crate::core::link::OverlayLink;
use crate::core::node::OverlayNode;
use crate::core::solver::Solver;

pub struct StringGraph {
    pub(super) solver: Solver,
    pub(super) nodes: Vec<Vec<usize>>,
    pub(super) links: Vec<OverlayLink>,
}

impl StringGraph {
    #[inline]
    pub(super) fn new(solver: Solver, links: Vec<OverlayLink>) -> Self {
        let mut m_links = links;
        let old_nodes = OverlayGraph::build_nodes_and_connect_links(&solver, &mut m_links);

        let nodes = old_nodes.into_iter().map(|node| match node {
            OverlayNode::Bridge(data) => data.to_vec(),
            OverlayNode::Cross(indices) => indices
        }).collect();

        Self { solver, nodes, links: m_links }
    }

    #[inline(always)]
    pub(super) fn node(&self, index: usize) -> &Vec<usize> {
        unsafe { self.nodes.get_unchecked(index) }
    }

    #[inline(always)]
    pub(super) fn link(&self, index: usize) -> &OverlayLink {
        unsafe { self.links.get_unchecked(index) }
    }
}