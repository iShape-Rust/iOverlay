use crate::core::fill_rule::FillRule;
use crate::core::overlay::ShapeType;
use crate::core::overlay_rule::OverlayRule;
use crate::core::solver::Solver;
use crate::float::overlay::{FloatOverlay, OverlayOptions};
use crate::float::relate::FloatPredicateOverlay;
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

impl FixedScaleOverlayError {
    #[inline]
    pub fn validate_scale<T: FloatNumber>(scale: T) -> Result<f64, Self> {
        let s = scale.to_f64();
        if !s.is_finite() {
            return Err(Self::ScaleNotFinite);
        }
        if s <= 0.0 {
            return Err(Self::ScaleNonPositive);
        }
        Ok(s)
    }
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
        Ok(FloatOverlay::with_subj_and_clip_fixed_scale(self, source, scale)?
            .overlay(overlay_rule, fill_rule))
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
        let s = FixedScaleOverlayError::validate_scale(scale)?;

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
        let s = FixedScaleOverlayError::validate_scale(scale)?;

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

impl<P: FloatPointCompatible<T>, T: FloatNumber> FloatPredicateOverlay<P, T> {
    /// Creates a new predicate overlay with subject and clip shapes using fixed-scale precision.
    ///
    /// This variant uses a fixed float-to-integer scale instead of auto-scaling.
    /// It validates that the requested scale fits the input bounds and returns an error if not.
    ///
    /// `scale = 1.0 / grid_size` if you want a grid-size style parameter.
    ///
    /// # Arguments
    /// * `subj` - A `ShapeResource` defining the subject geometry.
    /// * `clip` - A `ShapeResource` defining the clip geometry.
    /// * `scale` - Fixed float-to-integer scale factor.
    pub fn with_subj_and_clip_fixed_scale<R0, R1>(
        subj: &R0,
        clip: &R1,
        scale: T,
    ) -> Result<Self, FixedScaleOverlayError>
    where
        R0: ShapeResource<P, T> + ?Sized,
        R1: ShapeResource<P, T> + ?Sized,
    {
        let s = FixedScaleOverlayError::validate_scale(scale)?;

        let iter = subj.iter_paths().chain(clip.iter_paths()).flatten();
        let mut adapter = FloatPointAdapter::with_iter(iter);
        if adapter.dir_scale < scale {
            return Err(FixedScaleOverlayError::ScaleTooLarge);
        }

        adapter.dir_scale = scale;
        adapter.inv_scale = T::from_float(1.0 / s);

        let subj_capacity = subj.iter_paths().fold(0, |s, c| s + c.len());
        let clip_capacity = clip.iter_paths().fold(0, |s, c| s + c.len());

        let mut result = Self::with_adapter(adapter, subj_capacity + clip_capacity);
        result.add_source(subj, ShapeType::Subject);
        result.add_source(clip, ShapeType::Clip);
        Ok(result)
    }

    /// Creates a new predicate overlay with subject and clip shapes using fixed-scale precision
    /// and custom fill rule and solver.
    ///
    /// # Arguments
    /// * `subj` - A `ShapeResource` defining the subject geometry.
    /// * `clip` - A `ShapeResource` defining the clip geometry.
    /// * `fill_rule` - Fill rule to determine filled areas.
    /// * `solver` - Type of solver to use.
    /// * `scale` - Fixed float-to-integer scale factor.
    pub fn with_subj_and_clip_fixed_scale_custom<R0, R1>(
        subj: &R0,
        clip: &R1,
        fill_rule: FillRule,
        solver: Solver,
        scale: T,
    ) -> Result<Self, FixedScaleOverlayError>
    where
        R0: ShapeResource<P, T> + ?Sized,
        R1: ShapeResource<P, T> + ?Sized,
    {
        let s = FixedScaleOverlayError::validate_scale(scale)?;

        let iter = subj.iter_paths().chain(clip.iter_paths()).flatten();
        let mut adapter = FloatPointAdapter::with_iter(iter);
        if adapter.dir_scale < scale {
            return Err(FixedScaleOverlayError::ScaleTooLarge);
        }

        adapter.dir_scale = scale;
        adapter.inv_scale = T::from_float(1.0 / s);

        let subj_capacity = subj.iter_paths().fold(0, |s, c| s + c.len());
        let clip_capacity = clip.iter_paths().fold(0, |s, c| s + c.len());

        let mut result = Self::with_adapter_custom(adapter, fill_rule, solver, subj_capacity + clip_capacity);
        result.add_source(subj, ShapeType::Subject);
        result.add_source(clip, ShapeType::Clip);
        Ok(result)
    }
}

/// Trait for spatial predicate operations with fixed-scale precision.
///
/// This trait provides methods for testing spatial relationships using a fixed
/// float-to-integer scale, which is useful when you need consistent precision
/// across multiple operations or when working with known coordinate bounds.
///
/// # Example
///
/// ```
/// use i_overlay::float::scale::FixedScaleFloatRelate;
///
/// let square = vec![[0.0, 0.0], [0.0, 10.0], [10.0, 10.0], [10.0, 0.0]];
/// let other = vec![[5.0, 5.0], [5.0, 15.0], [15.0, 15.0], [15.0, 5.0]];
///
/// // Use fixed scale of 1000.0 for consistent precision
/// let result = square.intersects_with_fixed_scale(&other, 1000.0);
/// assert!(result.unwrap());
/// ```
pub trait FixedScaleFloatRelate<R1, P, T>
where
    R1: ShapeResource<P, T> + ?Sized,
    P: FloatPointCompatible<T>,
    T: FloatNumber,
{
    /// Returns `true` if shapes intersect, using fixed-scale precision.
    fn intersects_with_fixed_scale(&self, other: &R1, scale: T) -> Result<bool, FixedScaleOverlayError>;

    /// Returns `true` if interiors of shapes overlap, using fixed-scale precision.
    fn interiors_intersect_with_fixed_scale(
        &self,
        other: &R1,
        scale: T,
    ) -> Result<bool, FixedScaleOverlayError>;

    /// Returns `true` if shapes touch (boundaries intersect but interiors don't), using fixed-scale precision.
    fn touches_with_fixed_scale(&self, other: &R1, scale: T) -> Result<bool, FixedScaleOverlayError>;

    /// Returns `true` if this shape is completely within another, using fixed-scale precision.
    fn within_with_fixed_scale(&self, other: &R1, scale: T) -> Result<bool, FixedScaleOverlayError>;

    /// Returns `true` if shapes do not intersect, using fixed-scale precision.
    fn disjoint_with_fixed_scale(&self, other: &R1, scale: T) -> Result<bool, FixedScaleOverlayError>;

    /// Returns `true` if this shape completely covers another, using fixed-scale precision.
    fn covers_with_fixed_scale(&self, other: &R1, scale: T) -> Result<bool, FixedScaleOverlayError>;
}

impl<R0, R1, P, T> FixedScaleFloatRelate<R1, P, T> for R0
where
    R0: ShapeResource<P, T> + ?Sized,
    R1: ShapeResource<P, T> + ?Sized,
    P: FloatPointCompatible<T>,
    T: FloatNumber,
{
    #[inline]
    fn intersects_with_fixed_scale(&self, other: &R1, scale: T) -> Result<bool, FixedScaleOverlayError> {
        Ok(FloatPredicateOverlay::with_subj_and_clip_fixed_scale(self, other, scale)?.intersects())
    }

    #[inline]
    fn interiors_intersect_with_fixed_scale(
        &self,
        other: &R1,
        scale: T,
    ) -> Result<bool, FixedScaleOverlayError> {
        Ok(FloatPredicateOverlay::with_subj_and_clip_fixed_scale(self, other, scale)?.interiors_intersect())
    }

    #[inline]
    fn touches_with_fixed_scale(&self, other: &R1, scale: T) -> Result<bool, FixedScaleOverlayError> {
        Ok(FloatPredicateOverlay::with_subj_and_clip_fixed_scale(self, other, scale)?.touches())
    }

    #[inline]
    fn within_with_fixed_scale(&self, other: &R1, scale: T) -> Result<bool, FixedScaleOverlayError> {
        Ok(FloatPredicateOverlay::with_subj_and_clip_fixed_scale(self, other, scale)?.within())
    }

    #[inline]
    fn disjoint_with_fixed_scale(&self, other: &R1, scale: T) -> Result<bool, FixedScaleOverlayError> {
        Ok(!FloatPredicateOverlay::with_subj_and_clip_fixed_scale(self, other, scale)?.intersects())
    }

    #[inline]
    fn covers_with_fixed_scale(&self, other: &R1, scale: T) -> Result<bool, FixedScaleOverlayError> {
        Ok(FloatPredicateOverlay::with_subj_and_clip_fixed_scale(other, self, scale)?.within())
    }
}

#[cfg(test)]
mod tests {
    use crate::core::fill_rule::FillRule;
    use crate::core::overlay_rule::OverlayRule;
    use crate::float::overlay::FloatOverlay;
    use crate::float::relate::FloatPredicateOverlay;
    use crate::float::scale::{FixedScaleFloatOverlay, FixedScaleFloatRelate};
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
        assert!(
            FloatOverlay::with_subj_and_clip_fixed_scale(&left_rect, &right_rect, f64::INFINITY).is_err()
        );
    }

    #[test]
    fn test_intersects_with_fixed_scale() {
        let square = vec![[0.0, 0.0], [0.0, 10.0], [10.0, 10.0], [10.0, 0.0]];
        let other = vec![[5.0, 5.0], [5.0, 15.0], [15.0, 15.0], [15.0, 5.0]];

        let result = square.intersects_with_fixed_scale(&other, 1000.0);
        assert!(result.unwrap());
    }

    #[test]
    fn test_intersects_with_fixed_scale_disjoint() {
        let square = vec![[0.0, 0.0], [0.0, 10.0], [10.0, 10.0], [10.0, 0.0]];
        let other = vec![[20.0, 20.0], [20.0, 30.0], [30.0, 30.0], [30.0, 20.0]];

        let result = square.intersects_with_fixed_scale(&other, 1000.0);
        assert!(!result.unwrap());
    }

    #[test]
    fn test_interiors_intersect_with_fixed_scale() {
        let square = vec![[0.0, 0.0], [0.0, 10.0], [10.0, 10.0], [10.0, 0.0]];
        let other = vec![[5.0, 5.0], [5.0, 15.0], [15.0, 15.0], [15.0, 5.0]];

        let result = square.interiors_intersect_with_fixed_scale(&other, 1000.0);
        assert!(result.unwrap());
    }

    #[test]
    fn test_touches_with_fixed_scale() {
        let left = vec![[0.0, 0.0], [0.0, 10.0], [10.0, 10.0], [10.0, 0.0]];
        let right = vec![[10.0, 0.0], [10.0, 10.0], [20.0, 10.0], [20.0, 0.0]];

        let result = left.touches_with_fixed_scale(&right, 1000.0);
        assert!(result.unwrap());
    }

    #[test]
    fn test_within_with_fixed_scale() {
        let outer = vec![[0.0, 0.0], [0.0, 20.0], [20.0, 20.0], [20.0, 0.0]];
        let inner = vec![[5.0, 5.0], [5.0, 15.0], [15.0, 15.0], [15.0, 5.0]];

        let result = inner.within_with_fixed_scale(&outer, 1000.0);
        assert!(result.unwrap());
    }

    #[test]
    fn test_predicate_overlay_with_fixed_scale() {
        let square = vec![[0.0, 0.0], [0.0, 10.0], [10.0, 10.0], [10.0, 0.0]];
        let other = vec![[5.0, 5.0], [5.0, 15.0], [15.0, 15.0], [15.0, 5.0]];

        let mut overlay =
            FloatPredicateOverlay::with_subj_and_clip_fixed_scale(&square, &other, 1000.0).unwrap();
        assert!(overlay.intersects());
    }

    #[test]
    fn test_predicate_fixed_scale_invalid() {
        let square = vec![[0.0, 0.0], [0.0, 10.0], [10.0, 10.0], [10.0, 0.0]];
        let other = vec![[5.0, 5.0], [5.0, 15.0], [15.0, 15.0], [15.0, 5.0]];

        assert!(FloatPredicateOverlay::with_subj_and_clip_fixed_scale(&square, &other, -1.0).is_err());
        assert!(FloatPredicateOverlay::with_subj_and_clip_fixed_scale(&square, &other, 0.0).is_err());
        assert!(FloatPredicateOverlay::with_subj_and_clip_fixed_scale(&square, &other, f64::NAN).is_err());
        assert!(
            FloatPredicateOverlay::with_subj_and_clip_fixed_scale(&square, &other, f64::INFINITY).is_err()
        );
    }

    #[test]
    fn test_disjoint_with_fixed_scale() {
        let square = vec![[0.0, 0.0], [0.0, 10.0], [10.0, 10.0], [10.0, 0.0]];
        let other = vec![[20.0, 20.0], [20.0, 30.0], [30.0, 30.0], [30.0, 20.0]];

        let result = square.disjoint_with_fixed_scale(&other, 1000.0);
        assert!(result.unwrap());

        let overlapping = vec![[5.0, 5.0], [5.0, 15.0], [15.0, 15.0], [15.0, 5.0]];
        let result = square.disjoint_with_fixed_scale(&overlapping, 1000.0);
        assert!(!result.unwrap());
    }

    #[test]
    fn test_covers_with_fixed_scale() {
        let outer = vec![[0.0, 0.0], [0.0, 20.0], [20.0, 20.0], [20.0, 0.0]];
        let inner = vec![[5.0, 5.0], [5.0, 15.0], [15.0, 15.0], [15.0, 5.0]];

        let result = outer.covers_with_fixed_scale(&inner, 1000.0);
        assert!(result.unwrap());

        let result = inner.covers_with_fixed_scale(&outer, 1000.0);
        assert!(!result.unwrap());
    }

    #[test]
    fn test_fixed_scale_custom_overlay() {
        use crate::core::solver::Solver;

        let left_rect = vec![[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]];
        let right_rect = vec![[1.0, 0.0], [1.0, 1.0], [2.0, 1.0], [2.0, 0.0]];

        let shapes = FloatOverlay::with_subj_and_clip_fixed_scale_custom(
            &left_rect,
            &right_rect,
            Default::default(),
            Solver::default(),
            10.0,
        )
        .unwrap()
        .overlay(OverlayRule::Union, FillRule::EvenOdd);

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].len(), 1);
        assert_eq!(shapes[0][0].len(), 4);
    }

    #[test]
    fn test_fixed_scale_custom_overlay_invalid() {
        use crate::core::solver::Solver;

        let left_rect = vec![[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]];
        let right_rect = vec![[1.0, 0.0], [1.0, 1.0], [2.0, 1.0], [2.0, 0.0]];

        let result = FloatOverlay::with_subj_and_clip_fixed_scale_custom(
            &left_rect,
            &right_rect,
            Default::default(),
            Solver::default(),
            -1.0,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_fixed_scale_custom_overlay_scale_too_large() {
        use crate::core::solver::Solver;

        let left_rect = vec![[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]];
        let right_rect = vec![[1.0, 0.0], [1.0, 1.0], [2.0, 1.0], [2.0, 0.0]];

        let scale = (1u64 << 32) as f64;
        let result = FloatOverlay::with_subj_and_clip_fixed_scale_custom(
            &left_rect,
            &right_rect,
            Default::default(),
            Solver::default(),
            scale,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_predicate_overlay_with_fixed_scale_custom() {
        use crate::core::solver::Solver;

        let square = vec![[0.0, 0.0], [0.0, 10.0], [10.0, 10.0], [10.0, 0.0]];
        let other = vec![[5.0, 5.0], [5.0, 15.0], [15.0, 15.0], [15.0, 5.0]];

        let mut overlay = FloatPredicateOverlay::with_subj_and_clip_fixed_scale_custom(
            &square,
            &other,
            FillRule::NonZero,
            Solver::default(),
            1000.0,
        )
        .unwrap();
        assert!(overlay.intersects());
    }

    #[test]
    fn test_predicate_overlay_with_fixed_scale_custom_invalid() {
        use crate::core::solver::Solver;

        let square = vec![[0.0, 0.0], [0.0, 10.0], [10.0, 10.0], [10.0, 0.0]];
        let other = vec![[5.0, 5.0], [5.0, 15.0], [15.0, 15.0], [15.0, 5.0]];

        let result = FloatPredicateOverlay::with_subj_and_clip_fixed_scale_custom(
            &square,
            &other,
            FillRule::NonZero,
            Solver::default(),
            -1.0,
        );
        assert!(result.is_err());

        let scale = (1u64 << 32) as f64;
        let result = FloatPredicateOverlay::with_subj_and_clip_fixed_scale_custom(
            &square,
            &other,
            FillRule::NonZero,
            Solver::default(),
            scale,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_fixed_scale_relate_invalid_scale() {
        let square = vec![[0.0, 0.0], [0.0, 10.0], [10.0, 10.0], [10.0, 0.0]];
        let other = vec![[5.0, 5.0], [5.0, 15.0], [15.0, 15.0], [15.0, 5.0]];

        assert!(square.intersects_with_fixed_scale(&other, -1.0).is_err());
        assert!(square.interiors_intersect_with_fixed_scale(&other, -1.0).is_err());
        assert!(square.touches_with_fixed_scale(&other, -1.0).is_err());
        assert!(square.within_with_fixed_scale(&other, -1.0).is_err());
        assert!(square.disjoint_with_fixed_scale(&other, -1.0).is_err());
        assert!(square.covers_with_fixed_scale(&other, -1.0).is_err());
    }
}
