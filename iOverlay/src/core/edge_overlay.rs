use crate::build::builder::GraphBuilder;
use crate::core::edge_data::OverlayEdgeData;
use crate::core::fill_rule::FillRule;
use crate::core::graph::OverlayNode;
use crate::core::overlay::{IntOverlayOptions, ShapeType};
use crate::core::overlay_rule::OverlayRule;
use crate::core::solver::Solver;
use crate::segm::boolean::ShapeCountBoolean;
use crate::segm::segment::Segment;
use crate::segm::winding::WindingCount;
use crate::split::solver::SplitSolver;
use crate::vector::edge::DataVectorEdge;
use alloc::vec::Vec;
use i_float::int::point::IntPoint;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InputEdge<D> {
    pub a: IntPoint,
    pub b: IntPoint,
    pub data: D,
}

pub struct EdgeOverlay<D: OverlayEdgeData> {
    pub solver: Solver,
    pub options: IntOverlayOptions,
    segments: Vec<Segment<ShapeCountBoolean, D>>,
    split_solver: SplitSolver,
    graph_builder: GraphBuilder<ShapeCountBoolean, OverlayNode, D>,
}

impl<D: OverlayEdgeData> EdgeOverlay<D> {
    pub fn new(capacity: usize) -> Self {
        Self {
            solver: Default::default(),
            options: IntOverlayOptions::keep_output_points(),
            segments: Vec::with_capacity(capacity),
            split_solver: SplitSolver::new(),
            graph_builder: GraphBuilder::<ShapeCountBoolean, OverlayNode, D>::new(),
        }
    }

    pub fn add_edge(&mut self, edge: InputEdge<D>, shape_type: ShapeType) {
        if edge.a == edge.b {
            return;
        }

        let (direct, invert) = ShapeCountBoolean::with_shape_type(shape_type);
        self.segments.push(Segment::with_ab_and_data(
            edge.a, edge.b, direct, invert, edge.data,
        ));
    }

    pub fn add_edges<I>(&mut self, edges: I, shape_type: ShapeType)
    where
        I: IntoIterator<Item = InputEdge<D>>,
    {
        for edge in edges {
            self.add_edge(edge, shape_type);
        }
    }

    pub fn build_separate_vectors(
        &mut self,
        overlay_rule: OverlayRule,
        fill_rule: FillRule,
    ) -> Vec<DataVectorEdge<D>> {
        self.split_solver.split_segments(&mut self.segments, &self.solver);
        if self.segments.is_empty() {
            return Vec::new();
        }

        self.graph_builder
            .build_boolean_overlay(
                fill_rule,
                overlay_rule,
                self.options,
                &self.solver,
                &self.segments,
            )
            .extract_separate_data_vectors()
    }
}
