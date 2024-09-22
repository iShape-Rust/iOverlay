//! This module provides methods to simplify paths and shapes by reducing complexity
//! (e.g., removing small artifacts or shapes below a certain area threshold) based on a fill rule.

use i_shape::int::path::IntPath;
use i_shape::int::shape::{IntShape, PointsCount};
use crate::core::fill_rule::FillRule;
use crate::core::overlay::{Overlay, ShapeType};
use crate::core::overlay_rule::OverlayRule;

pub trait Simplify {
    fn simplify(&self, fill_rule: FillRule, min_area: i64) -> Vec<IntShape>;
}

impl Simplify for IntPath {
    fn simplify(&self, fill_rule: FillRule, min_area: i64) -> Vec<IntShape> {
        let mut overlay = Overlay::new(self.len());
        overlay.add_path(self, ShapeType::Subject);

        let graph = overlay.into_graph(fill_rule);
        graph.extract_shapes_min_area(OverlayRule::Subject, min_area)
    }
}

impl Simplify for [IntPath] {
    fn simplify(&self, fill_rule: FillRule, min_area: i64) -> Vec<IntShape> {
        let mut overlay = Overlay::new(self.points_count());

        overlay.add_paths(self, ShapeType::Subject);

        let graph = overlay.into_graph(fill_rule);
        graph.extract_shapes_min_area(OverlayRule::Subject, min_area)
    }
}

impl Simplify for IntShape {
    fn simplify(&self, fill_rule: FillRule, min_area: i64) -> Vec<IntShape> {
        let mut overlay = Overlay::new(self.points_count());
        overlay.add_shape(self, ShapeType::Subject);
        let graph = overlay.into_graph(fill_rule);
        graph.extract_shapes_min_area(OverlayRule::Subject, min_area)
    }
}

impl Simplify for [IntShape] {
    fn simplify(&self, fill_rule: FillRule, min_area: i64) -> Vec<IntShape> {
        let mut overlay = Overlay::new(self.len());
        for shape in self.iter() {
            overlay.add_shape(shape, ShapeType::Subject);
        }
        let graph = overlay.into_graph(fill_rule);
        graph.extract_shapes_min_area(OverlayRule::Subject, min_area)
    }
}