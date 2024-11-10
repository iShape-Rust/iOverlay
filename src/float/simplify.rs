use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_shape::base::data::Shapes;
use crate::core::fill_rule::FillRule;
use crate::core::overlay::ShapeType;
use crate::core::overlay_rule::OverlayRule;
use crate::float::filter::ContourFilter;
use crate::float::overlay::FloatOverlay;
use crate::float::source::resource::OverlayResource;

/// Trait `Simplify` provides a method to simplify geometric shapes by reducing the number of points in contours or shapes
/// while preserving overall shape and topology. The method applies a minimum area threshold and a fill rule to
/// determine which areas should be retained or excluded.
pub trait Simplify<P, T: FloatNumber> {
    /// Simplifies the shape or collection of points, contours, or shapes, based on a specified minimum area threshold.
    ///
    /// - `fill_rule`: Fill rule to determine filled areas (non-zero, even-odd, positive, negative).
    /// - `min_area`: The minimum area below which shapes or contours will be excluded from the result.
    /// - Returns: A collection of `Shapes<P>` that represents the simplified geometry.
    fn simplify(&self, fill_rule: FillRule, min_area: T) -> Shapes<P>;
}

impl<S, P, T> Simplify<P, T> for S
where
    S: OverlayResource<P, T>,
    P: FloatPointCompatible<T>,
    T: FloatNumber,
{
    #[inline]
    fn simplify(&self, fill_rule: FillRule, min_area: T) -> Shapes<P> {
        let adapter = FloatPointAdapter::with_iter(self.iter_paths().flatten());
        let capacity = self.iter_paths().fold(0, |s, c| s + c.len());
        let filter = ContourFilter { min_area, simplify: true };
        FloatOverlay::with_adapter(adapter, capacity)
            .unsafe_add_source(self, ShapeType::Subject)
            .overlay_with_filter_and_solver(OverlayRule::Subject, fill_rule, filter, Default::default())
    }
}

#[cfg(test)]
mod tests {
    use crate::core::fill_rule::FillRule;
    use crate::float::simplify::Simplify;

    #[test]
    fn test_contour_slice() {
        let rect = [[0.0, 0.0], [0.0, 0.5], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]];

        let shapes = rect.as_slice().simplify(FillRule::NonZero, 0.0);

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].len(), 1);
        assert_eq!(shapes[0][0].len(), 4);
    }

    #[test]
    fn test_contour_vec() {
        let rect = vec![[0.0, 0.0], [0.0, 0.5], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]];

        let shapes = rect.simplify(FillRule::NonZero, 0.0);

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].len(), 1);
        assert_eq!(shapes[0][0].len(), 4);
    }
}