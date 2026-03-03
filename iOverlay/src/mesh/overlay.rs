use crate::build::builder::GraphBuilder;
use crate::core::graph::OverlayNode;
use crate::segm::segment::Segment;
use crate::split::solver::SplitSolver;
use alloc::vec::Vec;
use crate::core::overlay::Overlay;
use crate::segm::boolean::ShapeCountBoolean;

impl Overlay {
    #[inline]
    pub(crate) fn add_segments(&mut self, segments: &[Segment<ShapeCountBoolean>]) {
        self.segments.extend_from_slice(segments);
    }

    #[inline]
    pub(crate) fn with_segments(segments: Vec<Segment<ShapeCountBoolean>>) -> Self {
        Self {
            solver: Default::default(),
            options: Default::default(),
            boolean_buffer: None,
            segments,
            split_solver: SplitSolver::new(),
            graph_builder: GraphBuilder::<ShapeCountBoolean, OverlayNode>::new(),
        }
    }
}
