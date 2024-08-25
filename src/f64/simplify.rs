use i_shape::f64::shape::{F64Path, F64Shape, F64Shapes};
use crate::core::fill_rule::FillRule;
use crate::core::overlay_rule::OverlayRule;
use crate::f64::overlay::F64Overlay;

pub trait Simplify {
    fn simplify(self, fill_rule: FillRule, min_area: f64) -> F64Shapes;
}


impl Simplify for F64Path {
    fn simplify(self, fill_rule: FillRule, min_area: f64) -> F64Shapes {
        let graph = F64Overlay::with_path(self, Vec::new()).into_graph(fill_rule);
        graph.extract_shapes_min_area(OverlayRule::Subject, min_area)
    }
}

impl Simplify for F64Shape {
    fn simplify(self, fill_rule: FillRule, min_area: f64) -> F64Shapes {
        let graph = F64Overlay::with_paths(self, Vec::new()).into_graph(fill_rule);
        graph.extract_shapes_min_area(OverlayRule::Subject, min_area)
    }
}

impl Simplify for F64Shapes {
    fn simplify(self, fill_rule: FillRule, min_area: f64) -> F64Shapes {
        let graph = F64Overlay::with_shapes(self, Vec::new()).into_graph(fill_rule);
        graph.extract_shapes_min_area(OverlayRule::Subject, min_area)
    }
}