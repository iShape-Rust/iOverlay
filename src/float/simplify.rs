use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_shape::base::data::{Contour, Shape, Shapes};
use i_shape::float::count::PointsCount;
use crate::core::fill_rule::FillRule;
use crate::core::overlay::ShapeType;
use crate::core::overlay_rule::OverlayRule;
use crate::float::overlay::FloatOverlay;

/// Trait `Simplify` provides a method to simplify geometric shapes by reducing the number of points in contours or shapes
/// while preserving overall shape and topology. The method applies a minimum area threshold and a fill rule to
/// determine which areas should be retained or excluded.
pub trait Simplify<P, T: FloatNumber> {
    /// Simplifies the shape or collection of points, contours, or shapes, based on a specified minimum area threshold.
    ///
    /// - `fill_rule`: Determines how filled areas are computed for the geometry, influencing the retention of certain
    ///   regions during simplification.
    /// - `min_area`: The minimum area below which shapes or contours will be excluded from the result.
    /// - Returns: A collection of `Shapes<P>` that represents the simplified geometry.
    fn simplify(&self, fill_rule: FillRule, min_area: T) -> Shapes<P>;
}

impl<P, T> Simplify<P, T> for [P]
where
    P: FloatPointCompatible<T>,
    T: FloatNumber,
{
    fn simplify(&self, fill_rule: FillRule, min_area: T) -> Shapes<P> {
        FloatOverlay::with_adapter(FloatPointAdapter::with_iter(self.iter()), self.len())
            .unsafe_add_contour(self, ShapeType::Subject)
            .overlay_with_min_area_and_solver(OverlayRule::Subject, fill_rule, min_area, Default::default())
    }
}

impl<P, T> Simplify<P, T> for [Contour<P>]
where
    P: FloatPointCompatible<T>,
    T: FloatNumber,
{
    fn simplify(&self, fill_rule: FillRule, min_area: T) -> Shapes<P> {
        FloatOverlay::with_adapter(FloatPointAdapter::with_iter(self.iter().flatten()), self.points_count())
            .unsafe_add_contours(self, ShapeType::Subject)
            .overlay_with_min_area_and_solver(OverlayRule::Subject, fill_rule, min_area, Default::default())
    }
}

impl<P, T> Simplify<P, T> for [Shape<P>]
where
    P: FloatPointCompatible<T>,
    T: FloatNumber,
{
    fn simplify(&self, fill_rule: FillRule, min_area: T) -> Shapes<P> {
        FloatOverlay::with_adapter(FloatPointAdapter::with_iter(self.iter().flatten().flatten()), self.points_count())
            .unsafe_add_shapes(self, ShapeType::Subject)
            .overlay_with_min_area_and_solver(OverlayRule::Subject, fill_rule, min_area, Default::default())
    }
}