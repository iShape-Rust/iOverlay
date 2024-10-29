//! This module contains functionality to construct and manage overlays, which are used to perform
//! boolean operations (union, intersection, etc.) on polygons. It provides structures and methods to
//! manage subject and clip polygons and convert them into graphs for further operations.

use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_shape::base::data::{Contour, Shape, Shapes};
use i_shape::float::count::PointsCount;
use crate::core::fill_rule::FillRule;
use crate::core::overlay::{Overlay, ShapeType};
use crate::core::solver::Solver;
use crate::float::graph::FloatOverlayGraph;

/// This struct is essential for describing and uploading the geometry or shapes required to construct an `FloatOverlay`. It prepares the necessary data for boolean operations.
#[derive(Clone)]
pub struct FloatOverlay<T: FloatNumber> {
    pub(super) overlay: Overlay,
    pub(super) adapter: FloatPointAdapter<T>,
}

impl<T: FloatNumber> FloatOverlay<T> {
    #[inline]
    pub fn new(adapter: FloatPointAdapter<T>, capacity: usize) -> Self {
        Self { overlay: Overlay::new(capacity), adapter }
    }

    /// Creates a new `Overlay` instance and initializes it with subject and clip paths.
    /// - `subj_shapes`: An array of shapes that together define the subject.
    /// - `clip_shapes`: An array of shapes that together define the clip.
    pub fn with_shapes<P: FloatPointCompatible<T>>(subj_shapes: &Shapes<P>, clip_shapes: &Shapes<P>) -> Self {
        let subj_iter = subj_shapes.iter().flatten().flatten();
        let clip_iter = clip_shapes.iter().flatten().flatten();
        let iter = subj_iter.chain(clip_iter);
        let adapter = FloatPointAdapter::with_iter(iter);

        let capacity = subj_shapes.points_count() + clip_shapes.points_count();

        Self::new(adapter, capacity)
            .unsafe_add_shapes(subj_shapes, ShapeType::Subject)
            .unsafe_add_shapes(clip_shapes, ShapeType::Clip)
    }

    /// Creates a new `Overlay` instance and initializes it with subject and clip paths.
    /// - `subj_paths`: An array of paths that together define the subject.
    /// - `clip_paths`: An array of paths that together define the clip.
    pub fn with_paths<P: FloatPointCompatible<T>>(subj_paths: &Shape<P>, clip_paths: &Shape<P>) -> Self {
        let subj_iter = subj_paths.iter().flatten();
        let clip_iter = clip_paths.iter().flatten();
        let iter = subj_iter.chain(clip_iter);
        let adapter = FloatPointAdapter::with_iter(iter);

        let capacity = subj_paths.points_count() + clip_paths.points_count();

        Self::new(adapter, capacity)
            .unsafe_add_paths(subj_paths, ShapeType::Subject)
            .unsafe_add_paths(clip_paths, ShapeType::Clip)
    }

    /// Creates a new `Overlay` instance and initializes it with subject and clip path.
    /// - `subj_path`: A path that define the subject.
    /// - `clip_path`: A path that define the clip.
    pub fn with_path<P: FloatPointCompatible<T>>(subj_path: &Contour<P>, clip_path: &Contour<P>) -> Self {
        let iter = subj_path.iter().chain(clip_path.iter());
        let adapter = FloatPointAdapter::with_iter(iter);

        Self::new(adapter, subj_path.len() + clip_path.len())
            .unsafe_add_path(subj_path, ShapeType::Subject)
            .unsafe_add_path(clip_path, ShapeType::Clip)
    }

    /// Adds a single closed shape path to the overlay.
    /// - `path`: An array of points that form a closed path.
    /// - `shape_type`: Specifies the role of the added paths in the overlay operation, either as `Subject` or `Clip`.
    /// - **Safety**: Marked `unsafe` because it assumes the path is fully contained within the bounding box.
    #[inline]
    pub fn unsafe_add_path<P: FloatPointCompatible<T>>(mut self, path: &[P], shape_type: ShapeType) -> Self {
        self.overlay.add_path_iter(path.iter().map(|&p|self.adapter.float_to_int(p)), shape_type);
        self
    }

    /// Adds multiple closed shape paths to the overlay.
    /// - `paths`: An array of `Contour` instances, each representing a closed path.
    /// - `shape_type`: Specifies the role of the added paths in the overlay operation, either as `Subject` or `Clip`.
    /// - **Safety**: Marked `unsafe` because it assumes each path is fully contained within the bounding box.
    pub fn unsafe_add_paths<P: FloatPointCompatible<T>>(mut self, paths: &[Contour<P>], shape_type: ShapeType) -> Self {
        for path in paths.iter() {
            self = self.unsafe_add_path(path, shape_type);
        }
        self
    }

    /// Adds multiple shapes to the overlay.
    /// - `shapes`: An array of `Shape` instances.
    /// - `shape_type`: Specifies the role of the added paths in the overlay operation, either as `Subject` or `Clip`.
    /// - **Safety**: Marked `unsafe` because it assumes each path is fully contained within the bounding box.
    pub fn unsafe_add_shapes<P: FloatPointCompatible<T>>(mut self, shapes: &[Shape<P>], shape_type: ShapeType) -> Self {
        for shape in shapes.iter() {
            self = self.unsafe_add_paths(shape, shape_type);
        }
        self
    }

    /// Convert into `FloatOverlayGraph` from the added paths or shapes using the specified fill rule. This graph is the foundation for executing boolean operations, allowing for the analysis and manipulation of the geometric data. The `OverlayGraph` created by this method represents a preprocessed state of the input shapes, optimized for the application of boolean operations based on the provided fill rule.
    /// - `fill_rule`: Specifies the rule for determining filled areas within the shapes, influencing how the resulting graph represents intersections and unions.
    #[inline]
    pub fn into_graph(self, fill_rule: FillRule) -> FloatOverlayGraph<T> {
        self.into_graph_with_solver(fill_rule, Solver::AUTO)
    }

    /// Convert into `FloatOverlayGraph` from the added paths or shapes using the specified fill rule. This graph is the foundation for executing boolean operations, allowing for the analysis and manipulation of the geometric data. The `OverlayGraph` created by this method represents a preprocessed state of the input shapes, optimized for the application of boolean operations based on the provided fill rule.
    /// - `fill_rule`: Specifies the rule for determining filled areas within the shapes, influencing how the resulting graph represents intersections and unions.
    /// - `solver`: Type of solver to use.
    #[inline]
    pub fn into_graph_with_solver(self, fill_rule: FillRule, solver: Solver) -> FloatOverlayGraph<T> {
        let graph = self.overlay.into_graph_with_solver(fill_rule, solver);
        FloatOverlayGraph::new(graph, self.adapter)
    }
}
