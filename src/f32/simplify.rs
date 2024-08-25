use i_shape::f32::shape::{F32Path, F32Shape, F32Shapes};
use crate::core::fill_rule::FillRule;
use crate::core::overlay_rule::OverlayRule;
use crate::f32::overlay::F32Overlay;

pub trait Simplify {
    fn simplify(self, fill_rule: FillRule, min_area: f32) -> F32Shapes;
}


impl Simplify for F32Path {
    fn simplify(self, fill_rule: FillRule, min_area: f32) -> F32Shapes {
        let graph = F32Overlay::with_path(self, Vec::new()).into_graph(fill_rule);
        graph.extract_shapes_min_area(OverlayRule::Subject, min_area)
    }
}

impl Simplify for F32Shape {
    fn simplify(self, fill_rule: FillRule, min_area: f32) -> F32Shapes {
        let graph = F32Overlay::with_paths(self, Vec::new()).into_graph(fill_rule);
        graph.extract_shapes_min_area(OverlayRule::Subject, min_area)
    }
}

impl Simplify for F32Shapes {
    fn simplify(self, fill_rule: FillRule, min_area: f32) -> F32Shapes {
        let graph = F32Overlay::with_shapes(self, Vec::new()).into_graph(fill_rule);
        graph.extract_shapes_min_area(OverlayRule::Subject, min_area)
    }
}