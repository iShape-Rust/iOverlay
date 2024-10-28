//! This module contains functionality to construct and manage overlays, which are used to perform
//! boolean operations (union, intersection, etc.) on polygons. It provides structures and methods to
//! manage subject and clip polygons and convert them into graphs for further operations.

use i_float::adapter::FloatPointAdapter;
use i_float::float::Float;
use i_float::float_point::FloatPointCompatible;
use crate::core::fill_rule::FillRule;
use crate::core::overlay::Overlay;
use crate::core::solver::Solver;
use crate::float::graph::FloatOverlayGraph;

/// This struct is essential for describing and uploading the geometry or shapes required to construct an `F32OverlayGraph`. It prepares the necessary data for boolean operations.
#[derive(Clone)]
pub struct FloatOverlay<T: Float> {
    pub(super) overlay: Overlay,
    pub(super) adapter: FloatPointAdapter<T>,
}

impl<T: Float> FloatOverlay<T> {

    #[inline]
    pub fn new(adapter: FloatPointAdapter<T>, capacity: usize) -> Self {
        Self { overlay: Overlay::new(capacity), adapter }
    }

    /// Creates a new `Overlay` instance and initializes it with subject and clip paths.
    /// - `subj_shapes`: An array of shapes that together define the subject.
    /// - `clip_shapes`: An array of shapes that together define the clip.
    pub fn with_shapes<P: FloatPointCompatible<T>>(subj_shapes: Vec<Vec<Vec<P>>>, clip_shapes: Vec<Vec<Vec<P>>>) -> Self {
        let subj_iter = subj_shapes.iter().flatten().flatten().map(|p|p.to_float_point());
        let clip_iter = clip_shapes.iter().flatten().flatten().map(|p|p.to_float_point());
        let iter = subj_iter.chain(clip_iter);
        let adapter = FloatPointAdapter::with_iter(iter);

        let subj_count: usize = subj_shapes.iter().flatten().map(|path| path.len()).sum();
        let clip_count: usize = clip_shapes.iter().flatten().map(|path| path.len()).sum();
        let capacity = subj_count + clip_count;
        let mut overlay = Overlay::new(capacity);

        Self { adapter, overlay }
    }

    /// Creates a new `Overlay` instance and initializes it with subject and clip paths.
    /// - `subj_paths`: An array of paths that together define the subject.
    /// - `clip_paths`: An array of paths that together define the clip.
    pub fn with_paths<P: FloatPointCompatible<T>>(subj_paths: Vec<Vec<P>>, clip_paths: Vec<Vec<P>>) -> Self {
        let subj_iter = subj_paths.iter().flatten().map(|p|p.to_float_point());
        let clip_iter = clip_paths.iter().flatten().map(|p|p.to_float_point());
        let iter = subj_iter.chain(clip_iter);
        let adapter = FloatPointAdapter::with_iter(iter);

        let subj_count: usize = subj_paths.iter().map(|path| path.len()).sum();
        let clip_count: usize = clip_paths.iter().map(|path| path.len()).sum();
        let capacity = subj_count + clip_count;
        let mut overlay = Overlay::new(capacity);

        Self { adapter, overlay }
    }

    /// Creates a new `Overlay` instance and initializes it with subject and clip path.
    /// - `subj_path`: A path that define the subject.
    /// - `clip_path`: A path that define the clip.
    pub fn with_path<P: FloatPointCompatible<T>>(subj_path: Vec<P>, clip_path: Vec<P>) -> Self {
        let iter = subj_path.iter().chain(clip_path.iter()).map(|p|p.to_float_point());
        let adapter = FloatPointAdapter::with_iter(iter);

        let mut overlay = Overlay::new(subj_path.len() + clip_path.len());

        Self { adapter, overlay }
    }

    /// Convert into `FloatOverlayGraph` from the added paths or shapes using the specified fill rule. This graph is the foundation for executing boolean operations, allowing for the analysis and manipulation of the geometric data. The `OverlayGraph` created by this method represents a preprocessed state of the input shapes, optimized for the application of boolean operations based on the provided fill rule.
    /// - `fill_rule`: Specifies the rule for determining filled areas within the shapes, influencing how the resulting graph represents intersections and unions.
    #[inline(always)]
    pub fn into_graph(self, fill_rule: FillRule) -> FloatOverlayGraph<T> {
        self.into_graph_with_solver(fill_rule, Solver::AUTO)
    }

    /// Convert into `FloatOverlayGraph` from the added paths or shapes using the specified fill rule. This graph is the foundation for executing boolean operations, allowing for the analysis and manipulation of the geometric data. The `OverlayGraph` created by this method represents a preprocessed state of the input shapes, optimized for the application of boolean operations based on the provided fill rule.
    /// - `fill_rule`: Specifies the rule for determining filled areas within the shapes, influencing how the resulting graph represents intersections and unions.
    /// - `solver`: Type of solver to use.
    pub fn into_graph_with_solver(self, fill_rule: FillRule, solver: Solver) -> FloatOverlayGraph<T> {
        let graph = self.overlay.into_graph_with_solver(fill_rule, solver);
        FloatOverlayGraph::new(graph, self.adapter)
    }
}
