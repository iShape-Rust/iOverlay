use i_shape::fix_path::FixPath;
use i_shape::fix_shape::FixShape;
use crate::core::fill_rule::FillRule;
use crate::core::overlay::{Overlay, ShapeType};
use crate::core::overlay_rule::OverlayRule;

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

impl Simplify for [FixPath] {

    fn simplify(&self, fill_rule: FillRule) -> Vec<FixShape> {
        let mut overlay = Overlay::new(self.len());

        overlay.add_paths(self, ShapeType::Subject);

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

impl Simplify for [FixShape] {

    fn simplify(&self, fill_rule: FillRule) -> Vec<FixShape> {
        let mut overlay = Overlay::new(self.len());
        for shape in self.iter() {
            overlay.add_shape(shape, ShapeType::Subject);
        }
        let graph = overlay.build_graph(fill_rule);
        graph.extract_shapes(OverlayRule::Subject)
    }

}