use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_shape::base::data::{Contour, Shape, Shapes};
use i_shape::float::count::PointsCount;
use crate::core::fill_rule::FillRule;
use crate::core::overlay::ShapeType;
use crate::core::overlay_rule::OverlayRule;
use crate::float::overlay::FloatOverlay;

pub trait Simplify<P, T: FloatNumber> {
    fn simplify(self, fill_rule: FillRule, min_area: T) -> Shapes<P>;
}

impl<P, T> Simplify<P, T> for Contour<P>
where
    P: FloatPointCompatible<T>,
    T: FloatNumber,
{
    fn simplify(self, fill_rule: FillRule, min_area: T) -> Shapes<P> {
        FloatOverlay::new(FloatPointAdapter::with_iter(self.iter()), self.len())
            .unsafe_add_path(&self, ShapeType::Subject)
            .into_graph(fill_rule)
            .extract_shapes_min_area(OverlayRule::Subject, min_area)
    }
}

impl<P, T> Simplify<P, T> for Shape<P>
where
    P: FloatPointCompatible<T>,
    T: FloatNumber,
{
    fn simplify(self, fill_rule: FillRule, min_area: T) -> Shapes<P> {
        FloatOverlay::new(FloatPointAdapter::with_iter(self.iter().flatten()), self.points_count())
            .unsafe_add_paths(&self, ShapeType::Subject)
            .into_graph(fill_rule)
            .extract_shapes_min_area(OverlayRule::Subject, min_area)
    }
}

impl<P, T> Simplify<P, T> for Shapes<P>
where
    P: FloatPointCompatible<T>,
    T: FloatNumber,
{
    fn simplify(self, fill_rule: FillRule, min_area: T) -> Shapes<P> {
        FloatOverlay::new(FloatPointAdapter::with_iter(self.iter().flatten().flatten()), self.points_count())
            .unsafe_add_shapes(&self, ShapeType::Subject)
            .into_graph(fill_rule)
            .extract_shapes_min_area(OverlayRule::Subject, min_area)
    }
}