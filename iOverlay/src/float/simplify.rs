use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_shape::base::data::Shapes;
use crate::core::fill_rule::FillRule;
use crate::core::overlay_rule::OverlayRule;
use crate::core::solver::Solver;
use crate::float::overlay::{FloatOverlay, OverlayOptions};
use crate::float::source::resource::OverlayResource;

/// Trait `Simplify` provides a method to simplify geometric shapes by reducing the number of points in contours or shapes
/// while preserving overall shape and topology. The method applies a minimum area threshold and a fill rule to
/// determine which areas should be retained or excluded.
pub trait SimplifyShape<P, T: FloatNumber> {
    /// Simplifies the shape or collection of points, contours, or shapes, based on a specified minimum area threshold.
    ///
    /// - `options`: Adjust custom behavior.
    /// - Returns: A collection of `Shapes<P>` that represents the simplified geometry.
    ///
    /// Note: Outer boundary paths have a **main_direction** order, and holes have an opposite to **main_direction** order.
    fn simplify_shape(&self, fill_rule: FillRule, options: OverlayOptions<T>) -> Shapes<P>;

    /// Simplifies the shape or collection of points, contours, or shapes, based on a specified minimum area threshold.
    /// - `options`: Adjust custom behavior.
    /// - `solver`: Type of solver to use.
    /// - Returns: A collection of Shapes<P> that represents the simplified geometry.
    ///
    /// Note: Outer boundary paths have a **main_direction** order, and holes have an opposite to **main_direction** order.
    fn simplify_shape_with_solver(&self, fill_rule: FillRule, options: OverlayOptions<T>, solver: Solver) -> Shapes<P>;
}

impl<S, P, T> SimplifyShape<P, T> for S
where
    S: OverlayResource<P, T>,
    P: FloatPointCompatible<T>,
    T: FloatNumber,
{
    #[inline]
    fn simplify_shape(&self, fill_rule: FillRule, options: OverlayOptions<T>) -> Shapes<P> {

        FloatOverlay::with_subj(self)
            .overlay_custom(OverlayRule::Subject, fill_rule, options, Default::default())
    }

    #[inline]
    fn simplify_shape_with_solver(&self, fill_rule: FillRule, options: OverlayOptions<T>, solver: Solver) -> Shapes<P> {
        FloatOverlay::with_subj(self)
            .overlay_custom(OverlayRule::Subject, fill_rule, options, solver)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::fill_rule::FillRule;
    use crate::float::simplify::SimplifyShape;

    #[test]
    fn test_contour_slice() {
        let rect = [[0.0, 0.0], [0.0, 0.5], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]];

        let shapes = rect.as_slice().simplify_shape(FillRule::NonZero, Default::default());

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].len(), 1);
        assert_eq!(shapes[0][0].len(), 4);
    }

    #[test]
    fn test_contour_vec() {
        let rect = vec![[0.0, 0.0], [0.0, 0.5], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]];

        let shapes = rect.simplify_shape(FillRule::NonZero, Default::default());

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].len(), 1);
        assert_eq!(shapes[0][0].len(), 4);
    }
}