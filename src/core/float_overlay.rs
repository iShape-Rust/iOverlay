use i_float::adapter::PointAdapter;
use i_float::f64_point::F64Point;
use i_float::f64_rect::F64Rect;
use i_shape::f64::adapter::ShapeToInt;
use i_shape::f64::rect::RectInit;
use i_shape::f64::shape::F64Path;
use crate::core::fill_rule::FillRule;
use crate::core::float_graph::FloatOverlayGraph;
use crate::core::overlay::{Overlay, ShapeType};
use crate::core::solver::Solver;

#[derive(Clone)]
pub struct FloatOverlay {
    subj_paths: Vec<F64Path>,
    clip_paths: Vec<F64Path>,
}

impl FloatOverlay {

    #[inline]
    pub fn new() -> Self {
        Self { subj_paths: vec![], clip_paths: vec![] }
    }

    /// Creates a new `Overlay` instance and initializes it with subject and clip paths.
    /// - `subject_paths`: An array of paths that together define the subject shape.
    /// - `clip_paths`: An array of paths that together define the clip shape.
    #[inline]
    pub fn with_paths(subject_paths: Vec<F64Path>, clip_paths: Vec<F64Path>) -> Self {
        Self { subj_paths: subject_paths, clip_paths }
    }

    /// Adds a single path to the overlay as either subject or clip paths.
    /// - `path`: A reference to a `F64Path` instance to be added.
    /// - `shape_type`: Specifies the role of the added path in the overlay operation, either as `Subject` or `Clip`.
    #[inline]
    pub fn add_path(&mut self, path: &[F64Point], shape_type: ShapeType) {
        match shape_type {
            ShapeType::Subject => {
                self.subj_paths.push(path.to_vec());
            }
            ShapeType::Clip => {
                self.clip_paths.push(path.to_vec());
            }
        }
    }

    /// Adds multiple paths to the overlay as either subject or clip paths.
    /// - `paths`: An array of `F64Path` instances to be added to the overlay.
    /// - `shape_type`: Specifies the role of the added paths in the overlay operation, either as `Subject` or `Clip`.
    #[inline]
    pub fn add_paths(&mut self, paths: &[F64Path], shape_type: ShapeType) {
        match shape_type {
            ShapeType::Subject => {
                self.subj_paths.extend(paths.to_vec());
            }
            ShapeType::Clip => {
                self.clip_paths.extend(paths.to_vec());
            }
        }
    }

    /// Convert into `FloatOverlayGraph` from the added paths or shapes using the specified fill rule. This graph is the foundation for executing boolean operations, allowing for the analysis and manipulation of the geometric data. The `OverlayGraph` created by this method represents a preprocessed state of the input shapes, optimized for the application of boolean operations based on the provided fill rule.
    /// - `fill_rule`: Specifies the rule for determining filled areas within the shapes, influencing how the resulting graph represents intersections and unions.
    #[inline(always)]
    pub fn into_graph(self, fill_rule: FillRule) -> FloatOverlayGraph {
        self.into_graph_with_solver(fill_rule, Solver::AUTO)
    }

    /// Convert into `FloatOverlayGraph` from the added paths or shapes using the specified fill rule. This graph is the foundation for executing boolean operations, allowing for the analysis and manipulation of the geometric data. The `OverlayGraph` created by this method represents a preprocessed state of the input shapes, optimized for the application of boolean operations based on the provided fill rule.
    /// - `fill_rule`: Specifies the rule for determining filled areas within the shapes, influencing how the resulting graph represents intersections and unions.
    /// - `solver`: Type of solver to use.
    pub fn into_graph_with_solver(self, fill_rule: FillRule, solver: Solver) -> FloatOverlayGraph {
        let subj_rect = F64Rect::with_shape(&self.subj_paths);
        let clip_rect = F64Rect::with_shape(&self.clip_paths);

        let union_rect = F64Rect::with_optional_rects(subj_rect, clip_rect)
            .unwrap_or(F64Rect {
                min_x: -1.0,
                max_x: 1.0,
                min_y: -1.0,
                max_y: 1.0,
            });

        let adapter = PointAdapter::new(union_rect);

        let int_subj = self.subj_paths.to_int(&adapter);
        let int_clip = self.clip_paths.to_int(&adapter);

        let overlay = Overlay::with_paths(&int_subj, &int_clip);
        let graph = overlay.into_graph_with_solver(fill_rule, solver);

        FloatOverlayGraph::new(graph, adapter)
    }
}