//! This module contains functionality to construct and manage overlays, which are used to perform
//! boolean operations (union, intersection, etc.) on polygons. It provides structures and methods to
//! manage subject and clip polygons and convert them into graphs for further operations.

use crate::core::fill_rule::FillRule;
use crate::core::link::OverlayLinkBuilder;
use crate::core::overlay_rule::OverlayRule;
use i_float::int::point::IntPoint;
use i_shape::int::count::PointsCount;
use i_shape::int::shape::{IntContour, IntShape, IntShapes};

use crate::core::solver::Solver;
use crate::segm::build::BuildSegments;
use crate::segm::segment::Segment;
use crate::segm::winding_count::ShapeCountBoolean;
use crate::vector::edge::{VectorEdge, VectorShape};

use super::graph::OverlayGraph;

/// Configuration options for polygon Boolean operations using [`Overlay`].
///
/// These options control precision, simplification, and contour filtering
/// during the Boolean operation process. You can use this to adjust output
/// direction, eliminate small artifacts, or retain collinear points.
#[derive(Debug, Clone, Copy)]
pub struct IntOverlayOptions {
    /// Preserve collinear points in the input before Boolean operations.
    pub preserve_input_collinear: bool,

    /// Desired direction for output contours (default outer: CCW / hole: CW).
    pub output_direction: ContourDirection,

    /// Preserve collinear points in the output after Boolean operations.
    pub preserve_output_collinear: bool,

    /// Minimum area threshold to include a contour in the result.
    pub min_output_area: u64,
}

/// Specifies the type of shape being processed, influencing how the shape participates in Boolean operations.
/// Note: All operations except for `Difference` are commutative, meaning the order of `Subject` and `Clip` shapes does not impact the outcome.
/// - `Subject`: The primary shape(s) for operations. Acts as the base layer in the operation.
/// - `Clip`: The modifying shape(s) that are applied to the `Subject`. Determines how the `Subject` is altered or intersected.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShapeType {
    Subject,
    Clip,
}

/// Represents the winding direction of a contour.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ContourDirection {
    CounterClockwise,
    Clockwise,
}

/// This struct is essential for describing and uploading the geometry or shapes required to construct an `OverlayGraph`. It prepares the necessary data for boolean operations.
#[derive(Clone)]
pub struct Overlay {
    pub options: IntOverlayOptions,
    pub(crate) segments: Vec<Segment<ShapeCountBoolean>>,
}

impl Overlay {
    /// Constructs a new `Overlay` instance, initializing it with a capacity that should closely match the total count of edges from all shapes being processed.
    /// This pre-allocation helps in optimizing memory usage and performance.
    /// - `capacity`: The initial capacity for storing edge data. Ideally, this should be set to the sum of the edges of all shapes to be added to the overlay, ensuring efficient data management.
    pub fn new(capacity: usize) -> Self {
        Self {
            options: Default::default(),
            segments: Vec::with_capacity(capacity)
        }
    }

    /// Constructs a new `Overlay` instance, initializing it with a capacity that should closely match the total count of edges from all shapes being processed.
    /// This pre-allocation helps in optimizing memory usage and performance.
    /// - `capacity`: The initial capacity for storing edge data. Ideally, this should be set to the sum of the edges of all shapes to be added to the overlay, ensuring efficient data management.
    /// - `options`: Adjust custom behavior.
    pub fn with_options(capacity: usize, options: IntOverlayOptions) -> Self {
        Self {
            options,
            segments: Vec::with_capacity(capacity)
        }
    }

    /// Creates a new `Overlay` instance and initializes it with subject and clip contours.
    /// - `subj`: An array of contours that together define the subject.
    /// - `clip`: An array of contours that together define the clip.
    pub fn with_contour(subj: &[IntPoint], clip: &[IntPoint]) -> Self {
        let mut overlay = Self::new(subj.len() + clip.len());
        overlay.add_contour(subj, ShapeType::Subject);
        overlay.add_contour(clip, ShapeType::Clip);
        overlay
    }

    /// Creates a new `Overlay` instance and initializes it with subject and clip contours.
    /// - `subj`: An array of contours that together define the subject.
    /// - `clip`: An array of contours that together define the clip.
    /// - `options`: Adjust custom behavior.
    pub fn with_contour_options(
        subj: &[IntPoint],
        clip: &[IntPoint],
        options: IntOverlayOptions,
    ) -> Self {
        let mut overlay = Self::with_options(subj.len() + clip.len(), options);
        overlay.add_contour(subj, ShapeType::Subject);
        overlay.add_contour(clip, ShapeType::Clip);
        overlay
    }

    /// Creates a new `Overlay` instance and initializes it with subject and clip contours.
    /// - `subj`: An array of contours that together define the subject shape.
    /// - `clip`: An array of contours that together define the clip shape.
    pub fn with_contours(subj: &[IntContour], clip: &[IntContour]) -> Self {
        let mut overlay = Self::new(subj.points_count() + clip.points_count());
        overlay.add_contours(subj, ShapeType::Subject);
        overlay.add_contours(clip, ShapeType::Clip);
        overlay
    }

    /// Creates a new `Overlay` instance and initializes it with subject and clip contours.
    /// - `subj`: An array of contours that together define the subject shape.
    /// - `clip`: An array of contours that together define the clip shape.
    /// - `options`: Adjust custom behavior.
    pub fn with_contours_options(
        subj: &[IntContour],
        clip: &[IntContour],
        options: IntOverlayOptions,
    ) -> Self {
        let mut overlay = Self::with_options(subj.points_count() + clip.points_count(), options);
        overlay.add_contours(subj, ShapeType::Subject);
        overlay.add_contours(clip, ShapeType::Clip);
        overlay
    }

    /// Creates a new `Overlay` instance and initializes it with subject and clip shapes.
    /// - `subj`: An array of shapes to be used as the subject in the overlay operation.
    /// - `clip`: An array of shapes to be used as the clip in the overlay operation.
    pub fn with_shapes(subj: &[IntShape], clip: &[IntShape]) -> Self {
        let mut overlay = Self::new(subj.points_count() + clip.points_count());
        overlay.add_shapes(subj, ShapeType::Subject);
        overlay.add_shapes(clip, ShapeType::Clip);
        overlay
    }

    /// Creates a new `Overlay` instance and initializes it with subject and clip shapes.
    /// - `subj`: An array of shapes to be used as the subject in the overlay operation.
    /// - `clip`: An array of shapes to be used as the clip in the overlay operation.
    /// - `options`: Adjust custom behavior.
    pub fn with_shapes_options(
        subj: &[IntShape],
        clip: &[IntShape],
        options: IntOverlayOptions,
    ) -> Self {
        let mut overlay = Self::with_options(subj.points_count() + clip.points_count(), options);
        overlay.add_shapes(subj, ShapeType::Subject);
        overlay.add_shapes(clip, ShapeType::Clip);
        overlay
    }

    /// Adds a path to the overlay using an iterator, allowing for more flexible path input.
    /// This function is particularly useful when working with dynamically generated paths or
    /// when paths are not directly stored in a collection.
    /// - `iter`: An iterator over references to `IntPoint` that defines the path.
    /// - `shape_type`: Specifies the role of the added path in the overlay operation, either as `Subject` or `Clip`.
    #[inline]
    pub fn add_path_iter<I: Iterator<Item = IntPoint>>(&mut self, iter: I, shape_type: ShapeType) {
        self.segments
            .append_path_iter(iter, shape_type, self.options.preserve_input_collinear);
    }

    /// Adds a single path to the overlay as either subject or clip paths.
    /// - `contour`: An array of points that form a closed path.
    /// - `shape_type`: Specifies the role of the added path in the overlay operation, either as `Subject` or `Clip`.
    #[inline]
    pub fn add_contour(&mut self, contour: &[IntPoint], shape_type: ShapeType) {
        self.segments.append_path_iter(
            contour.iter().copied(),
            shape_type,
            self.options.preserve_input_collinear,
        );
    }

    /// Adds multiple paths to the overlay as either subject or clip paths.
    /// - `contours`: An array of `IntContour` instances to be added to the overlay.
    /// - `shape_type`: Specifies the role of the added paths in the overlay operation, either as `Subject` or `Clip`.
    #[inline]
    pub fn add_contours(&mut self, contours: &[IntContour], shape_type: ShapeType) {
        for contour in contours.iter() {
            self.add_contour(contour, shape_type);
        }
    }

    /// Adds a single shape to the overlay as either a subject or clip shape.
    /// - `shape`: A reference to a `IntShape` instance to be added.
    /// - `shape_type`: Specifies the role of the added shape in the overlay operation, either as `Subject` or `Clip`.
    #[inline]
    pub fn add_shape(&mut self, shape: &IntShape, shape_type: ShapeType) {
        self.add_contours(shape, shape_type);
    }

    /// Adds multiple shapes to the overlay as either subject or clip shapes.
    /// - `shapes`: An array of `IntShape` instances to be added to the overlay.
    /// - `shape_type`: Specifies the role of the added shapes in the overlay operation, either as `Subject` or `Clip`.
    pub fn add_shapes(&mut self, shapes: &[IntShape], shape_type: ShapeType) {
        for shape in shapes.iter() {
            self.add_contours(shape, shape_type);
        }
    }

    /// Convert into vector shapes from the added paths or shapes, applying the specified fill and overlay rules. This method is particularly useful for development purposes and for creating visualizations in educational demos, where understanding the impact of different rules on the final geometry is crucial.
    /// - `fill_rule`: The fill rule to use for the shapes.
    /// - `overlay_rule`: The overlay rule to apply.
    /// - `solver`: Type of solver to use.
    pub fn into_shape_vectors(
        self,
        fill_rule: FillRule,
        overlay_rule: OverlayRule,
        solver: Solver,
    ) -> Vec<VectorShape> {
        let links = OverlayLinkBuilder::build_with_overlay_filter(
            self.segments,
            fill_rule,
            overlay_rule,
            solver,
        );
        let graph = OverlayGraph::new(solver, links);
        graph.extract_shape_vectors(overlay_rule)
    }

    /// Convert into vectors from the added paths or shapes, applying the specified fill rule. This method is particularly useful for development purposes and for creating visualizations in educational demos, where understanding the impact of different rules on the final geometry is crucial.
    /// - `fill_rule`: The fill rule to use for the shapes.
    /// - `solver`: Type of solver to use.
    pub fn into_separate_vectors(self, fill_rule: FillRule, solver: Solver) -> Vec<VectorEdge> {
        let links = OverlayLinkBuilder::build_without_filter(self.segments, fill_rule, solver);
        OverlayGraph::new(solver, links).extract_separate_vectors()
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
        let links = OverlayLinkBuilder::build_with_filler_filter(self.segments, fill_rule, solver);
        OverlayGraph::new(solver, links)
    }

    /// Executes a single Boolean operation on the current geometry using the specified overlay and fill rules.
    /// This method provides a streamlined approach for performing a Boolean operation without generating
    /// an entire `OverlayGraph`. Ideal for cases where only one Boolean operation is needed, `overlay`
    /// saves on computational resources by building only the necessary links, optimizing CPU usage by 0-20%
    /// compared to a full graph-based approach.
    ///
    /// ### Parameters:
    /// - `overlay_rule`: The boolean operation rule to apply, determining how shapes are combined or subtracted.
    /// - `fill_rule`: Specifies the rule for determining filled areas within the shapes, influencing how the resulting graph represents intersections and unions.
    /// - Returns: A vector of `IntShape` that meet the specified area criteria, representing the cleaned-up geometric result.
    /// # Shape Representation
    /// The output is a `IntShapes`, where:
    /// - The outer `Vec<IntShape>` represents a set of shapes.
    /// - Each shape `Vec<IntContour>` represents a collection of contours, where the first contour is the outer boundary, and all subsequent contours are holes in this boundary.
    /// - Each path `Vec<IntPoint>` is a sequence of points, forming a closed path.
    ///
    /// Note: Outer boundary paths have a counterclockwise order, and holes have a clockwise order.
    /// ### Usage:
    /// This function is suitable when a single, optimized Boolean operation is required on the provided
    /// geometry. For example:
    ///
    /// ```rust
    /// use i_float::int::point::IntPoint;
    /// use i_float::int_pnt;
    /// use i_overlay::core::fill_rule::FillRule;
    /// use i_overlay::core::overlay::Overlay;
    /// use i_overlay::core::overlay_rule::OverlayRule;
    ///
    /// let left_rect = [int_pnt!(0, 0), int_pnt!(0, 10), int_pnt!(10, 10), int_pnt!(10, 0)];
    /// let right_rect = [int_pnt!(10, 0), int_pnt!(10, 10), int_pnt!(20, 10), int_pnt!(20, 0)];
    /// let overlay = Overlay::with_contour(&left_rect, &right_rect);
    ///
    /// let result = overlay.overlay(OverlayRule::Union, FillRule::EvenOdd);
    /// ```
    ///
    /// This method is particularly useful in scenarios where the geometry only needs one overlay operation
    /// without subsequent modifications. By excluding unnecessary graph structures, it optimizes performance,
    /// particularly for complex or resource-intensive geometries.
    #[inline]
    pub fn overlay(self, overlay_rule: OverlayRule, fill_rule: FillRule) -> IntShapes {
        self.overlay_custom(
            overlay_rule,
            fill_rule,
            Default::default(),
        )
    }

    /// Executes a single Boolean operation on the current geometry using the specified overlay and fill rules.
    /// This method provides a streamlined approach for performing a Boolean operation without generating
    /// an entire `OverlayGraph`. Ideal for cases where only one Boolean operation is needed, `overlay`
    /// saves on computational resources by building only the necessary links, optimizing CPU usage by 0-20%
    /// compared to a full graph-based approach.
    ///
    /// ### Parameters:
    /// - `overlay_rule`: The boolean operation rule to apply, determining how shapes are combined or subtracted.
    /// - `fill_rule`: Specifies the rule for determining filled areas within the shapes, influencing how the resulting graph represents intersections and unions.
    /// - Returns: A vector of `IntShape` that meet the specified area criteria, representing the cleaned-up geometric result.
    /// # Shape Representation
    /// The output is a `IntShapes`, where:
    /// - The outer `Vec<IntShape>` represents a set of shapes.
    /// - Each shape `Vec<IntContour>` represents a collection of contours, where the first contour is the outer boundary, and all subsequent contours are holes in this boundary.
    /// - Each path `Vec<IntPoint>` is a sequence of points, forming a closed path.
    ///
    /// This method is particularly useful in scenarios where the geometry only needs one overlay operation
    /// without subsequent modifications. By excluding unnecessary graph structures, it optimizes performance,
    /// particularly for complex or resource-intensive geometries.
    ///
    /// Note: Outer boundary paths have a **main_direction** order, and holes have an opposite to **main_direction** order.
    #[inline]
    pub fn overlay_custom(
        self,
        overlay_rule: OverlayRule,
        fill_rule: FillRule,
        solver: Solver,
    ) -> IntShapes {
        let links = OverlayLinkBuilder::build_with_overlay_filter(
            self.segments,
            fill_rule,
            overlay_rule,
            solver,
        );
        let graph = OverlayGraph::new(solver, links);
        let filter = vec![false; graph.links.len()];
        graph.extract(filter, overlay_rule, self.options)
    }
}

impl Default for IntOverlayOptions {
    fn default() -> Self {
        Self {
            preserve_input_collinear: false,
            output_direction: ContourDirection::CounterClockwise,
            preserve_output_collinear: false,
            min_output_area: 0,
        }
    }
}

impl IntOverlayOptions {
    pub fn keep_all_points() -> Self {
        Self {
            preserve_input_collinear: true,
            output_direction: ContourDirection::CounterClockwise,
            preserve_output_collinear: true,
            min_output_area: 0,
        }
    }
    pub fn keep_output_points() -> Self {
        Self {
            preserve_input_collinear: false,
            output_direction: ContourDirection::CounterClockwise,
            preserve_output_collinear: true,
            min_output_area: 0,
        }
    }
}
