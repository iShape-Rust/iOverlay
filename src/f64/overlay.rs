//! This module contains functionality to construct and manage overlays, which are used to perform
//! boolean operations (union, intersection, etc.) on polygons. It provides structures and methods to
//! manage subject and clip polygons and convert them into graphs for further operations.

use i_float::f64_adapter::F64PointAdapter;
use i_float::f64_rect::F64Rect;
use i_shape::f64::adapter::ShapeToInt;
use i_shape::f64::rect::RectInit;
use i_shape::f64::shape::{F64Path, F64Shapes};
use crate::core::fill_rule::FillRule;
use crate::core::overlay::{Overlay, ShapeType};
use crate::core::solver::Solver;
use crate::f64::graph::F64OverlayGraph;


// #[deprecated(
//     since = "1.8.0",
//     note = "Use FloatOverlay<P, T> instead, which provides a more flexible and efficient API"
// )]
/// This struct is essential for describing and uploading the geometry or shapes required to construct an `F64OverlayGraph`. It prepares the necessary data for boolean operations.
#[derive(Clone)]
pub struct F64Overlay {
    subj_paths: Vec<F64Path>,
    clip_paths: Vec<F64Path>,
}

impl F64Overlay {
    #[inline]
    pub fn new() -> Self {
        Self { subj_paths: vec![], clip_paths: vec![] }
    }

    /// Creates a new `Overlay` instance and initializes it with subject and clip paths.
    /// - `subj_shapes`: An array of shapes that together define the subject.
    /// - `clip_shapes`: An array of shapes that together define the clip.
    #[inline]
    pub fn with_shapes(subj_shapes: F64Shapes, clip_shapes: F64Shapes) -> Self {
        let subj_paths = subj_shapes.into_iter().flat_map(|v| v.into_iter()).collect();
        let clip_paths = clip_shapes.into_iter().flat_map(|v| v.into_iter()).collect();
        Self { subj_paths, clip_paths }
    }

    /// Creates a new `Overlay` instance and initializes it with subject and clip paths.
    /// - `subj_paths`: An array of paths that together define the subject.
    /// - `clip_paths`: An array of paths that together define the clip.
    #[inline]
    pub fn with_paths(subj_paths: Vec<F64Path>, clip_paths: Vec<F64Path>) -> Self {
        Self { subj_paths, clip_paths }
    }

    /// Creates a new `Overlay` instance and initializes it with subject and clip path.
    /// - `subj_path`: A path that define the subject.
    /// - `clip_path`: A path that define the clip.
    #[inline]
    pub fn with_path(subj_path: F64Path, clip_path: F64Path) -> Self {
        Self { subj_paths: vec![subj_path], clip_paths: vec![clip_path] }
    }

    /// Adds a single path to the overlay as either subject or clip paths.
    /// - `path`: A reference to a `F64Path` instance to be added.
    /// - `shape_type`: Specifies the role of the added path in the overlay operation, either as `Subject` or `Clip`.
    #[inline]
    pub fn add_path(&mut self, path: F64Path, shape_type: ShapeType) {
        match shape_type {
            ShapeType::Subject => {
                self.subj_paths.push(path);
            }
            ShapeType::Clip => {
                self.clip_paths.push(path);
            }
        }
    }

    /// Adds multiple paths to the overlay as either subject or clip paths.
    /// - `paths`: An array of `F64Path` instances to be added to the overlay.
    /// - `shape_type`: Specifies the role of the added paths in the overlay operation, either as `Subject` or `Clip`.
    #[inline]
    pub fn add_paths(&mut self, paths: Vec<F64Path>, shape_type: ShapeType) {
        let mut mpaths = paths;
        match shape_type {
            ShapeType::Subject => {
                self.subj_paths.append(&mut mpaths);
            }
            ShapeType::Clip => {
                self.clip_paths.append(&mut mpaths);
            }
        }
    }

    /// Convert into `FloatOverlayGraph` from the added paths or shapes using the specified fill rule. This graph is the foundation for executing boolean operations, allowing for the analysis and manipulation of the geometric data. The `OverlayGraph` created by this method represents a preprocessed state of the input shapes, optimized for the application of boolean operations based on the provided fill rule.
    /// - `fill_rule`: Specifies the rule for determining filled areas within the shapes, influencing how the resulting graph represents intersections and unions.
    #[inline(always)]
    pub fn into_graph(self, fill_rule: FillRule) -> F64OverlayGraph {
        self.into_graph_with_solver(fill_rule, Solver::AUTO)
    }

    /// Convert into `FloatOverlayGraph` from the added paths or shapes using the specified fill rule. This graph is the foundation for executing boolean operations, allowing for the analysis and manipulation of the geometric data. The `OverlayGraph` created by this method represents a preprocessed state of the input shapes, optimized for the application of boolean operations based on the provided fill rule.
    /// - `fill_rule`: Specifies the rule for determining filled areas within the shapes, influencing how the resulting graph represents intersections and unions.
    /// - `solver`: Type of solver to use.
    pub fn into_graph_with_solver(self, fill_rule: FillRule, solver: Solver) -> F64OverlayGraph {
        let (overlay, adapter) = self.into_overlay();

        let graph = overlay.into_graph_with_solver(fill_rule, solver);

        F64OverlayGraph::new(graph, adapter)
    }

    /// Convert into int overlay from the added paths or shapes, applying the specified fill rule. This method is particularly useful for development purposes and for creating visualizations in educational demos, where understanding the impact of different rules on the final geometry is crucial.
    pub fn into_overlay(self) -> (Overlay, F64PointAdapter) {
        let subj_rect = F64Rect::with_shape(&self.subj_paths);
        let clip_rect = F64Rect::with_shape(&self.clip_paths);

        let union_rect = F64Rect::with_optional_rects(subj_rect, clip_rect)
            .unwrap_or(F64Rect {
                min_x: -1.0,
                max_x: 1.0,
                min_y: -1.0,
                max_y: 1.0,
            });

        let adapter = F64PointAdapter::new(union_rect);

        let int_subj = self.subj_paths.to_int(&adapter);
        let int_clip = self.clip_paths.to_int(&adapter);

        let overlay = Overlay::with_paths(&int_subj, &int_clip);

        (overlay, adapter)
    }
}

impl Default for F64Overlay {
    fn default() -> Self {
        Self::new()
    }
}
