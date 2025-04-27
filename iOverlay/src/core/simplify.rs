//! This module provides methods to simplify paths and shapes by reducing complexity
//! (e.g., removing small artifacts or shapes below a certain area threshold) based on a fill rule.

use i_shape::int::count::PointsCount;
use i_shape::int::path::IntPath;
use i_shape::int::shape::{IntShape, IntShapes};
use crate::core::fill_rule::FillRule;
use crate::core::overlay::{IntOverlayOptions, Overlay, ShapeType};
use crate::core::overlay_rule::OverlayRule;

/// Trait `Simplify` provides a method to simplify geometric shapes by reducing the number of points in contours or shapes
/// while preserving overall shape and topology. The method applies a minimum area threshold and a fill rule to
/// determine which areas should be retained or excluded.
pub trait Simplify {
    /// Simplifies the shape or collection of points, contours, or shapes, based on a specified minimum area threshold.
    ///
    /// - `fill_rule`: Fill rule to determine filled areas (non-zero, even-odd, positive, negative).
    /// - `options`: Adjust custom behavior.
    /// # Shape Representation
    /// The output is a `IntShapes`, where:
    /// - The outer `Vec<IntShape>` represents a set of shapes.
    /// - Each shape `Vec<IntContour>` represents a collection of contours, where the first contour is the outer boundary, and all subsequent contours are holes in this boundary.
    /// - Each path `Vec<IntPoint>` is a sequence of points, forming a closed path.
    ///
    /// Note: Outer boundary paths have a **main_direction** order, and holes have an opposite to **main_direction** order.
    fn simplify(&self, fill_rule: FillRule, options: IntOverlayOptions) -> IntShapes;
}

impl Simplify for IntPath {
    fn simplify(&self, fill_rule: FillRule, options: IntOverlayOptions) -> IntShapes {
        let mut overlay = Overlay::with_options(self.len(), options);
        overlay.add_contour(self, ShapeType::Subject);
        overlay.overlay_custom(OverlayRule::Subject, fill_rule, Default::default())
    }
}

impl Simplify for [IntPath] {
    fn simplify(&self, fill_rule: FillRule, options: IntOverlayOptions) -> IntShapes {
        let mut overlay = Overlay::with_options(self.points_count(), options);
        overlay.add_contours(self, ShapeType::Subject);
        overlay.overlay_custom(OverlayRule::Subject, fill_rule, Default::default())
    }
}

impl Simplify for IntShape {
    fn simplify(&self, fill_rule: FillRule, options: IntOverlayOptions) -> IntShapes {
        let mut overlay = Overlay::with_options(self.points_count(), options);
        overlay.add_shape(self, ShapeType::Subject);
        overlay.overlay_custom(OverlayRule::Subject, fill_rule, Default::default())
    }
}

impl Simplify for [IntShape] {
    fn simplify(&self, fill_rule: FillRule, options: IntOverlayOptions) -> IntShapes {
        let mut overlay = Overlay::with_options(self.points_count(), options);
        overlay.add_shapes(self, ShapeType::Subject);
        overlay.overlay_custom(OverlayRule::Subject, fill_rule, Default::default())
    }
}