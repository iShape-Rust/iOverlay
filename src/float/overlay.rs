//! This module contains functionality to construct and manage overlays, which are used to perform
//! boolean operations (union, intersection, etc.) on polygons. It provides structures and methods to
//! manage subject and clip polygons and convert them into graphs for further operations.

use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_shape::base::data::{Contour, Shape};
use i_shape::float::count::PointsCount;
use crate::core::fill_rule::FillRule;
use crate::core::overlay::{Overlay, ShapeType};
use crate::core::solver::Solver;
use crate::float::graph::FloatOverlayGraph;

/// This struct is essential for describing and uploading the geometry or shapes required to construct an `FloatOverlay`. It prepares the necessary data for boolean operations.
#[derive(Clone)]
pub struct FloatOverlay<P: FloatPointCompatible<T>, T: FloatNumber> {
    pub(super) overlay: Overlay,
    pub(super) adapter: FloatPointAdapter<P, T>,
}

impl<P: FloatPointCompatible<T>, T: FloatNumber> FloatOverlay<P, T> {
    /// Constructs a new `FloatOverlay`, a builder for overlaying geometric shapes
    /// by converting float-based geometry to integer space, using a pre-configured adapter.
    ///
    /// - `adapter`: A `FloatPointAdapter` instance responsible for coordinate conversion between
    ///   float and integer values, ensuring accuracy during geometric transformations.
    /// - `capacity`: Initial capacity for storing segments, ideally matching the total number of
    ///   segments for efficient memory allocation.
    #[inline]
    pub fn with_adapter(adapter: FloatPointAdapter<P, T>, capacity: usize) -> Self {
        Self { overlay: Overlay::new(capacity), adapter }
    }

    /// Creates a new `FloatOverlay` instance and initializes it with subject and clip paths.
    /// - `subj`: An array of shapes that together define the subject.
    /// - `clip`: An array of shapes that together define the clip.
    pub fn with_shapes(subj: &[Shape<P>], clip: &[Shape<P>]) -> Self {
        let subj_iter = subj.iter().flatten().flatten();
        let clip_iter = clip.iter().flatten().flatten();
        let iter = subj_iter.chain(clip_iter);
        let adapter = FloatPointAdapter::with_iter(iter);

        let capacity = subj.points_count() + clip.points_count();

        Self::with_adapter(adapter, capacity)
            .unsafe_add_shapes(subj, ShapeType::Subject)
            .unsafe_add_shapes(clip, ShapeType::Clip)
    }

    /// Creates a new `FloatOverlay` instance and initializes it with subject and clip paths.
    /// - `subj`: An array of contours that together define the subject.
    /// - `clip`: An array of contours that together define the clip.
    pub fn with_contours(subj: &[Contour<P>], clip: &[Contour<P>]) -> Self {
        let subj_iter = subj.iter().flatten();
        let clip_iter = clip.iter().flatten();
        let iter = subj_iter.chain(clip_iter);
        let adapter = FloatPointAdapter::with_iter(iter);

        let capacity = subj.points_count() + clip.points_count();

        Self::with_adapter(adapter, capacity)
            .unsafe_add_contours(subj, ShapeType::Subject)
            .unsafe_add_contours(clip, ShapeType::Clip)
    }

    /// Creates a new `FloatOverlay` instance and initializes it with subject and clip path.
    /// - `subj`: An array of points that define the subject.
    /// - `clip`: An array of points that define the clip.
    pub fn with_contour(subj: &[P], clip: &[P]) -> Self {
        let iter = subj.iter().chain(clip.iter());
        let adapter = FloatPointAdapter::with_iter(iter);

        Self::with_adapter(adapter, subj.len() + clip.len())
            .unsafe_add_contour(subj, ShapeType::Subject)
            .unsafe_add_contour(clip, ShapeType::Clip)
    }

    /// Adds a single closed shape path to the overlay.
    /// - `path`: An array of points that form a closed path.
    /// - `shape_type`: Specifies the role of the added paths in the overlay operation, either as `Subject` or `Clip`.
    /// - **Safety**: Marked `unsafe` because it assumes the path is fully contained within the bounding box.
    #[inline]
    pub fn unsafe_add_contour(mut self, path: &[P], shape_type: ShapeType) -> Self {
        self.overlay.add_path_iter(path.iter().map(|&p| self.adapter.float_to_int(p)), shape_type);
        self
    }

    /// Adds multiple closed shape paths to the overlay.
    /// - `contours`: An array of `Contour` instances, each representing a closed path.
    /// - `shape_type`: Specifies the role of the added paths in the overlay operation, either as `Subject` or `Clip`.
    /// - **Safety**: Marked `unsafe` because it assumes each path is fully contained within the bounding box.
    pub fn unsafe_add_contours(mut self, paths: &[Contour<P>], shape_type: ShapeType) -> Self {
        for path in paths.iter() {
            self = self.unsafe_add_contour(path, shape_type);
        }
        self
    }

    /// Adds multiple shapes to the overlay.
    /// - `shapes`: An array of `Shape` instances.
    /// - `shape_type`: Specifies the role of the added paths in the overlay operation, either as `Subject` or `Clip`.
    /// - **Safety**: Marked `unsafe` because it assumes each path is fully contained within the bounding box.
    pub fn unsafe_add_shapes(mut self, shapes: &[Shape<P>], shape_type: ShapeType) -> Self {
        for shape in shapes.iter() {
            self = self.unsafe_add_contours(shape, shape_type);
        }
        self
    }

    /// Convert into `FloatOverlayGraph` from the added paths or shapes using the specified fill rule. This graph is the foundation for executing boolean operations, allowing for the analysis and manipulation of the geometric data. The `OverlayGraph` created by this method represents a preprocessed state of the input shapes, optimized for the application of boolean operations based on the provided fill rule.
    /// - `fill_rule`: Specifies the rule for determining filled areas within the shapes, influencing how the resulting graph represents intersections and unions.
    #[inline]
    pub fn into_graph(self, fill_rule: FillRule) -> FloatOverlayGraph<P, T> {
        self.into_graph_with_solver(fill_rule, Solver::AUTO)
    }

    /// Convert into `FloatOverlayGraph` from the added paths or shapes using the specified fill rule. This graph is the foundation for executing boolean operations, allowing for the analysis and manipulation of the geometric data. The `OverlayGraph` created by this method represents a preprocessed state of the input shapes, optimized for the application of boolean operations based on the provided fill rule.
    /// - `fill_rule`: Specifies the rule for determining filled areas within the shapes, influencing how the resulting graph represents intersections and unions.
    /// - `solver`: Type of solver to use.
    #[inline]
    pub fn into_graph_with_solver(self, fill_rule: FillRule, solver: Solver) -> FloatOverlayGraph<P, T> {
        let graph = self.overlay.into_graph_with_solver(fill_rule, solver);
        FloatOverlayGraph::new(graph, self.adapter)
    }
}
