use crate::build::builder::GraphBuilder;
use crate::core::graph::OverlayNode;
use crate::core::overlay::Overlay;
use crate::segm::boolean::ShapeCountBoolean;
use crate::segm::segment::Segment;
use crate::split::solver::SplitSolver;
use alloc::vec::Vec;

impl Overlay<i32> {
    #[inline]
    pub(crate) fn add_segments(&mut self, segments: &[Segment<ShapeCountBoolean, i32>]) {
        self.segments.extend_from_slice(segments);
    }

    #[inline]
    pub(crate) fn with_segments(segments: Vec<Segment<ShapeCountBoolean, i32>>) -> Self {
        Self {
            solver: Default::default(),
            options: Default::default(),
            boolean_buffer: None,
            segments,
            split_solver: SplitSolver::new(),
            graph_builder: GraphBuilder::<ShapeCountBoolean, OverlayNode, i32>::new(),
        }
    }
}
