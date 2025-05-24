//! This module contains functionality to construct and manage overlays, which are used to perform
//! boolean operations (union, intersection, etc.) on polygons. It provides structures and methods to
//! manage subject and clip polygons and convert them into graphs for further operations.

use crate::i_shape::source::resource::ShapeResource;
use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_float::float::rect::FloatRect;
use i_shape::base::data::Shapes;
use i_shape::float::adapter::ShapesToFloat;
use i_shape::float::despike::DeSpikeContour;
use i_shape::float::simple::SimplifyContour;
use crate::core::fill_rule::FillRule;
use crate::core::overlay::{ContourDirection, IntOverlayOptions, Overlay, ShapeType};
use crate::core::overlay_rule::OverlayRule;
use crate::core::solver::Solver;
use crate::float::graph::FloatOverlayGraph;

#[derive(Debug, Clone, Copy)]
pub struct OverlayOptions<T: FloatNumber> {
    /// Preserve collinear points in the input before Boolean operations.
    pub preserve_input_collinear: bool,

    /// Desired direction for output contours (default outer: CCW / hole: CW).
    pub output_direction: ContourDirection,

    /// Preserve collinear points in the output after Boolean operations.
    pub preserve_output_collinear: bool,

    /// Minimum area threshold to include a contour in the result.
    pub min_output_area: T,

    /// If true, the result will be cleaned from precision-related issues
    /// such as duplicate or nearly identical points. Especially useful for `f32` coordinates.
    pub clean_result: bool,
}

/// This struct is essential for describing and uploading the geometry or shapes required to construct an `FloatOverlay`. It prepares the necessary data for boolean operations.
pub struct FloatOverlay<P: FloatPointCompatible<T>, T: FloatNumber> {
    pub(super) overlay: Overlay,
    pub(super) clean_result: bool,
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
        Self::new_custom(adapter, Default::default(), Default::default(), capacity)
    }

    /// Constructs a new `FloatOverlay`, a builder for overlaying geometric shapes
    /// by converting float-based geometry to integer space, using a pre-configured adapter.
    ///
    /// - `adapter`: A `FloatPointAdapter` instance responsible for coordinate conversion between
    ///   float and integer values, ensuring accuracy during geometric transformations.
    /// - `options`: Adjust custom behavior.
    /// - `solver`: Type of solver to use.
    /// - `capacity`: Initial capacity for storing segments, ideally matching the total number of
    ///   segments for efficient memory allocation.
    #[inline]
    pub fn new_custom(adapter: FloatPointAdapter<P, T>, options: OverlayOptions<T>, solver: Solver, capacity: usize) -> Self {
        let clean_result = options.clean_result;
        let overlay = Overlay::new_custom(capacity, options.int_with_adapter(&adapter), solver);
        Self { overlay, clean_result, adapter }
    }

    /// Constructs a new `FloatOverlay`, a builder for overlaying geometric shapes
    /// by converting float-based geometry to integer space, using a pre-configured adapter.
    /// - `options`: Adjust custom behavior.
    /// - `solver`: Type of solver to use.
    /// - `capacity`: Initial capacity for storing segments, ideally matching the total number of
    ///   segments for efficient memory allocation.
    #[inline]
    pub fn new_empty(options: OverlayOptions<T>, solver: Solver, capacity: usize) -> Self {
        let clean_result = options.clean_result;
        let adapter = FloatPointAdapter::new(FloatRect::zero());
        let overlay = Overlay::new_custom(capacity, options.int_default(), solver);
        Self { overlay, clean_result, adapter }
    }

    /// Creates a new `FloatOverlay` instance and initializes it with subject and clip shapes.
    /// - `subj`: A `ShapeResource` that define the subject.
    /// - `clip`: A `ShapeResource` that define the clip.
    ///   `ShapeResource` can be one of the following:
    ///     - `Contour`: A contour representing a closed path. This path is interpreted as closed, so it doesn’t require the start and endpoint to be the same for processing.
    ///     - `Contours`: A collection of contours, each representing a closed path.
    ///     - `Shapes`: A collection of shapes, where each shape may consist of multiple contours.
    pub fn with_subj_and_clip<R0, R1>(subj: &R0, clip: &R1) -> Self
    where
        R0: ShapeResource<P, T> +?Sized,
        R1: ShapeResource<P, T> +?Sized,
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
    /// - `subj`: A `ShapeResource` that define the subject.
    /// - `clip`: A `ShapeResource` that define the clip.
    ///   `ShapeResource` can be one of the following:
    ///     - `Contour`: A contour representing a closed path. This path is interpreted as closed, so it doesn’t require the start and endpoint to be the same for processing.
    ///     - `Contours`: A collection of contours, each representing a closed path.
    ///     - `Shapes`: A collection of shapes, where each shape may consist of multiple contours.
    /// - `options`: Adjust custom behavior.
    /// - `solver`: Type of solver to use.
    pub fn with_subj_and_clip_custom<R0, R1>(subj: &R0, clip: &R1, options: OverlayOptions<T>, solver: Solver) -> Self
    where
        R0: ShapeResource<P, T> +?Sized,
        R1: ShapeResource<P, T> +?Sized,
        P: FloatPointCompatible<T>,
        T: FloatNumber,
    {
        let iter = subj.iter_paths().chain(clip.iter_paths()).flatten();
        let adapter = FloatPointAdapter::with_iter(iter);
        let subj_capacity = subj.iter_paths().fold(0, |s, c| s + c.len());
        let clip_capacity = clip.iter_paths().fold(0, |s, c| s + c.len());

        Self::new_custom(adapter, options, solver, subj_capacity + clip_capacity)
            .unsafe_add_source(subj, ShapeType::Subject)
            .unsafe_add_source(clip, ShapeType::Clip)
    }

    /// Creates a new `FloatOverlay` instance and initializes it with subject and clip shapes.
    /// - `subj`: A `ShapeResource` that define the subject.
    ///   `ShapeResource` can be one of the following:
    ///     - `Contour`: A contour representing a closed path. This path is interpreted as closed, so it doesn’t require the start and endpoint to be the same for processing.
    ///     - `Contours`: A collection of contours, each representing a closed path.
    ///     - `Shapes`: A collection of shapes, where each shape may consist of multiple contours.
    pub fn with_subj<R>(subj: &R) -> Self
    where
        R: ShapeResource<P, T> +?Sized,
        P: FloatPointCompatible<T>,
        T: FloatNumber,
    {
        let iter = subj.iter_paths().flatten();
        let adapter = FloatPointAdapter::with_iter(iter);
        let subj_capacity = subj.iter_paths().fold(0, |s, c| s + c.len());

        Self::with_adapter(adapter, subj_capacity)
            .unsafe_add_source(subj, ShapeType::Subject)
    }

    /// Creates a new `FloatOverlay` instance and initializes it with subject.
    /// - `subj`: A `ShapeResource` that define the subject.
    ///   `ShapeResource` can be one of the following:
    ///     - `Contour`: A contour representing a closed path. This path is interpreted as closed, so it doesn’t require the start and endpoint to be the same for processing.
    ///     - `Contours`: A collection of contours, each representing a closed path.
    ///     - `Shapes`: A collection of shapes, where each shape may consist of multiple contours.
    /// - `options`: Adjust custom behavior.
    /// - `solver`: Type of solver to use.
    pub fn with_subj_custom<R>(subj: &R, options: OverlayOptions<T>, solver: Solver) -> Self
    where
        R: ShapeResource<P, T> +?Sized,
        P: FloatPointCompatible<T>,
        T: FloatNumber,
    {
        let iter = subj.iter_paths().flatten();
        let adapter = FloatPointAdapter::with_iter(iter);
        let subj_capacity = subj.iter_paths().fold(0, |s, c| s + c.len());

        Self::new_custom(adapter, options, solver, subj_capacity)
            .unsafe_add_source(subj, ShapeType::Subject)
    }

    /// Adds a shapes to the overlay.
    /// - `resource`: A `ShapeResource` that define subject or clip.
    ///   `ShapeResource` can be one of the following:
    ///     - `Contour`: A contour representing a closed path. This path is interpreted as closed, so it doesn’t require the start and endpoint to be the same for processing.
    ///     - `Contours`: A collection of contours, each representing a closed path.
    ///     - `Shapes`: A collection of shapes, where each shape may consist of multiple contours.
    /// - `shape_type`: Specifies the role of the added paths in the overlay operation, either as `Subject` or `Clip`.
    #[inline]
    pub fn unsafe_add_source<R: ShapeResource<P, T> +?Sized>(mut self, resource: &R, shape_type: ShapeType) -> Self {
        self.add_source(resource, shape_type);
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

    #[inline]
    pub fn clear(&mut self) {
        self.overlay.clear();
    }

    #[inline]
    fn add_source<R: ShapeResource<P, T> +?Sized>(&mut self, resource: &R, shape_type: ShapeType) {
        for contour in resource.iter_paths() {
            self.overlay.add_path_iter(contour.iter().map(|p| self.adapter.float_to_int(p)), shape_type);
        }
    }

    /// Reinit `FloatOverlay` instance and initializes it with subject and clip shapes.
    /// - `subj`: A `ShapeResource` that define the subject.
    /// - `clip`: A `ShapeResource` that define the clip.
    ///   `ShapeResource` can be one of the following:
    ///     - `Contour`: A contour representing a closed path. This path is interpreted as closed, so it doesn’t require the start and endpoint to be the same for processing.
    ///     - `Contours`: A collection of contours, each representing a closed path.
    ///     - `Shapes`: A collection of shapes, where each shape may consist of multiple contours.
    pub fn reinit_with_subj_and_clip<R0, R1>(&mut self, subj: &R0, clip: &R1)
    where
        R0: ShapeResource<P, T> +?Sized,
        R1: ShapeResource<P, T> +?Sized,
        P: FloatPointCompatible<T>,
        T: FloatNumber,
    {
        self.clear();

        let iter = subj.iter_paths().chain(clip.iter_paths()).flatten();
        self.adapter = FloatPointAdapter::with_iter(iter);
        self.add_source(subj, ShapeType::Subject);
        self.add_source(clip, ShapeType::Clip);
    }

    /// Reinit `FloatOverlay` instance and initializes it with subject
    /// - `subj`: A `ShapeResource` that define the subject.
    ///   `ShapeResource` can be one of the following:
    ///     - `Contour`: A contour representing a closed path. This path is interpreted as closed, so it doesn’t require the start and endpoint to be the same for processing.
    ///     - `Contours`: A collection of contours, each representing a closed path.
    ///     - `Shapes`: A collection of shapes, where each shape may consist of multiple contours.
    pub fn reinit_with_subj<R>(&mut self, subj: &R)
    where
        R: ShapeResource<P, T> +?Sized,
        P: FloatPointCompatible<T>,
        T: FloatNumber,
    {
        self.clear();

        let iter = subj.iter_paths().flatten();
        self.adapter = FloatPointAdapter::with_iter(iter);
        self.add_source(subj, ShapeType::Subject);
    }

    /// Convert into `FloatOverlayGraph` from the added paths or shapes using the specified build rule. This graph is the foundation for executing boolean operations, allowing for the analysis and manipulation of the geometric data. The `OverlayGraph` created by this method represents a preprocessed state of the input shapes, optimized for the application of boolean operations based on the provided build rule.
    /// - `fill_rule`: Specifies the rule for determining filled areas within the shapes, influencing how the resulting graph represents intersections and unions.
    #[inline]
    pub fn build_graph_view(&mut self, fill_rule: FillRule) -> Option<FloatOverlayGraph<P, T>> {
        let graph = self.overlay.build_graph_view(fill_rule)?;
        Some(FloatOverlayGraph::new(graph, self.adapter.clone(), self.clean_result))
    }

    /// Executes a single Boolean operation on the current geometry using the specified overlay and build rules.
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
    /// Note: Outer boundary paths have a counterclockwise order, and holes have a clockwise order.
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
    /// let mut overlay = FloatOverlay::with_subj_and_clip(&left_rect, &right_rect);
    ///
    /// let result_shapes = overlay.overlay(OverlayRule::Union, FillRule::EvenOdd);
    /// ```
    ///
    /// This method is particularly useful in scenarios where the geometry only needs one overlay operation
    /// without subsequent modifications. By excluding unnecessary graph structures, it optimizes performance,
    /// particularly for complex or resource-intensive geometries.
    #[inline]
    pub fn overlay(&mut self, overlay_rule: OverlayRule, fill_rule: FillRule) -> Shapes<P> {
        let preserve_output_collinear = self.overlay.options.preserve_output_collinear;
        let shapes = self.overlay.overlay(overlay_rule, fill_rule);
        let mut float = shapes.to_float(&self.adapter);

        if self.clean_result {
            if preserve_output_collinear {
                float.despike_contour(&self.adapter);
            } else {
                float.simplify_contour(&self.adapter);
            }
        }

        float
    }
}

impl<T: FloatNumber> Default for OverlayOptions<T> {
    fn default() -> Self {
        // f32 precision is not enough to cover i32
        let clean_result = T::bit_width() <= 32;
        Self {
            preserve_input_collinear: false,
            output_direction: ContourDirection::CounterClockwise,
            preserve_output_collinear: false,
            min_output_area: T::from_float(0.0),
            clean_result,
        }
    }
}

impl<T: FloatNumber> OverlayOptions<T> {
    pub(crate) fn int_with_adapter<P: FloatPointCompatible<T>>(&self, adapter: &FloatPointAdapter<P, T>) -> IntOverlayOptions {
        IntOverlayOptions {
            preserve_input_collinear: self.preserve_input_collinear,
            output_direction: self.output_direction,
            preserve_output_collinear: self.preserve_output_collinear,
            min_output_area: adapter.sqr_float_to_int(self.min_output_area),
        }
    }

    pub(crate) fn int_default(&self) -> IntOverlayOptions {
        IntOverlayOptions {
            preserve_input_collinear: self.preserve_input_collinear,
            output_direction: self.output_direction,
            preserve_output_collinear: self.preserve_output_collinear,
            min_output_area: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec;
use crate::core::fill_rule::FillRule;
    use crate::core::overlay_rule::OverlayRule;
    use crate::float::overlay::FloatOverlay;

    #[test]
    fn test_contour_fixed() {
        let left_rect = [[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]];
        let right_rect = [[1.0, 0.0], [1.0, 1.0], [2.0, 1.0], [2.0, 0.0]];

        let shapes = FloatOverlay::with_subj_and_clip(&left_rect, &right_rect)
            .overlay(
                OverlayRule::Union,
                FillRule::EvenOdd,
            );

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].len(), 1);
        assert_eq!(shapes[0][0].len(), 4);
    }

    #[test]
    fn test_contour_vec() {
        let left_rect = vec![[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]];
        let right_rect = vec![[1.0, 0.0], [1.0, 1.0], [2.0, 1.0], [2.0, 0.0]];


        let shapes = FloatOverlay::with_subj_and_clip(&left_rect, &right_rect)
            .overlay(
                OverlayRule::Union,
                FillRule::EvenOdd
            );

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].len(), 1);
        assert_eq!(shapes[0][0].len(), 4);
    }

    #[test]
    fn test_contour_slice() {
        let left_rect = [[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]];
        let right_rect = [[1.0, 0.0], [1.0, 1.0], [2.0, 1.0], [2.0, 0.0]];

        let shapes = FloatOverlay::with_subj_and_clip(left_rect.as_slice(), right_rect.as_slice())
            .overlay(
                OverlayRule::Union,
                FillRule::EvenOdd
            );

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
            .overlay(
                OverlayRule::Union,
                FillRule::EvenOdd
            );

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
            .overlay(
                OverlayRule::Union,
                FillRule::EvenOdd
            );

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
            .overlay(
                OverlayRule::Union,
                FillRule::EvenOdd
            );

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
            .overlay(
                OverlayRule::Union,
                FillRule::EvenOdd
            );

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

        let mut overlay = FloatOverlay::with_subj_and_clip(&subj, &clip);

        let result = overlay.overlay(OverlayRule::Intersect, FillRule::EvenOdd );

        assert_eq!(result.len(), 16);
    }
}