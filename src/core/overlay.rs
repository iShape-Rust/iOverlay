//! This module contains functionality to construct and manage overlays, which are used to perform
//! boolean operations (union, intersection, etc.) on polygons. It provides structures and methods to
//! manage subject and clip polygons and convert them into graphs for further operations.

use i_float::int::point::IntPoint;
use i_shape::int::count::PointsCount;
use i_shape::int::path::IntPath;
use i_shape::int::shape::IntShape;

use crate::core::fill_rule::FillRule;
use crate::core::overlay_rule::OverlayRule;

use crate::core::solver::Solver;
use crate::fill::solver::{FillSolver, FillStrategy};
use crate::segm::build::BuildSegments;
use crate::segm::segment::{Segment, SegmentFill, CLIP_BOTH, NONE, SUBJ_BOTH};
use crate::segm::shape_count::ShapeCount;
use crate::split::solver::SplitSegments;
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
    pub(crate) segments: Vec<Segment>,
}

impl Overlay {
    /// Constructs a new `Overlay` instance, initializing it with a capacity that should closely match the total count of edges from all shapes being processed.
    /// This pre-allocation helps in optimizing memory usage and performance.
    /// - `capacity`: The initial capacity for storing edge data. Ideally, this should be set to the sum of the edges of all shapes to be added to the overlay, ensuring efficient data management.
    pub fn new(capacity: usize) -> Self {
        Self {
            segments: Vec::with_capacity(capacity),
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

    /// Adds a path to the overlay using an iterator, allowing for more flexible path input.
    /// This function is particularly useful when working with dynamically generated paths or
    /// when paths are not directly stored in a collection.
    /// - `iter`: An iterator over references to `IntPoint` that defines the path.
    /// - `shape_type`: Specifies the role of the added path in the overlay operation, either as `Subject` or `Clip`.
    #[inline]
    pub fn add_path_iter<I: Iterator<Item = IntPoint>>(&mut self, iter: I, shape_type: ShapeType) {
        self.segments.append_path_iter(iter, shape_type);
    }

    /// Adds a single path to the overlay as either subject or clip paths.
    /// - `path`: A reference to a `IntPath` instance to be added.
    /// - `shape_type`: Specifies the role of the added path in the overlay operation, either as `Subject` or `Clip`.
    #[inline]
    pub fn add_path(&mut self, path: &[IntPoint], shape_type: ShapeType) {
        self.segments.append_path_iter(path.iter().copied(), shape_type);
    }

    /// Adds multiple paths to the overlay as either subject or clip paths.
    /// - `paths`: An array of `IntPath` instances to be added to the overlay.
    /// - `shape_type`: Specifies the role of the added paths in the overlay operation, either as `Subject` or `Clip`.
    #[inline]
    pub fn add_paths(&mut self, paths: &[IntPath], shape_type: ShapeType) {
        for path in paths.iter() {
            self.add_path(path, shape_type);
        }
    }

    /// Adds a single shape to the overlay as either a subject or clip shape.
    /// - `shape`: A reference to a `IntShape` instance to be added.
    /// - `shape_type`: Specifies the role of the added shape in the overlay operation, either as `Subject` or `Clip`.
    #[inline]
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

    /// Convert into vector shapes from the added paths or shapes, applying the specified fill and overlay rules. This method is particularly useful for development purposes and for creating visualizations in educational demos, where understanding the impact of different rules on the final geometry is crucial.
    /// - `fill_rule`: The fill rule to use for the shapes.
    /// - `overlay_rule`: The overlay rule to apply.
    /// - `solver`: Type of solver to use.
    pub fn into_shape_vectors(self, fill_rule: FillRule, overlay_rule: OverlayRule, solver: Solver) -> Vec<VectorShape> {
        if self.segments.is_empty() {
            return Vec::new();
        }

        OverlayGraph::new(solver, self.into_segments(fill_rule, false, solver)).extract_shape_vectors(overlay_rule)
    }

    /// Convert into vectors from the added paths or shapes, applying the specified fill rule. This method is particularly useful for development purposes and for creating visualizations in educational demos, where understanding the impact of different rules on the final geometry is crucial.
    /// - `fill_rule`: The fill rule to use for the shapes.
    /// - `solver`: Type of solver to use.
    pub fn into_separate_vectors(self, fill_rule: FillRule, solver: Solver) -> Vec<VectorEdge> {
        if self.segments.is_empty() {
            return Vec::new();
        }

        OverlayGraph::new(solver, self.into_segments(fill_rule, false, solver)).extract_separate_vectors()
    }

    /// Convert into `OverlayGraph` from the added paths or shapes using the specified fill rule. This graph is the foundation for executing boolean operations, allowing for the analysis and manipulation of the geometric data. The `OverlayGraph` created by this method represents a preprocessed state of the input shapes, optimized for the application of boolean operations based on the provided fill rule.
    /// - `fill_rule`: Specifies the rule for determining filled areas within the shapes, influencing how the resulting graph represents intersections and unions.
    #[inline]
    pub fn into_graph(self, fill_rule: FillRule) -> OverlayGraph {
        self.into_graph_with_solver(fill_rule, Default::default())
    }

    /// Convert into `OverlayGraph` from the added paths or shapes using the specified fill rule. This graph is the foundation for executing boolean operations, allowing for the analysis and manipulation of the geometric data. The `OverlayGraph` created by this method represents a preprocessed state of the input shapes, optimized for the application of boolean operations based on the provided fill rule.
    /// - `fill_rule`: Specifies the rule for determining filled areas within the shapes, influencing how the resulting graph represents intersections and unions.
    /// - `solver`: Type of solver to use.
    #[inline]
    pub fn into_graph_with_solver(self, fill_rule: FillRule, solver: Solver) -> OverlayGraph {
        OverlayGraph::new(solver, self.into_segments(fill_rule, true, solver))
    }

    /// Convert into segments from the added paths or shapes according to the specified fill rule.
    /// - `fill_rule`: The fill rule to use when determining the inside of shapes.
    /// - `filter`: Is need to clean empty segments
    /// - `solver`: Type of solver to use.
    pub(crate) fn into_segments(self, fill_rule: FillRule, filter: bool, solver: Solver) -> (Vec<Segment>, Vec<SegmentFill>) {
        if self.segments.is_empty() {
            return (Vec::new(), Vec::new());
        }

        let mut segments = self.segments.split_segments(solver);

        let is_list = solver.is_list_fill(&segments);

        let mut fills = match fill_rule {
            FillRule::EvenOdd => FillSolver::fill::<EvenOddStrategy>(is_list, &segments),
            FillRule::NonZero => FillSolver::fill::<NonZeroStrategy>(is_list, &segments),
            FillRule::Positive => FillSolver::fill::<PositiveStrategy>(is_list, &segments),
            FillRule::Negative => FillSolver::fill::<NegativeStrategy>(is_list, &segments),
        };

        if filter {
            if let Some(first_empty_index) = fills.iter().position(|fill| fill.is_empty()) {
                filter_empties(&mut segments, &mut fills, first_empty_index);
            }
        }

        (segments, fills)
    }
}

fn filter_empties(segments: &mut Vec<Segment>, fills: &mut Vec<SegmentFill>, after: usize) {
    let mut j = after;
    for i in (after + 1)..fills.len() {
        let fill = fills[i];
        if !fill.is_empty() {
            fills[j] = fills[i];
            segments[j] = segments[i];
            j += 1;
        }
    }

    fills.truncate(j);
    segments.truncate(j);
}

trait Empty {
    fn is_empty(&self) -> bool;
}

impl Empty for SegmentFill {
    #[inline(always)]
    fn is_empty(&self) -> bool {
        let fill = *self;
        fill == NONE || fill == SUBJ_BOTH || fill == CLIP_BOTH
    }
}

pub(crate) struct EvenOddStrategy;

pub(crate) struct NonZeroStrategy;

pub(crate) struct PositiveStrategy;

pub(crate) struct NegativeStrategy;


impl FillStrategy for EvenOddStrategy {
    #[inline(always)]
    fn add_and_fill(this: ShapeCount, bot: ShapeCount) -> (ShapeCount, SegmentFill) {
        let top = bot.add(this);
        let subj_top = 1 & top.subj as SegmentFill;
        let subj_bot = 1 & bot.subj as SegmentFill;
        let clip_top = 1 & top.clip as SegmentFill;
        let clip_bot = 1 & bot.clip as SegmentFill;

        let fill = subj_top | (subj_bot << 1) | (clip_top << 2) | (clip_bot << 3);

        (top, fill)
    }
}

impl FillStrategy for NonZeroStrategy {
    #[inline(always)]
    fn add_and_fill(this: ShapeCount, bot: ShapeCount) -> (ShapeCount, SegmentFill) {
        let top = bot.add(this);
        let subj_top = (top.subj != 0) as SegmentFill;
        let subj_bot = (bot.subj != 0) as SegmentFill;
        let clip_top = (top.clip != 0) as SegmentFill;
        let clip_bot = (bot.clip != 0) as SegmentFill;

        let fill = subj_top | (subj_bot << 1) | (clip_top << 2) | (clip_bot << 3);

        (top, fill)
    }
}

impl FillStrategy for PositiveStrategy {
    #[inline(always)]
    fn add_and_fill(this: ShapeCount, bot: ShapeCount) -> (ShapeCount, SegmentFill) {
        let top = bot.add(this);
        let subj_top = (top.subj < 0) as SegmentFill;
        let subj_bot = (bot.subj < 0) as SegmentFill;
        let clip_top = (top.clip < 0) as SegmentFill;
        let clip_bot = (bot.clip < 0) as SegmentFill;

        let fill = subj_top | (subj_bot << 1) | (clip_top << 2) | (clip_bot << 3);

        (top, fill)
    }
}

impl FillStrategy for NegativeStrategy {
    #[inline(always)]
    fn add_and_fill(this: ShapeCount, bot: ShapeCount) -> (ShapeCount, SegmentFill) {
        let top = bot.add(this);
        let subj_top = (top.subj > 0) as SegmentFill;
        let subj_bot = (bot.subj > 0) as SegmentFill;
        let clip_top = (top.clip > 0) as SegmentFill;
        let clip_bot = (bot.clip > 0) as SegmentFill;

        let fill = subj_top | (subj_bot << 1) | (clip_top << 2) | (clip_bot << 3);

        (top, fill)
    }
}
