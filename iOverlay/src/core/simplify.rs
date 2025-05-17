//! This module provides methods to simplify paths and shapes by reducing complexity
//! (e.g., removing small artifacts or shapes below a certain area threshold) based on a fill rule.

use crate::core::overlay::ContourDirection;
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
        let append_modified = self.segments.append_path_iter(
            contour.iter().copied(),
            ShapeType::Subject,
            self.options.preserve_input_collinear,
        );

        let split_modified = self.split_solver.split_segments(&mut self.segments, &solver);
        if !split_modified && !append_modified && !Self::has_loops(contour) {
            // the path is perfect just need to check direction
            return Self::apply_fill_rule(self.options.output_direction, fill_rule, contour);
        }

        let links = OverlayLinkBuilder::build_with_overlay_filter(
            &self.segments,
            fill_rule,
            OverlayRule::Subject,
            &solver
        );

        let graph = OverlayGraph::new(solver, links);
        let filter = vec![false; graph.links.len()];
        graph.extract(filter, OverlayRule::Subject, self.options, &mut self.points_buffer)
    }

    #[inline]
    fn apply_fill_rule(output_direction: ContourDirection, fill_rule: FillRule, contour: &IntContour) -> IntShapes {
        let contour_clockwise = contour.is_clockwise_ordered();
        let output_clockwise = output_direction == Clockwise;

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

    #[inline]
    fn has_loops(contour: &IntContour) -> bool {
        use std::collections::HashSet;
        let mut seen = HashSet::with_capacity(contour.len());
        for pt in contour {
            if !seen.insert(pt) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::core::fill_rule::FillRule;
    use crate::core::simplify::Simplify;
    use i_float::int::point::IntPoint;
    use crate::core::overlay::IntOverlayOptions;

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

    #[test]
    fn test_1() {
        let contour = vec![
            IntPoint::new(0, 0),
            IntPoint::new(10, 10),
            IntPoint::new(10, 0),
            IntPoint::new(0, 10),
        ];

        let mut rev_contour = contour.clone();
        rev_contour.reverse();

        let r0 = contour.simplify(FillRule::NonZero, Default::default());
        assert_eq!(r0.len(), 2);

        let r1 = rev_contour.simplify(FillRule::NonZero, Default::default());
        assert_eq!(r1.len(), 2);
    }

    #[test]
    fn test_2() {
        // 2 outer contours, not intersections but share point
        let contour = vec![
            IntPoint::new(-2, -1),
            IntPoint::new(0, 0),
            IntPoint::new(2, 1),
            IntPoint::new(2, -1),
            IntPoint::new(0, 0),
            IntPoint::new(-2, 1),
        ];

        let mut rev_contour = contour.clone();
        rev_contour.reverse();

        let r0 = contour.simplify(FillRule::NonZero, IntOverlayOptions::keep_all_points());
        assert_eq!(r0.len(), 2);

        let r1 = rev_contour.simplify(FillRule::NonZero, IntOverlayOptions::keep_all_points());
        assert_eq!(r1.len(), 2);
    }

    #[test]
    fn test_3() {
        // outer and inner contours, not intersections but share point
        let contour = vec![
            IntPoint::new(0, 0),
            IntPoint::new(-3, 2),
            IntPoint::new(-3, -2),
            IntPoint::new(0, 0),
            IntPoint::new(-2, -1),
            IntPoint::new(-2, 1),
        ];

        let mut rev_contour = contour.clone();
        rev_contour.reverse();

        let r0 = contour.simplify(FillRule::NonZero, Default::default());
        assert_eq!(r0.len(), 1);

        let r1 = rev_contour.simplify(FillRule::NonZero, Default::default());
        assert_eq!(r1.len(), 1);
    }

    #[test]
    fn test_4() {
        // 2 inner contours (one inside other), not intersections but share point
        let contour = vec![
            IntPoint::new(0, 0),
            IntPoint::new(-3, 2),
            IntPoint::new(-3, -2),
            IntPoint::new(0, 0),
            IntPoint::new(-2, 1),
            IntPoint::new(-2, -1),
        ];

        let mut rev_contour = contour.clone();
        rev_contour.reverse();

        let r0 = contour.simplify(FillRule::NonZero, Default::default());
        assert_eq!(r0.len(), 1);
        assert_eq!(r0[0].len(), 1);
        assert_eq!(r0[0][0].len(), 3);

        let r1 = rev_contour.simplify(FillRule::NonZero, Default::default());
        assert_eq!(r1.len(), 1);
        assert_eq!(r1[0][0].len(), 3);
    }
}
