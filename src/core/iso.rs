use crate::core::fill_rule::FillRule;
use crate::core::graph::OverlayGraph;
use crate::core::link::OverlayLinkBuilder;
use crate::core::overlay::Overlay;
use crate::core::solver::Solver;

impl Overlay {

    /// Convert into `OverlayGraph` from the added paths or shapes using the specified fill rule. This graph is the foundation for executing boolean operations, allowing for the analysis and manipulation of the geometric data. The `OverlayGraph` created by this method represents a preprocessed state of the input shapes, optimized for the application of boolean operations based on the provided fill rule.
    /// - `fill_rule`: Specifies the rule for determining filled areas within the shapes, influencing how the resulting graph represents intersections and unions.
    /// - `solver`: Type of solver to use.
    #[inline]
    pub fn into_iso_graph_with_solver(self, fill_rule: FillRule, solver: Solver) -> OverlayGraph {
        let links = OverlayLinkBuilder::iso_build_with_filler_filter(self.segments, fill_rule, solver);
        OverlayGraph::new(solver, links)
    }
}