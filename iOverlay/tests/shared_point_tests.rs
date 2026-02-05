#[cfg(test)]
mod tests {
    use i_float::int::point::IntPoint;
    use i_overlay::core::fill_rule::FillRule;
    use i_overlay::core::overlay::{Overlay, ShapeType};
    use i_overlay::core::overlay_rule::OverlayRule;

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
    // Expected exterior contour (6 points):
    //   (0,0) → (3,0) → (3,2) → (2,2) → (2,3) → (0,3)
    //
    // Expected hole contour (4 points):
    //   (1,1) → (2,1) → (2,2) → (1,2)
    //
    // BUG: The library currently produces a single merged contour that
    // visits (2,2) twice in a figure-8 instead of two separate contours:
    //   (0,0)→(3,0)→(3,2)→(2,2)→(2,1)→(1,1)→(1,2)→(2,2)→(2,3)→(0,3)

    fn rect(x1: i32, y1: i32, x2: i32, y2: i32) -> Vec<IntPoint> {
        vec![
            IntPoint::new(x1, y1),
            IntPoint::new(x2, y1),
            IntPoint::new(x2, y2),
            IntPoint::new(x1, y2),
        ]
    }

    fn overlay() -> Overlay {
        let mut overlay = Overlay::new(16);

        overlay.add_contour(&rect(0, 0, 3, 1), ShapeType::Subject);
        overlay.add_contour(&rect(0, 1, 1, 2), ShapeType::Subject);
        overlay.add_contour(&rect(2, 1, 3, 2), ShapeType::Subject);
        overlay.add_contour(&rect(0, 2, 2, 3), ShapeType::Subject);

        overlay
    }

    fn contour_contains(contour: &[IntPoint], point: IntPoint) -> bool {
        contour.iter().any(|p| *p == point)
    }

    #[test]
    fn test_shared_point_even_odd() {
        let mut buffer = Default::default();

        let result = overlay()
            .build_graph_view(FillRule::EvenOdd)
            .unwrap()
            .extract_shapes(OverlayRule::Subject, &mut buffer);

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
        let mut buffer = Default::default();

        let result = overlay()
            .build_graph_view(FillRule::NonZero)
            .unwrap()
            .extract_shapes(OverlayRule::Subject, &mut buffer);

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
        let mut buffer = Default::default();

        let result = overlay()
            .build_graph_view(FillRule::Positive)
            .unwrap()
            .extract_shapes(OverlayRule::Subject, &mut buffer);

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
        let mut buffer = Default::default();

        let result = overlay()
            .build_graph_view(FillRule::Negative)
            .unwrap()
            .extract_shapes(OverlayRule::Subject, &mut buffer);

        // All contours are CCW (positive winding), so Negative fill rule
        // produces no filled regions.
        assert_eq!(result.len(), 0, "expected 0 shapes for Negative fill rule");
    }
}
