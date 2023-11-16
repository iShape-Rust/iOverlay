use i_shape::fix_path::FixPath;
use i_shape::fix_shape::FixShape;
use crate::bool::fill_rule::FillRule;
use crate::layout::overlay::Overlay;

pub trait Simplify {

    fn simplify(&self) -> Vec<FixShape>;

}

impl Simplify for FixPath {

    fn simplify(&self) -> Vec<FixShape> {
        let mut overlay = Overlay::from_subject_path(self.clone());
        let graph = overlay.build_graph();
        graph.extract_shapes(FillRule::Subject)
    }

}

impl Simplify for FixShape {

    fn simplify(&self) -> Vec<FixShape> {
        let paths = self.paths.clone();
        let mut overlay = Overlay::from_subject_paths(paths);
        let graph = overlay.build_graph();
        graph.extract_shapes(FillRule::Subject)
    }

}