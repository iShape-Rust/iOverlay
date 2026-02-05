#[cfg(test)]
mod tests {
    use i_float::int::point::IntPoint;
    use i_overlay::core::fill_rule::FillRule;
    use i_overlay::core::overlay::{IntOverlayOptions, Overlay};
    use i_overlay::core::overlay_rule::OverlayRule;
    use i_shape::int::area::Area;

    // Four boxes that form a shape with a hole, where the exterior
    // contour and hole contour should share a point at (2,2):
    //
    //      0  1  2  3
    // y=3  +--+--+
    //      |  |  |
    // y=2  +--+--*--+     * = shared point (2,2)
    //      |  |##|  |     ## = hole
    // y=1  +--+--+--+
    //      |  |  |  |
    // y=0  +--+--+--+
    //
    // Box 1: (0,0)-(3,1) bottom strip
    // Box 2: (0,1)-(1,2) middle-left
    // Box 3: (2,1)-(3,2) middle-right
    // Box 4: (0,2)-(2,3) top-left
    //
    // The hole is at (1,1)-(2,2). The point (2,2) lies on both the
    // exterior contour and the hole contour.
    //
    // Expected exterior contour (6 points, CCW):
    //   (0,0) → (3,0) → (3,2) → (2,2) → (2,3) → (0,3)
    //
    // Expected hole contour (4 points, CW):
    //   (1,1) → (2,1) → (2,2) → (1,2)
    //
    // BUG: The library currently produces a single merged contour that
    // visits (2,2) twice in a figure-8 instead of two separate contours:
    //   (0,0)→(3,0)→(3,2)→(2,2)→(2,1)→(1,1)→(1,2)→(2,2)→(2,3)→(0,3)

    /// Create a CCW rectangle contour from (x1,y1) to (x2,y2).
    fn rect(x1: i32, y1: i32, x2: i32, y2: i32) -> Vec<IntPoint> {
        vec![
            IntPoint::new(x1, y1),
            IntPoint::new(x2, y1),
            IntPoint::new(x2, y2),
            IntPoint::new(x1, y2),
        ]
    }

    fn subject_contours() -> Vec<Vec<IntPoint>> {
        vec![
            rect(0, 0, 3, 1),
            rect(0, 1, 1, 2),
            rect(2, 1, 3, 2),
            rect(0, 2, 2, 3),
        ]
    }

    fn overlay(fill_rule: FillRule) -> Vec<Vec<Vec<IntPoint>>> {
        let contours = subject_contours();
        let mut overlay = Overlay::with_contours_custom(
            &contours,
            &[],
            IntOverlayOptions::ogc(),
            Default::default(),
        );
        overlay.overlay(OverlayRule::Subject, fill_rule)
    }

    fn contour_contains(contour: &[IntPoint], point: IntPoint) -> bool {
        contour.iter().any(|p| *p == point)
    }

    #[test]
    fn test_input_winding_order() {
        // All input rectangles must be CCW (negative area_two).
        for contour in &subject_contours() {
            assert!(
                contour.area_two() < 0,
                "input contour must be counter-clockwise, got area_two={}", contour.area_two()
            );
        }
    }

    #[test]
    fn test_shared_point_even_odd() {
        let result = overlay(FillRule::EvenOdd);

        assert_eq!(result.len(), 1, "expected 1 shape");
        assert_eq!(result[0].len(), 2, "expected 2 contours (exterior + hole)");

        let shared = IntPoint::new(2, 2);
        assert!(
            contour_contains(&result[0][0], shared),
            "exterior contour must contain (2,2)"
        );
        assert!(
            contour_contains(&result[0][1], shared),
            "hole contour must contain (2,2)"
        );
    }

    #[test]
    fn test_shared_point_non_zero() {
        let result = overlay(FillRule::NonZero);

        assert_eq!(result.len(), 1, "expected 1 shape");
        assert_eq!(result[0].len(), 2, "expected 2 contours (exterior + hole)");

        let shared = IntPoint::new(2, 2);
        assert!(
            contour_contains(&result[0][0], shared),
            "exterior contour must contain (2,2)"
        );
        assert!(
            contour_contains(&result[0][1], shared),
            "hole contour must contain (2,2)"
        );
    }

    #[test]
    fn test_shared_point_positive() {
        let result = overlay(FillRule::Positive);

        assert_eq!(result.len(), 1, "expected 1 shape");
        assert_eq!(result[0].len(), 2, "expected 2 contours (exterior + hole)");

        let shared = IntPoint::new(2, 2);
        assert!(
            contour_contains(&result[0][0], shared),
            "exterior contour must contain (2,2)"
        );
        assert!(
            contour_contains(&result[0][1], shared),
            "hole contour must contain (2,2)"
        );
    }

    #[test]
    fn test_shared_point_negative() {
        let result = overlay(FillRule::Negative);

        // All input contours are CCW (positive winding), so Negative
        // fill rule produces no filled regions.
        assert_eq!(result.len(), 0, "expected 0 shapes for Negative fill rule");
    }
}
