use i_float::fix_float::FixFloat;
use i_shape::fix_shape::FixShape;

use crate::{layout::overlay::Overlay, fill::shape_type::ShapeType};

use super::overlay_rule::OverlayRule;

pub trait SelfIntersection {
    fn resolve_self_intersection(&self) -> Vec<FixShape>;
}


impl SelfIntersection for FixShape {
    
    fn resolve_self_intersection(&self) -> Vec<FixShape> {
        let mut overlay = Overlay::new(self.paths.len());
        let paths = self.paths.clone();

        overlay.add_paths(paths, ShapeType::SUBJECT);
        let graph = overlay.build_graph();

        graph.extract_shapes_min_area(OverlayRule::Subject, FixFloat::new_i64(0))
    }
}
