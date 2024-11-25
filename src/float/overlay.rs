//! This module contains functionality to construct and manage overlays, which are used to perform
//! boolean operations (union, intersection, etc.) on polygons. It provides structures and methods to
//! manage subject and clip polygons and convert them into graphs for further operations.

use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_shape::base::data::Shapes;
use i_shape::float::adapter::ShapesToFloat;
use i_shape::float::simple::SimplifyContour;
use crate::core::fill_rule::FillRule;
use crate::core::overlay::{Overlay, ShapeType};
use crate::core::overlay_rule::OverlayRule;
use crate::core::solver::Solver;
use crate::float::filter::ContourFilter;
use crate::float::graph::FloatOverlayGraph;
use crate::float::source::resource::OverlayResource;

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

    /// Creates a new `FloatOverlay` instance and initializes it with subject and clip shapes.
    /// - `subj`: A `OverlayResource` that define the subject.
    /// - `clip`: A `OverlayResource` that define the clip.
    ///   `OverlayResource` can be one of the following:
    ///     - `Contour`: A contour representing a closed path. This path is interpreted as closed, so it doesn’t require the start and endpoint to be the same for processing.
    ///     - `Contours`: A collection of contours, each representing a closed path.
    ///     - `Shapes`: A collection of shapes, where each shape may consist of multiple contours.
    pub fn with_subj_and_clip<R0, R1>(subj: &R0, clip: &R1) -> Self
    where
        R0: OverlayResource<P, T> +?Sized,
        R1: OverlayResource<P, T> +?Sized,
        P: FloatPointCompatible<T>,
        T: FloatNumber,
    {
        let iter = subj.iter_paths().chain(clip.iter_paths()).flatten();
        let adapter = FloatPointAdapter::with_iter(iter);
        let subj_capacity = subj.iter_paths().fold(0, |s, c| s + c.len());
        let clip_capacity = clip.iter_paths().fold(0, |s, c| s + c.len());

        Self::with_adapter(adapter, subj_capacity + clip_capacity)
            .unsafe_add_source(subj, ShapeType::Subject)
            .unsafe_add_source(clip, ShapeType::Clip)
    }

    /// Creates a new `FloatOverlay` instance and initializes it with subject and clip shapes.
    /// - `subj`: A `OverlayResource` that define the subject.
    ///   `OverlayResource` can be one of the following:
    ///     - `Contour`: A contour representing a closed path. This path is interpreted as closed, so it doesn’t require the start and endpoint to be the same for processing.
    ///     - `Contours`: A collection of contours, each representing a closed path.
    ///     - `Shapes`: A collection of shapes, where each shape may consist of multiple contours.
    pub fn with_subj<R>(subj: &R) -> Self
    where
        R: OverlayResource<P, T> +?Sized,
        P: FloatPointCompatible<T>,
        T: FloatNumber,
    {
        let iter = subj.iter_paths().flatten();
        let adapter = FloatPointAdapter::with_iter(iter);
        let subj_capacity = subj.iter_paths().fold(0, |s, c| s + c.len());

        Self::with_adapter(adapter, subj_capacity)
            .unsafe_add_source(subj, ShapeType::Subject)
    }

    /// Adds a shapes to the overlay.
    /// - `resource`: A `OverlayResource` that define subject or clip.
    ///   `OverlayResource` can be one of the following:
    ///     - `Contour`: A contour representing a closed path. This path is interpreted as closed, so it doesn’t require the start and endpoint to be the same for processing.
    ///     - `Contours`: A collection of contours, each representing a closed path.
    ///     - `Shapes`: A collection of shapes, where each shape may consist of multiple contours.
    /// - `shape_type`: Specifies the role of the added paths in the overlay operation, either as `Subject` or `Clip`.
    #[inline]
    pub fn unsafe_add_source<R: OverlayResource<P, T> +?Sized>(mut self, resource: &R, shape_type: ShapeType) -> Self {
        for contour in resource.iter_paths() {
            self = self.unsafe_add_contour(contour, shape_type);
        }
        self
    }

    /// Adds a closed path to the overlay.
    /// - `contour`: A contour representing a closed path. This path is interpreted as closed, so it doesn’t require the start and endpoint to be the same for processing.
    /// - `shape_type`: Specifies the role of the added path in the overlay operation, either as `Subject` or `Clip`.
    /// - **Safety**: Marked `unsafe` because it assumes the path is fully contained within the bounding box.
    #[inline]
    pub fn unsafe_add_contour(mut self, contour: &[P], shape_type: ShapeType) -> Self {
        self.overlay.add_path_iter(contour.iter().map(|p| self.adapter.float_to_int(p)), shape_type);
        self
    }

    /// Convert into `FloatOverlayGraph` from the added paths or shapes using the specified fill rule. This graph is the foundation for executing boolean operations, allowing for the analysis and manipulation of the geometric data. The `OverlayGraph` created by this method represents a preprocessed state of the input shapes, optimized for the application of boolean operations based on the provided fill rule.
    /// - `fill_rule`: Specifies the rule for determining filled areas within the shapes, influencing how the resulting graph represents intersections and unions.
    #[inline]
    pub fn into_graph(self, fill_rule: FillRule) -> FloatOverlayGraph<P, T> {
        self.into_graph_with_solver(fill_rule, Solver::AUTO)
    }

    /// Convert into `FloatOverlayGraph` from the added paths or shapes using the specified fill rule. This graph is the foundation for executing boolean operations, allowing for the analysis and manipulation of the geometric data. The `OverlayGraph` created by this method represents a preprocessed state of the input shapes, optimized for the application of boolean operations based on the provided fill rule.
    /// - `fill_rule`: Fill rule to determine filled areas (non-zero, even-odd, positive, negative).
    /// - `solver`: Type of solver to use.
    #[inline]
    pub fn into_graph_with_solver(self, fill_rule: FillRule, solver: Solver) -> FloatOverlayGraph<P, T> {
        let graph = self.overlay.into_graph_with_solver(fill_rule, solver);
        FloatOverlayGraph::new(graph, self.adapter)
    }

    /// Executes a single Boolean operation on the current geometry using the specified overlay and fill rules.
    /// This method provides a streamlined approach for performing a Boolean operation without generating
    /// an entire `FloatOverlayGraph`. Ideal for cases where only one Boolean operation is needed, `overlay`
    /// saves on computational resources by building only the necessary links, optimizing CPU usage by 0-20%
    /// compared to a full graph-based approach.
    ///
    /// ### Parameters:
    /// - `overlay_rule`: The boolean operation rule to apply, determining how shapes are combined or subtracted.
    /// - `fill_rule`: Fill rule to determine filled areas (non-zero, even-odd, positive, negative).
    /// - Returns: A vector of `Shapes<P>` that meet the specified area criteria, representing the cleaned-up geometric result.
    /// # Shape Representation
    /// The output is a `Shapes<P>`, where:
    /// - The outer `Vec<Shape<P>>` represents a set of shapes.
    /// - Each shape `Vec<Contour<P>>` represents a collection of paths, where the first path is the outer boundary, and all subsequent paths are holes in this boundary.
    /// - Each path `Vec<P>` is a sequence of points, forming a closed path.
    ///
    /// Note: Outer boundary paths have a clockwise order, and holes have a counterclockwise order.
    /// ### Usage:
    /// This function is suitable when a single, optimized Boolean operation is required on the provided
    /// geometry. For example:
    ///
    /// ```rust
    /// use i_float::float::compatible::FloatPointCompatible;
    /// use i_overlay::float::overlay::FloatOverlay;
    /// use i_overlay::core::fill_rule::FillRule;
    /// use i_overlay::core::overlay_rule::OverlayRule;
    ///
    /// let left_rect = vec![[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]];
    /// let right_rect = vec![[1.0, 0.0], [1.0, 1.0], [2.0, 1.0], [2.0, 0.0]];
    /// let overlay = FloatOverlay::with_subj_and_clip(&left_rect, &right_rect);
    ///
    /// let result_shapes = overlay.overlay(OverlayRule::Union, FillRule::EvenOdd);
    /// ```
    ///
    /// This method is particularly useful in scenarios where the geometry only needs one overlay operation
    /// without subsequent modifications. By excluding unnecessary graph structures, it optimizes performance,
    /// particularly for complex or resource-intensive geometries.
    #[inline]
    pub fn overlay(self, overlay_rule: OverlayRule, fill_rule: FillRule) -> Shapes<P> {
        self.overlay_with_filter_and_solver(overlay_rule, fill_rule, Default::default(), Default::default())
    }

    /// Executes a single Boolean operation on the current geometry using the specified overlay and fill rules.
    /// This method provides a streamlined approach for performing a Boolean operation without generating
    /// an entire `FloatOverlayGraph`. Ideal for cases where only one Boolean operation is needed, `overlay`
    /// saves on computational resources by building only the necessary links, optimizing CPU usage by 0-20%
    /// compared to a full graph-based approach.
    ///
    /// ### Parameters:
    /// - `overlay_rule`: The boolean operation rule to apply, determining how shapes are combined or subtracted.
    /// - `fill_rule`: Fill rule to determine filled areas (non-zero, even-odd, positive, negative).
    /// - `filter`: `ContourFilter<T>` for optional contour filtering and simplification:
    ///     - `min_area`: Only retain contours with an area larger than this.
    ///     - `simplify`: Simplifies contours and removes degenerate edges if `true`.
    /// - `solver`: Type of solver to use.
    /// - Returns: A vector of `Shapes<P>` that meet the specified area criteria, representing the cleaned-up geometric result.
    /// # Shape Representation
    /// The output is a `Shapes<P>`, where:
    /// - The outer `Vec<Shape<P>>` represents a set of shapes.
    /// - Each shape `Vec<Contour<P>>` represents a collection of paths, where the first path is the outer boundary, and all subsequent paths are holes in this boundary.
    /// - Each path `Vec<P>` is a sequence of points, forming a closed path.
    ///
    /// Note: Outer boundary paths have a clockwise order, and holes have a counterclockwise order.
    /// This method is particularly useful in scenarios where the geometry only needs one overlay operation
    /// without subsequent modifications. By excluding unnecessary graph structures, it optimizes performance,
    /// particularly for complex or resource-intensive geometries.
    #[inline]
    pub fn overlay_with_filter_and_solver(self, overlay_rule: OverlayRule, fill_rule: FillRule, filter: ContourFilter<T>, solver: Solver) -> Shapes<P> {
        let area = self.adapter.sqr_float_to_int(filter.min_area);
        let shapes = self.overlay.overlay_with_min_area_and_solver(overlay_rule, fill_rule, area, solver);
        let mut float = shapes.to_float(&self.adapter);

        if filter.simplify {
            float.simplify_contour(&self.adapter);
        }

        float
    }
}

#[cfg(test)]
mod tests {
    use crate::core::fill_rule::FillRule;
    use crate::core::overlay_rule::OverlayRule;
    use crate::float::overlay::FloatOverlay;

    #[test]
    fn test_contour_fixed() {
        let left_rect = [[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]];
        let right_rect = [[1.0, 0.0], [1.0, 1.0], [2.0, 1.0], [2.0, 0.0]];

        let shapes = FloatOverlay::with_subj_and_clip(&left_rect, &right_rect)
            .overlay(OverlayRule::Union, FillRule::EvenOdd);

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].len(), 1);
        assert_eq!(shapes[0][0].len(), 4);
    }

    #[test]
    fn test_contour_vec() {
        let left_rect = vec![[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]];
        let right_rect = vec![[1.0, 0.0], [1.0, 1.0], [2.0, 1.0], [2.0, 0.0]];
        let shapes = FloatOverlay::with_subj_and_clip(&left_rect, &right_rect)
            .overlay(OverlayRule::Union, FillRule::EvenOdd);

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].len(), 1);
        assert_eq!(shapes[0][0].len(), 4);
    }

    #[test]
    fn test_contour_slice() {
        let left_rect = [[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]];
        let right_rect = [[1.0, 0.0], [1.0, 1.0], [2.0, 1.0], [2.0, 0.0]];

        let shapes = FloatOverlay::with_subj_and_clip(left_rect.as_slice(), right_rect.as_slice())
            .overlay(OverlayRule::Union, FillRule::EvenOdd);

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].len(), 1);
        assert_eq!(shapes[0][0].len(), 4);
    }

    #[test]
    fn test_contours_vec() {
        let rects = vec![
            vec![
                [0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]
            ],
            vec![
                [0.0, 1.0], [0.0, 2.0], [1.0, 2.0], [1.0, 1.0]
            ],
            vec![
                [1.0, 1.0], [1.0, 2.0], [2.0, 2.0], [2.0, 1.0]
            ]
        ];
        let right_bottom_rect = vec![[1.0, 0.0], [1.0, 1.0], [2.0, 1.0], [2.0, 0.0]];

        let shapes = FloatOverlay::with_subj_and_clip(&rects.as_slice(), &right_bottom_rect)
            .overlay(OverlayRule::Union, FillRule::EvenOdd);

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].len(), 1);
        assert_eq!(shapes[0][0].len(), 4);
    }

    #[test]
    fn test_contours_slice() {
        let rects = vec![
            vec![
                [0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]
            ],
            vec![
                [0.0, 1.0], [0.0, 2.0], [1.0, 2.0], [1.0, 1.0]
            ],
            vec![
                [1.0, 1.0], [1.0, 2.0], [2.0, 2.0], [2.0, 1.0]
            ]
        ];
        let right_bottom_rect = vec![[1.0, 0.0], [1.0, 1.0], [2.0, 1.0], [2.0, 0.0]];

        let shapes = FloatOverlay::with_subj_and_clip(rects.as_slice(), right_bottom_rect.as_slice())
            .overlay(OverlayRule::Union, FillRule::EvenOdd);

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].len(), 1);
        assert_eq!(shapes[0][0].len(), 4);
    }

    #[test]
    fn test_shapes() {
        let shapes = vec![
            vec![
                vec![
                    [0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]
                ],
                vec![
                    [0.0, 1.0], [0.0, 2.0], [1.0, 2.0], [1.0, 1.0]
                ],
                vec![
                    [1.0, 1.0], [1.0, 2.0], [2.0, 2.0], [2.0, 1.0]
                ]
            ]
        ];
        let right_bottom_rect = vec![[1.0, 0.0], [1.0, 1.0], [2.0, 1.0], [2.0, 0.0]];


        let shapes = FloatOverlay::with_subj_and_clip(&shapes, &right_bottom_rect)
            .overlay(OverlayRule::Union, FillRule::EvenOdd);

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].len(), 1);
        assert_eq!(shapes[0][0].len(), 4);
    }

    #[test]
    fn test_different_resource() {
        let res_0 = vec![
            vec![
                vec![
                    [0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]
                ],
                vec![
                    [0.0, 1.0], [0.0, 2.0], [1.0, 2.0], [1.0, 1.0]
                ],
                vec![
                    [1.0, 1.0], [1.0, 2.0], [2.0, 2.0], [2.0, 1.0]
                ]
            ]
        ];

        let res_1 = [[1.0, 0.0], [1.0, 1.0], [2.0, 1.0], [2.0, 0.0]];


        let shapes = FloatOverlay::with_subj_and_clip(&res_0, &res_1.as_slice())
            .overlay(OverlayRule::Union, FillRule::EvenOdd);

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].len(), 1);
        assert_eq!(shapes[0][0].len(), 4);
    }

    #[test]
    fn test_grid() {
        let subj = vec![
            vec![vec![[175.0, 475.0], [175.0, 375.0], [275.0, 375.0], [275.0, 475.0]]],
            vec![vec![[175.0, 325.0], [175.0, 225.0], [275.0, 225.0], [275.0, 325.0]]],
            vec![vec![[175.0, 175.0], [175.0, 75.0], [275.0, 75.0], [275.0, 175.0]]],
            vec![vec![[325.0, 475.0], [325.0, 375.0], [425.0, 375.0], [425.0, 475.0]]],
            vec![vec![[325.0, 325.0], [325.0, 225.0], [425.0, 225.0], [425.0, 325.0]]],
            vec![vec![[325.0, 175.0], [325.0, 75.0], [425.0, 75.0], [425.0, 175.0]]],
            vec![vec![[475.0, 475.0], [475.0, 375.0], [575.0, 375.0], [575.0, 475.0]]],
            vec![vec![[475.0, 325.0], [475.0, 225.0], [575.0, 225.0], [575.0, 325.0]]],
            vec![vec![[475.0, 175.0], [475.0, 75.0], [575.0, 75.0], [575.0, 175.0]]]];

        let clip = vec![
            vec![vec![[250.0, 400.0], [250.0, 300.0], [350.0, 300.0], [350.0, 400.0]]],
            vec![vec![[250.0, 250.0], [250.0, 150.0], [350.0, 150.0], [350.0, 250.0]]],
            vec![vec![[400.0, 400.0], [400.0, 300.0], [500.0, 300.0], [500.0, 400.0]]],
            vec![vec![[400.0, 250.0], [400.0, 150.0], [500.0, 150.0], [500.0, 250.0]]]
        ];

        let overlay = FloatOverlay::with_subj_and_clip(&subj, &clip);

        let result = overlay.overlay(OverlayRule::Intersect, FillRule::EvenOdd );

        assert_eq!(result.len(), 16);
    }
}