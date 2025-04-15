//! This module provides methods to simplify paths and shapes by reducing complexity
//! (e.g., removing small artifacts or shapes below a certain area threshold) based on a fill rule.

use i_shape::int::count::PointsCount;
use i_shape::int::path::IntPath;
use i_shape::int::shape::IntShape;
use crate::core::fill_rule::FillRule;
use crate::core::overlay::{ContourDirection, Overlay, ShapeType};
use crate::core::overlay_rule::OverlayRule;

pub trait Simplify {
    fn simplify(&self, fill_rule: FillRule, main_direction: ContourDirection, min_area: usize) -> Vec<IntShape>;
}

impl Simplify for IntPath {
    fn simplify(&self, fill_rule: FillRule, main_direction: ContourDirection, min_area: usize) -> Vec<IntShape> {
        let mut overlay = Overlay::new(self.len());
        overlay.add_contour(self, ShapeType::Subject);
        overlay.overlay_custom(OverlayRule::Subject, fill_rule, main_direction, min_area, Default::default())
    }
}

impl Simplify for [IntPath] {
    fn simplify(&self, fill_rule: FillRule, main_direction: ContourDirection, min_area: usize) -> Vec<IntShape> {
        let mut overlay = Overlay::new(self.points_count());
        overlay.add_contours(self, ShapeType::Subject);
        overlay.overlay_custom(OverlayRule::Subject, fill_rule, main_direction, min_area, Default::default())
    }
}

impl Simplify for IntShape {
    fn simplify(&self, fill_rule: FillRule, main_direction: ContourDirection, min_area: usize) -> Vec<IntShape> {
        let mut overlay = Overlay::new(self.points_count());
        overlay.add_shape(self, ShapeType::Subject);
        overlay.overlay_custom(OverlayRule::Subject, fill_rule, main_direction, min_area, Default::default())
    }
}

impl Simplify for [IntShape] {
    fn simplify(&self, fill_rule: FillRule, main_direction: ContourDirection, min_area: usize) -> Vec<IntShape> {
        let mut overlay = Overlay::new(self.len());
        overlay.add_shapes(self, ShapeType::Subject);
        overlay.overlay_custom(OverlayRule::Subject, fill_rule, main_direction, min_area, Default::default())
    }
}