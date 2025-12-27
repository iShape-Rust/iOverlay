use crate::core::fill_rule::FillRule;
use crate::core::overlay::ShapeType;
use crate::core::overlay_rule::OverlayRule;
use crate::core::solver::Solver;
use crate::float::overlay::{FloatOverlay, OverlayOptions};
use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_shape::base::data::Shapes;
use i_shape::source::resource::ShapeResource;

#[derive(Debug, Clone, Copy)]
pub enum FixedScaleOverlayError {
    /// Requested scale is larger than the safe adapter scale for the input bounds.
    ScaleTooLarge,
    /// Requested scale is zero or negative.
    ScaleNonPositive,
    /// Requested scale is NaN or infinite.
    ScaleNotFinite,
}

/// Trait `FixedScaleFloatOverlay` provides methods for overlay operations between various geometric entities.
/// This trait supports boolean operations on contours, shapes, and collections of shapes, using customizable overlay and build rules.
///
/// The `scale` parameter defines the float-to-integer conversion:
/// `x_int = (x_float - offset_x) * scale`.
/// Larger `scale` gives higher precision but must fit within the safe integer bounds.
pub trait FixedScaleFloatOverlay<R0, R1, P, T>
where
    R0: ShapeResource<P, T>,
    R1: ShapeResource<P, T>,
    P: FloatPointCompatible<T>,
    T: FloatNumber,
{
    /// General overlay method that takes an `ShapeResource` to determine the input type.
    ///
    /// - `resource`: A `ShapeResource` specifying the type of geometric entity to overlay with.
    ///   It can be one of the following:
    ///     - `Contour`: A single contour representing a path or boundary.
    ///     - `Contours`: A collection of contours, each defining separate boundaries.
    ///     - `Shapes`: A collection of shapes, where each shape may consist of multiple contours.
    /// - `overlay_rule`: The boolean operation rule to apply when extracting shapes from the graph, such as union or intersection.
    /// - `fill_rule`: Fill rule to determine filled areas (non-zero, even-odd, positive, negative).
    /// - `scale`: Fixed float-to-integer scale. Use `scale = 1.0 / grid_size` if you prefer grid size semantics.
    /// - Returns: A vector of `Shapes<P>` representing the cleaned-up geometric result.
    fn overlay_with_fixed_scale(
        &self,
        source: &R1,
        overlay_rule: OverlayRule,
        fill_rule: FillRule,
        scale: T,
    ) -> Result<Shapes<P>, FixedScaleOverlayError>;
}

impl<R0, R1, P, T> FixedScaleFloatOverlay<R0, R1, P, T> for R0
where
    R0: ShapeResource<P, T>,
    R1: ShapeResource<P, T>,
    P: FloatPointCompatible<T>,
    T: FloatNumber,
{
    #[inline]
    fn overlay_with_fixed_scale(
        &self,
        source: &R1,
        overlay_rule: OverlayRule,
        fill_rule: FillRule,
        scale: T,
    ) -> Result<Shapes<P>, FixedScaleOverlayError> {
        Ok(
            FloatOverlay::with_subj_and_clip_fixed_scale(self, source, scale)?
                .overlay(overlay_rule, fill_rule),
        )
    }
}

impl<P: FloatPointCompatible<T>, T: FloatNumber> FloatOverlay<P, T> {
    /// Creates a new `FloatOverlay` instance and initializes it with subject and clip shapes.
    ///
    /// This variant uses a fixed float-to-integer scale instead of auto-scaling.
    /// It validates that the requested scale fits the input bounds and returns an error if not.
    ///
    /// `scale = 1.0 / grid_size` if you want a grid-size style parameter.
    /// - `subj`: A `ShapeResource` that define the subject.
    /// - `clip`: A `ShapeResource` that define the clip.
    ///   `ShapeResource` can be one of the following:
    ///     - `Contour`: A contour representing a closed path. This path is interpreted as closed, so it doesn’t require the start and endpoint to be the same for processing.
    ///     - `Contours`: A collection of contours, each representing a closed path.
    ///     - `Shapes`: A collection of shapes, where each shape may consist of multiple contours.
    pub fn with_subj_and_clip_fixed_scale<R0, R1>(
        subj: &R0,
        clip: &R1,
        scale: T,
    ) -> Result<Self, FixedScaleOverlayError>
    where
        R0: ShapeResource<P, T> + ?Sized,
        R1: ShapeResource<P, T> + ?Sized,
        P: FloatPointCompatible<T>,
        T: FloatNumber,
    {
        let s = scale.to_f64();
        if !s.is_finite() {
            return Err(FixedScaleOverlayError::ScaleNotFinite);
        }
        if s <= 0.0 {
            return Err(FixedScaleOverlayError::ScaleNonPositive);
        }

        let iter = subj.iter_paths().chain(clip.iter_paths()).flatten();
        let mut adapter = FloatPointAdapter::with_iter(iter);
        if adapter.dir_scale < scale {
            return Err(FixedScaleOverlayError::ScaleTooLarge);
        }

        adapter.dir_scale = scale;
        adapter.inv_scale = T::from_float(1.0 / s);

        let subj_capacity = subj.iter_paths().fold(0, |s, c| s + c.len());
        let clip_capacity = clip.iter_paths().fold(0, |s, c| s + c.len());

        Ok(Self::with_adapter(adapter, subj_capacity + clip_capacity)
            .unsafe_add_source(subj, ShapeType::Subject)
            .unsafe_add_source(clip, ShapeType::Clip))
    }

    /// Creates a new `FloatOverlay` instance and initializes it with subject and clip shapes.
    ///
    /// This variant uses a fixed float-to-integer scale instead of auto-scaling.
    /// It validates that the requested scale fits the input bounds and returns an error if not.
    ///
    /// `scale = 1.0 / grid_size` if you want a grid-size style parameter.
    /// - `subj`: A `ShapeResource` that define the subject.
    /// - `clip`: A `ShapeResource` that define the clip.
    ///   `ShapeResource` can be one of the following:
    ///     - `Contour`: A contour representing a closed path. This path is interpreted as closed, so it doesn’t require the start and endpoint to be the same for processing.
    ///     - `Contours`: A collection of contours, each representing a closed path.
    ///     - `Shapes`: A collection of shapes, where each shape may consist of multiple contours.
    /// - `options`: Adjust custom behavior.
    /// - `solver`: Type of solver to use.
    pub fn with_subj_and_clip_fixed_scale_custom<R0, R1>(
        subj: &R0,
        clip: &R1,
        options: OverlayOptions<T>,
        solver: Solver,
        scale: T,
    ) -> Result<Self, FixedScaleOverlayError>
    where
        R0: ShapeResource<P, T> + ?Sized,
        R1: ShapeResource<P, T> + ?Sized,
        P: FloatPointCompatible<T>,
        T: FloatNumber,
    {
        let s = scale.to_f64();
        if !s.is_finite() {
            return Err(FixedScaleOverlayError::ScaleNotFinite);
        }
        if s <= 0.0 {
            return Err(FixedScaleOverlayError::ScaleNonPositive);
        }

        let iter = subj.iter_paths().chain(clip.iter_paths()).flatten();
        let mut adapter = FloatPointAdapter::with_iter(iter);
        if adapter.dir_scale < scale {
            return Err(FixedScaleOverlayError::ScaleTooLarge);
        }

        adapter.dir_scale = scale;
        adapter.inv_scale = T::from_float(1.0 / s);

        let subj_capacity = subj.iter_paths().fold(0, |s, c| s + c.len());
        let clip_capacity = clip.iter_paths().fold(0, |s, c| s + c.len());

        Ok(
            Self::new_custom(adapter, options, solver, subj_capacity + clip_capacity)
                .unsafe_add_source(subj, ShapeType::Subject)
                .unsafe_add_source(clip, ShapeType::Clip),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::core::fill_rule::FillRule;
    use crate::core::overlay_rule::OverlayRule;
    use crate::float::overlay::FloatOverlay;
    use crate::float::scale::FixedScaleFloatOverlay;
    use alloc::vec;

    #[test]
    fn test_contour() {
        let left_rect = vec![[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]];
        let right_rect = vec![[1.0, 0.0], [1.0, 1.0], [2.0, 1.0], [2.0, 0.0]];

        let shapes = left_rect
            .overlay_with_fixed_scale(&right_rect, OverlayRule::Union, FillRule::EvenOdd, 10.0)
            .unwrap();

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].len(), 1);
        assert_eq!(shapes[0][0].len(), 4);
    }

    #[test]
    fn test_contours() {
        let r3 = vec![
            vec![[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]],
            vec![[0.0, 1.0], [0.0, 2.0], [1.0, 2.0], [1.0, 1.0]],
            vec![[1.0, 1.0], [1.0, 2.0], [2.0, 2.0], [2.0, 1.0]],
        ];
        let right_bottom_rect = vec![[1.0, 0.0], [1.0, 1.0], [2.0, 1.0], [2.0, 0.0]];

        let shapes = FloatOverlay::with_subj_and_clip_fixed_scale(&r3, &right_bottom_rect, 10.0)
            .unwrap()
            .overlay(OverlayRule::Union, FillRule::EvenOdd);

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].len(), 1);
        assert_eq!(shapes[0][0].len(), 4);
    }

    #[test]
    fn test_shapes() {
        let shapes = vec![vec![
            vec![[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]],
            vec![[0.0, 1.0], [0.0, 2.0], [1.0, 2.0], [1.0, 1.0]],
            vec![[1.0, 1.0], [1.0, 2.0], [2.0, 2.0], [2.0, 1.0]],
        ]];
        let right_bottom_rect = vec![[1.0, 0.0], [1.0, 1.0], [2.0, 1.0], [2.0, 0.0]];

        let shapes = FloatOverlay::with_subj_and_clip_fixed_scale(&shapes, &right_bottom_rect, 10.0)
            .unwrap()
            .overlay(OverlayRule::Union, FillRule::EvenOdd);

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].len(), 1);
        assert_eq!(shapes[0][0].len(), 4);
    }

    #[test]
    fn test_fail() {
        let shapes = vec![vec![
            vec![[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]],
            vec![[0.0, 1.0], [0.0, 2.0], [1.0, 2.0], [1.0, 1.0]],
            vec![[1.0, 1.0], [1.0, 2.0], [2.0, 2.0], [2.0, 1.0]],
        ]];
        let right_bottom_rect = vec![[1.0, 0.0], [1.0, 1.0], [2.0, 1.0], [2.0, 0.0]];

        let scale = (1u64 << 32) as f64;

        let result = FloatOverlay::with_subj_and_clip_fixed_scale(&shapes, &right_bottom_rect, scale);

        assert!(!result.is_ok());
    }

    #[test]
    fn test_invalid_scale() {
        let left_rect = vec![[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]];
        let right_rect = vec![[1.0, 0.0], [1.0, 1.0], [2.0, 1.0], [2.0, 0.0]];

        assert!(FloatOverlay::with_subj_and_clip_fixed_scale(&left_rect, &right_rect, -1.0).is_err());
        assert!(FloatOverlay::with_subj_and_clip_fixed_scale(&left_rect, &right_rect, 0.0).is_err());
        assert!(FloatOverlay::with_subj_and_clip_fixed_scale(&left_rect, &right_rect, f64::NAN).is_err());
        assert!(FloatOverlay::with_subj_and_clip_fixed_scale(&left_rect, &right_rect, f64::INFINITY).is_err());
    }
}
