use i_shape::fix_shape::FixShape;

use crate::{layout::overlay::Overlay, fill::shape_type::ShapeType};

use super::fill_rule::FillRule;

pub trait SelfIntersection {
    fn resolve_self_intersection(&self) -> Vec<FixShape>;
}


impl SelfIntersection for FixShape {
    
    fn resolve_self_intersection(&self) -> Vec<FixShape> {
        let mut overlay = Overlay::new(self.count());
        let paths = self.paths().clone();

        overlay.add_paths(paths, ShapeType::SUBJECT);
        let graph = overlay.build_graph();

        graph.extract_shapes(FillRule::Subject)
    }
}
