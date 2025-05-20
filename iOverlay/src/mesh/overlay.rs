use alloc::vec::Vec;
use crate::build::builder::GraphBuilder;
use crate::core::graph::OverlayNode;
use crate::core::solver::Solver;
use crate::mesh::graph::OffsetGraph;
use crate::segm::offset::ShapeCountOffset;
use crate::segm::segment::Segment;
use crate::split::solver::SplitSolver;

pub struct OffsetOverlay {
    pub(super) segments: Vec<Segment<ShapeCountOffset>>,
    pub(crate) split_solver: SplitSolver,
    pub(crate) graph_builder: GraphBuilder<ShapeCountOffset, OverlayNode>
}

impl OffsetOverlay {
    #[inline]
    pub fn new(capacity: usize) -> Self {
        Self {
            segments: Vec::with_capacity(capacity),
            split_solver: SplitSolver::new(),
            graph_builder: GraphBuilder::<ShapeCountOffset, OverlayNode>::new()
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.segments.clear();
    }


    #[inline]
    pub fn add_segments(&mut self, segments: &[Segment<ShapeCountOffset>]) {
        self.segments.extend_from_slice(segments);
    }

    #[inline]
    pub fn with_segments(segments: Vec<Segment<ShapeCountOffset>>) -> Self {
        Self {
            segments,
            split_solver: SplitSolver::new(),
            graph_builder: GraphBuilder::<ShapeCountOffset, OverlayNode>::new()
        }
    }

    #[inline]
    pub fn build_graph_view_with_solver(&mut self, solver: Solver) -> Option<OffsetGraph> {
        self.split_solver.split_segments(&mut self.segments, &solver);
        if self.segments.is_empty() {
            return None;
        }
        let graph = self
            .graph_builder
            .build_offset(&solver, &self.segments);

        Some(graph)
    }
}