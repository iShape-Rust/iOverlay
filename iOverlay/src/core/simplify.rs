//! This module provides methods to simplify paths and shapes by reducing complexity
//! (e.g., removing small artifacts or shapes below a certain area threshold) based on a fill rule.

use crate::core::fill_rule::FillRule;
use crate::core::graph::OverlayGraph;
use crate::core::link::OverlayLinkBuilder;
use crate::core::overlay::ContourDirection::Clockwise;
use crate::core::overlay::{IntOverlayOptions, Overlay, ShapeType};
use crate::core::overlay_rule::OverlayRule;
use crate::core::solver::Solver;
use crate::segm::build::BuildSegments;
use i_shape::int::count::PointsCount;
use i_shape::int::path::{IntPath, PointPathExtension};
use i_shape::int::shape::{IntContour, IntShape, IntShapes};

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
    #[inline]
    fn simplify(&self, fill_rule: FillRule, options: IntOverlayOptions) -> IntShapes {
        Overlay::with_options(self.len(), options).overlay_subject_contour(
            &self,
            fill_rule,
            Default::default(),
        )
    }
}

impl Simplify for [IntPath] {
    fn simplify(&self, fill_rule: FillRule, options: IntOverlayOptions) -> IntShapes {
        if self.len() == 1 {
            return self[0].simplify(fill_rule, options);
        }
        let mut overlay = Overlay::with_options(self.points_count(), options);
        overlay.add_contours(self, ShapeType::Subject);
        overlay.overlay_custom(OverlayRule::Subject, fill_rule, Default::default())
    }
}

impl Simplify for IntShape {
    fn simplify(&self, fill_rule: FillRule, options: IntOverlayOptions) -> IntShapes {
        if self.len() == 1 {
            return self[0].simplify(fill_rule, options);
        }
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

impl Overlay {
    #[inline]
    fn overlay_subject_contour(
        mut self,
        contour: &IntContour,
        fill_rule: FillRule,
        solver: Solver,
    ) -> IntShapes {
        let is_modified = self.segments.append_path_iter(
            contour.iter().copied(),
            ShapeType::Subject,
            self.options.preserve_input_collinear,
        );

        let early_out = !is_modified;

        if let Some(links) = OverlayLinkBuilder::build_with_overlay_short_subject(
            self.segments,
            fill_rule,
            solver,
            early_out,
        ) {
            let graph = OverlayGraph::new(solver, links);
            let filter = vec![false; graph.links.len()];
            graph.extract(filter, OverlayRule::Subject, self.options)
        } else {
            // the path is already perfect just need to check direction
            let contour_clockwise = contour.is_clockwise_ordered();
            let output_clockwise = self.options.output_direction == Clockwise;

            match fill_rule {
                FillRule::EvenOdd | FillRule::NonZero => {
                    if contour_clockwise != output_clockwise {
                        let rev_contour: Vec<_> = contour.iter().rev().cloned().collect();
                        vec![vec![rev_contour]]
                    } else {
                        vec![vec![contour.clone()]]
                    }
                }
                FillRule::Positive => {
                    if contour_clockwise == output_clockwise {
                        vec![vec![contour.clone()]]
                    } else {
                        vec![vec![vec![]]]
                    }
                }
                FillRule::Negative => {
                    if contour_clockwise != output_clockwise {
                        vec![vec![contour.clone()]]
                    } else {
                        vec![vec![vec![]]]
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::fill_rule::FillRule;
    use crate::core::simplify::Simplify;
    use i_float::int::point::IntPoint;

    #[test]
    fn test_0() {
        let contour = vec![
            IntPoint::new(0, 0),
            IntPoint::new(10, 0),
            IntPoint::new(10, 10),
            IntPoint::new(0, 10),
        ];

        let mut rev_contour = contour.clone();
        rev_contour.reverse();

        let c0 = contour.simplify(FillRule::NonZero, Default::default());
        assert_eq!(c0[0][0].len(), 4);

        let c1 = rev_contour.simplify(FillRule::NonZero, Default::default());
        assert_eq!(c1[0][0].len(), 4);
    }
}
