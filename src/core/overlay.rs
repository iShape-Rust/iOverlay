use i_float::point::IntPoint;
use i_shape::int::path::IntPath;
use i_shape::int::shape::{IntShape, PointsCount};
use i_shape::int::simple::Simple;

use crate::core::fill_rule::FillRule;
use crate::core::overlay_rule::OverlayRule;
use crate::segm::shape_count::ShapeCount;
use crate::segm::segment::{CLIP_BOTH, NONE, SegmentFill, ShapeEdgesMerge, SUBJ_BOTH};

use crate::core::solver::Solver;
use crate::fill::solver::FillSolver;
use crate::segm::segment::Segment;
use crate::segm::x_segment::XSegment;
use crate::sort::SmartSort;
use crate::split::solver::SplitSolver;
use crate::vector::edge::{VectorEdge, VectorShape};

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
        self.edges.append_edges(path, shape_type);
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
        self.add_paths(shape, shape_type);
    }

    /// Adds multiple shapes to the overlay as either subject or clip shapes.
    /// - `shapes`: An array of `IntShape` instances to be added to the overlay.
    /// - `shape_type`: Specifies the role of the added shapes in the overlay operation, either as `Subject` or `Clip`.
    pub fn add_shapes(&mut self, shapes: &[IntShape], shape_type: ShapeType) {
        for shape in shapes.iter() {
            self.add_paths(shape, shape_type);
        }
    }

    /// Convert into segments from the added paths or shapes according to the specified fill rule.
    /// - `fill_rule`: The fill rule to use when determining the inside of shapes.
    /// - `solver`: Type of solver to use.
    pub(crate) fn into_segments(self, fill_rule: FillRule, solver: Solver) -> (Vec<Segment>, Vec<SegmentFill>) {
        if self.edges.is_empty() {
            return (Vec::new(), Vec::new());
        }

        let (mut segments, mut fills) = self.prepare_segments_and_fills(fill_rule, solver);

        clean_if_needed(&mut segments, &mut fills);
        (segments, fills)
    }

    /// Convert into vector shapes from the added paths or shapes, applying the specified fill and overlay rules. This method is particularly useful for development purposes and for creating visualizations in educational demos, where understanding the impact of different rules on the final geometry is crucial.
    /// - `fill_rule`: The fill rule to use for the shapes.
    /// - `overlay_rule`: The overlay rule to apply.
    /// - `solver`: Type of solver to use.
    pub fn into_shape_vectors(self, fill_rule: FillRule, overlay_rule: OverlayRule, solver: Solver) -> Vec<VectorShape> {
        if self.edges.is_empty() {
            return Vec::new();
        }
        let graph = OverlayGraph::new(solver, self.prepare_segments_and_fills(fill_rule, solver));

        graph.extract_shape_vectors(overlay_rule)
    }

    /// Convert into vectors from the added paths or shapes, applying the specified fill rule. This method is particularly useful for development purposes and for creating visualizations in educational demos, where understanding the impact of different rules on the final geometry is crucial.
    /// - `fill_rule`: The fill rule to use for the shapes.
    /// - `solver`: Type of solver to use.
    pub fn into_separate_vectors(self, fill_rule: FillRule, solver: Solver) -> Vec<VectorEdge> {
        if self.edges.is_empty() {
            return Vec::new();
        }
        let graph = OverlayGraph::new(solver, self.prepare_segments_and_fills(fill_rule, solver));
        graph.extract_separate_vectors()
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

        SplitSolver::new(solver).split(&mut segments);

        let is_list = solver.is_list_fill(&segments);
        let fills = FillSolver::fill(fill_rule, is_list, &segments);

        (segments, fills)
    }
}

trait BuildEdges {
    fn append_edges(&mut self, path: &[IntPoint], shape_type: ShapeType);

    fn append_private_edges(&mut self, path: &[IntPoint], shape_type: ShapeType);
}

impl BuildEdges for Vec<Segment> {
    #[inline]
    fn append_edges(&mut self, path: &[IntPoint], shape_type: ShapeType) {
        if path.is_simple() {
            self.append_private_edges(path, shape_type);
        } else {
            let path = path.to_simple();
            if path.len() > 2 {
                self.append_private_edges(path.as_slice(), shape_type);
            }
        }
    }

    fn append_private_edges(&mut self, path: &[IntPoint], shape_type: ShapeType) {
        let mut p0 = path[path.len() - 1];

        match shape_type {
            ShapeType::Subject => {
                for &p1 in path {
                    let segment = if p0 < p1 {
                        Segment { x_segment: XSegment { a: p0, b: p1 }, count: ShapeCount::new(1, 0) }
                    } else {
                        Segment { x_segment: XSegment { a: p1, b: p0 }, count: ShapeCount::new(-1, 0) }
                    };
                    self.push(segment);
                    p0 = p1
                }
            }
            ShapeType::Clip => {
                for &p1 in path {
                    let segment = if p0 < p1 {
                        Segment { x_segment: XSegment { a: p0, b: p1 }, count: ShapeCount::new(0, 1) }
                    } else {
                        Segment { x_segment: XSegment { a: p1, b: p0 }, count: ShapeCount::new(0, -1) }
                    };
                    self.push(segment);
                    p0 = p1
                }
            }
        }
    }
}

trait Fill {
    fn is_empty(&self) -> bool;
}

impl Fill for SegmentFill {
    #[inline(always)]
    fn is_empty(&self) -> bool {
        *self == NONE || *self == SUBJ_BOTH || *self == CLIP_BOTH
    }
}

#[inline(always)]
fn clean_if_needed(segments: &mut Vec<Segment>, fills: &mut Vec<SegmentFill>) {
    if let Some(first_empty_index) = fills.iter().position(|fill| fill.is_empty()) {
        clean(segments, fills, first_empty_index);
    }
}

fn clean(segments: &mut Vec<Segment>, fills: &mut Vec<SegmentFill>, after: usize) {
    let mut j = after;

    for i in (after + 1)..fills.len() {
        if !fills[i].is_empty() {
            fills[j] = fills[i];
            segments[j] = segments[i];
            j += 1;
        }
    }

    fills.truncate(j);
    segments.truncate(j);
}