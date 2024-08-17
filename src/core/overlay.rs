use i_float::point::IntPoint;
use i_shape::int::path::IntPath;
use i_shape::int::shape::{IntShape, PointsCount};
use i_shape::int::simple::Simple;
use crate::fill::fill_segments::FillSegments;

use crate::core::fill_rule::FillRule;
use crate::core::overlay_rule::OverlayRule;
use crate::segm::shape_count::ShapeCount;
use crate::segm::segment::{CLIP_BOTH, SegmentFill, ShapeEdgesMerge, SUBJ_BOTH};

use crate::core::solver::Solver;
use crate::segm::segment::Segment;
use crate::sort::SmartSort;
use crate::split::solver::SplitSolver;
use crate::vector::vector::VectorShape;

use super::overlay_graph::OverlayGraph;

/// Specifies the type of shape being processed, influencing how the shape participates in Boolean operations.
/// Note: All operations except for `Difference` are commutative, meaning the order of `Subject` and `Clip` shapes does not impact the outcome.
/// - `Subject`: The primary shape(s) for operations. Acts as the base layer in the operation.
/// - `Clip`: The modifying shape(s) that are applied to the `Subject`. Determines how the `Subject` is altered or intersected.
#[derive(Debug, Clone, Copy)]
pub enum ShapeType {
    Subject,
    Clip,
}

/// This struct is essential for describing and uploading the geometry or shapes required to construct an `OverlayGraph`. It prepares the necessary data for boolean operations.
#[derive(Clone)]
pub struct Overlay {
    edges: Vec<Segment>,
}

impl Overlay {
    /// Constructs a new `Overlay` instance, initializing it with a capacity that should closely match the total count of edges from all shapes being processed.
    /// This pre-allocation helps in optimizing memory usage and performance.
    /// - `capacity`: The initial capacity for storing edge data. Ideally, this should be set to the sum of the edges of all shapes to be added to the overlay, ensuring efficient data management.
    pub fn new(capacity: usize) -> Self {
        Self {
            edges: Vec::with_capacity(capacity),
        }
    }

    /// Creates a new `Overlay` instance and initializes it with subject and clip paths.
    /// - `subject_paths`: An array of paths that together define the subject shape.
    /// - `clip_paths`: An array of paths that together define the clip shape.
    pub fn with_paths(subject_paths: &[IntPath], clip_paths: &[IntPath]) -> Self {
        let mut overlay = Self::new(subject_paths.points_count() + clip_paths.points_count());
        overlay.add_paths(subject_paths, ShapeType::Subject);
        overlay.add_paths(clip_paths, ShapeType::Clip);
        overlay
    }

    /// Creates a new `Overlay` instance and initializes it with subject and clip shapes.
    /// - `subject_shapes`: An array of shapes to be used as the subject in the overlay operation.
    /// - `clip_shapes`: An array of shapes to be used as the clip in the overlay operation.
    pub fn with_shapes(subject_shapes: &[IntShape], clip_shapes: &[IntShape]) -> Self {
        let mut overlay = Self::new(subject_shapes.points_count() + clip_shapes.points_count());
        overlay.add_shapes(subject_shapes, ShapeType::Subject);
        overlay.add_shapes(clip_shapes, ShapeType::Clip);
        overlay
    }

    /// Adds a single path to the overlay as either subject or clip paths.
    /// - `path`: A reference to a `IntPath` instance to be added.
    /// - `shape_type`: Specifies the role of the added path in the overlay operation, either as `Subject` or `Clip`.
    pub fn add_path(&mut self, path: &[IntPoint], shape_type: ShapeType) {
        if let Some(mut edges) = path.option_edges(shape_type) {
            self.edges.append(&mut edges);
        }
    }

    /// Adds multiple paths to the overlay as either subject or clip paths.
    /// - `paths`: An array of `IntPath` instances to be added to the overlay.
    /// - `shape_type`: Specifies the role of the added paths in the overlay operation, either as `Subject` or `Clip`.
    pub fn add_paths(&mut self, paths: &[IntPath], shape_type: ShapeType) {
        for path in paths.iter() {
            self.add_path(path, shape_type);
        }
    }

    /// Adds a single shape to the overlay as either a subject or clip shape.
    /// - `shape`: A reference to a `IntShape` instance to be added.
    /// - `shape_type`: Specifies the role of the added shape in the overlay operation, either as `Subject` or `Clip`.
    pub fn add_shape(&mut self, shape: &IntShape, shape_type: ShapeType) {
        self.add_paths(&shape, shape_type);
    }

    /// Adds multiple shapes to the overlay as either subject or clip shapes.
    /// - `shapes`: An array of `IntShape` instances to be added to the overlay.
    /// - `shape_type`: Specifies the role of the added shapes in the overlay operation, either as `Subject` or `Clip`.
    pub fn add_shapes(&mut self, shapes: &[IntShape], shape_type: ShapeType) {
        for shape in shapes.iter() {
            self.add_paths(&shape, shape_type);
        }
    }

    /// Convert into segments from the added paths or shapes according to the specified fill rule.
    /// - `fill_rule`: The fill rule to use when determining the inside of shapes.
    /// - `solver`: Type of solver to use.
    pub fn into_segments(self, fill_rule: FillRule, solver: Solver) -> (Vec<Segment>, Vec<SegmentFill>) {
        if self.edges.is_empty() {
            return (Vec::new(), Vec::new());
        }

        let (mut segments, mut fills) = self.prepare_segments_and_fills(fill_rule, solver);

        let mut iter = fills.iter();
        segments.retain(|_| {
            let fill = *iter.next().unwrap();
            !(fill == 0 || fill == SUBJ_BOTH || fill == CLIP_BOTH)
        });

        fills.retain(|&fill| {
            !(fill == 0 || fill == SUBJ_BOTH || fill == CLIP_BOTH)
        });

        (segments, fills)
    }

    /// Convert into vector shapes from the added paths or shapes, applying the specified fill and overlay rules. This method is particularly useful for development purposes and for creating visualizations in educational demos, where understanding the impact of different rules on the final geometry is crucial.
    /// - `fill_rule`: The fill rule to use for the shapes.
    /// - `overlay_rule`: The overlay rule to apply.
    /// - `solver`: Type of solver to use.
    pub fn into_vectors(self, fill_rule: FillRule, overlay_rule: OverlayRule, solver: Solver) -> Vec<VectorShape> {
        if self.edges.is_empty() {
            return Vec::new();
        }
        let graph = OverlayGraph::new(solver, self.prepare_segments_and_fills(fill_rule, solver));
        let vectors = graph.extract_vectors(overlay_rule);

        vectors
    }

    /// Convert into `OverlayGraph` from the added paths or shapes using the specified fill rule. This graph is the foundation for executing boolean operations, allowing for the analysis and manipulation of the geometric data. The `OverlayGraph` created by this method represents a preprocessed state of the input shapes, optimized for the application of boolean operations based on the provided fill rule.
    /// - `fill_rule`: Specifies the rule for determining filled areas within the shapes, influencing how the resulting graph represents intersections and unions.
    pub fn into_graph(self, fill_rule: FillRule) -> OverlayGraph {
        OverlayGraph::new(Default::default(), self.into_segments(fill_rule, Default::default()))
    }

    /// Convert into `OverlayGraph` from the added paths or shapes using the specified fill rule. This graph is the foundation for executing boolean operations, allowing for the analysis and manipulation of the geometric data. The `OverlayGraph` created by this method represents a preprocessed state of the input shapes, optimized for the application of boolean operations based on the provided fill rule.
    /// - `fill_rule`: Specifies the rule for determining filled areas within the shapes, influencing how the resulting graph represents intersections and unions.
    /// - `solver`: Type of solver to use.
    pub fn into_graph_with_solver(self, fill_rule: FillRule, solver: Solver) -> OverlayGraph {
        OverlayGraph::new(solver, self.into_segments(fill_rule, solver))
    }

    fn prepare_segments_and_fills(self, fill_rule: FillRule, solver: Solver) -> (Vec<Segment>, Vec<SegmentFill>) {
        let mut segments = self.edges;
        segments.smart_sort_by(&solver, |a, b| a.x_segment.cmp(&b.x_segment));

        segments.merge_if_needed();

        let is_list = SplitSolver::new(solver).split(&mut segments);

        let fills = segments.fill(fill_rule, is_list);

        (segments, fills)
    }
}

trait CreateEdges {
    fn option_edges(&self, shape_type: ShapeType) -> Option<Vec<Segment>>;
    fn edges(&self, shape_type: ShapeType) -> Vec<Segment>;
}

impl CreateEdges for &[IntPoint] {
    fn option_edges(&self, shape_type: ShapeType) -> Option<Vec<Segment>> {
        if self.is_simple() {
            Some(self.edges(shape_type))
        } else {
            let path = self.to_simple();
            if path.len() > 2 {
                Some(self.to_simple().as_slice().edges(shape_type))
            } else {
                None
            }
        }
    }

    fn edges(&self, shape_type: ShapeType) -> Vec<Segment> {
        let n = self.len();

        let mut edges = vec![Segment::ZERO; n];

        let i0 = n - 1;
        let mut p0 = self[i0];

        match shape_type {
            ShapeType::Subject => {
                for i in 0..n {
                    let p1 = self[i];
                    let value = if p0 < p1 { 1 } else { -1 };
                    edges[i] = Segment::new(p0, p1, ShapeCount::new(value, 0));
                    p0 = p1
                }
            }
            ShapeType::Clip => {
                for i in 0..n {
                    let p1 = self[i];
                    let value = if p0 < p1 { 1 } else { -1 };
                    edges[i] = Segment::new(p0, p1, ShapeCount::new(0, value));
                    p0 = p1
                }
            }
        }

        edges
    }
}