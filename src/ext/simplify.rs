use i_shape::fix_path::FixPath;
use i_shape::fix_shape::FixShape;
use crate::bool::fill_rule::FillRule;
use crate::bool::overlay_rule::OverlayRule;
use crate::layout::overlay::{Overlay, ShapeType};

pub trait Simplify {

    fn simplify(&self, fill_rule: FillRule) -> Vec<FixShape>;

}

impl Simplify for FixPath {

    fn simplify(&self, fill_rule: FillRule) -> Vec<FixShape> {
        let mut overlay = Overlay::new(self.len());
        overlay.add_path(self, ShapeType::Subject);

        let graph = overlay.build_graph(fill_rule);
        graph.extract_shapes(OverlayRule::Subject)
    }

}

impl Simplify for FixShape {

    fn simplify(&self, fill_rule: FillRule) -> Vec<FixShape> {
        let mut overlay = Overlay::new(self.paths[0].len());
        overlay.add_shape(self, ShapeType::Subject);
        let graph = overlay.build_graph(fill_rule);
        graph.extract_shapes(OverlayRule::Subject)
    }

}