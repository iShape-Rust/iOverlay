use i_float::adapter::FloatPointAdapter;
use i_float::float::Float;
use i_float::float_point::FloatPointCompatible;
use crate::core::fill_rule::FillRule;
use crate::core::overlay::ShapeType;
use crate::core::overlay_rule::OverlayRule;
use crate::float::overlay::FloatOverlay;

pub trait Simplify<P, T: Float> {
    fn simplify(self, fill_rule: FillRule, min_area: f64) -> Vec<Vec<Vec<P>>>;
}


impl<P, T> Simplify<P, T> for Vec<P>
where
    P: FloatPointCompatible<T>,
    T: Float,
{
    fn simplify(self, fill_rule: FillRule, min_area: f64) -> Vec<Vec<Vec<P>>> {
        let iter = self.iter().map(|p| p.to_float_point());
        let adapter = FloatPointAdapter::with_iter(iter);

        FloatOverlay::new(adapter, self.len())
            .unsafe_add_path(&self, ShapeType::Subject)
            .into_graph(fill_rule).extract_shapes_min_area(OverlayRule::Subject, min_area)
    }
}

impl<P, T> Simplify<P, T> for Vec<Vec<P>>
where
    P: FloatPointCompatible<T>,
    T: Float,
{
    fn simplify(self, fill_rule: FillRule, min_area: f64) -> Vec<Vec<Vec<P>>> {
        let iter = self.iter().flatten().map(|p| p.to_float_point());
        let adapter = FloatPointAdapter::with_iter(iter);

        let capacity: usize = self.iter().map(|path| path.len()).sum();
        FloatOverlay::new(adapter, capacity)
            .unsafe_add_paths(&self, ShapeType::Subject)
            .into_graph(fill_rule).extract_shapes_min_area(OverlayRule::Subject, min_area)
    }
}

impl<P, T> Simplify<P, T> for Vec<Vec<Vec<P>>>
where
    P: FloatPointCompatible<T>,
    T: Float,
{
    fn simplify(self, fill_rule: FillRule, min_area: f64) -> Vec<Vec<Vec<P>>> {
        let iter = self.iter().flatten().flatten().map(|p| p.to_float_point());
        let adapter = FloatPointAdapter::with_iter(iter);

        let capacity: usize = self.iter().flatten().map(|path| path.len()).sum();
        FloatOverlay::new(adapter, capacity)
            .unsafe_add_shapes(&self, ShapeType::Subject)
            .into_graph(fill_rule).extract_shapes_min_area(OverlayRule::Subject, min_area)
    }
}