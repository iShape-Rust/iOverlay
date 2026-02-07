#[cfg(test)]
mod tests {
    use i_float::int::point::IntPoint;
    use i_overlay::core::fill_rule::FillRule;
    use i_overlay::core::overlay::{IntOverlayOptions, Overlay};
    use i_overlay::core::overlay_rule::OverlayRule;
    use i_shape::int::area::Area;

    fn rect(x1: i32, y1: i32, x2: i32, y2: i32) -> Vec<IntPoint> {
        let contour = vec![
            IntPoint::new(x1, y1),
            IntPoint::new(x2, y1),
            IntPoint::new(x2, y2),
            IntPoint::new(x1, y2),
        ];
        assert!(
            contour.area_two() < 0,
            "input contour must be counter-clockwise, got area_two={}",
            contour.area_two()
        );
        contour
    }

    fn run_overlay(
        shapes: &[Vec<IntPoint>],
        overlay_rule: OverlayRule,
        fill_rule: FillRule,
    ) -> Vec<Vec<Vec<IntPoint>>> {
        match overlay_rule {
            OverlayRule::Subject => {
                let mut ov =
                    Overlay::with_contours_custom(shapes, &[], IntOverlayOptions::ogc(), Default::default());
                ov.overlay(overlay_rule, fill_rule)
            }
            _ => {
                let mut ov = Overlay::with_contours_custom(
                    &shapes[0..1],
                    &shapes[1..],
                    IntOverlayOptions::ogc(),
                    Default::default(),
                );
                ov.overlay(overlay_rule, fill_rule)
            }
        }
    }

    fn run_test(shapes: &[Vec<IntPoint>], fill_rule: FillRule, assert_fn: impl Fn(&[Vec<Vec<IntPoint>>])) {
        for &overlay_rule in &[OverlayRule::Subject, OverlayRule::Union] {
            let result = run_overlay(shapes, overlay_rule, fill_rule);
            assert_fn(&result);
        }
    }

    fn assert_empty(result: &[Vec<Vec<IntPoint>>]) {
        assert_eq!(result.len(), 0, "expected 0 shapes, got {result:?}");
    }

    // ---------------------------------------------------------------
    // Tests: shared_point
    // ---------------------------------------------------------------

    //      0  1  2  3
    // y=3  +--+--+
    //      |     |
    // y=2  +--+--*--+     * = shared point (2,2)
    //      |  |##|  |     ## = hole
    // y=1  +--+--+--+
    //      |        |
    // y=0  +--+--+--+
    fn shared_point_shapes() -> Vec<Vec<IntPoint>> {
        vec![
            rect(0, 0, 3, 1),
            rect(0, 1, 1, 2),
            rect(2, 1, 3, 2),
            rect(0, 2, 2, 3),
        ]
    }

    // 1 shape, 2 contours
    //   exterior (CCW, 6 pts): (0,0)→(3,0)→(3,2)→(2,2)→(2,3)→(0,3)
    //   hole (CW, 4 pts):      (2,2)→(2,1)→(1,1)→(1,2)
    fn assert_shared_point(result: &[Vec<Vec<IntPoint>>]) {
        assert_eq!(result.len(), 1, "expected 1 shape, got {result:?}");
        assert_eq!(result[0].len(), 2, "expected 2 contours, got {:?}", result[0]);
        assert_eq!(
            result[0][0],
            vec![
                IntPoint::new(0, 0),
                IntPoint::new(3, 0),
                IntPoint::new(3, 2),
                IntPoint::new(2, 2),
                IntPoint::new(2, 3),
                IntPoint::new(0, 3),
            ],
            "exterior mismatch"
        );
        assert_eq!(
            result[0][1],
            vec![
                IntPoint::new(2, 2),
                IntPoint::new(2, 1),
                IntPoint::new(1, 1),
                IntPoint::new(1, 2),
            ],
            "hole mismatch"
        );
    }

    #[test]
    fn test_shared_point_even_odd() {
        run_test(&shared_point_shapes(), FillRule::EvenOdd, assert_shared_point);
    }

    #[test]
    fn test_shared_point_non_zero() {
        run_test(&shared_point_shapes(), FillRule::NonZero, assert_shared_point);
    }

    #[test]
    fn test_shared_point_positive() {
        run_test(&shared_point_shapes(), FillRule::Positive, assert_shared_point);
    }

    #[test]
    fn test_shared_point_negative() {
        run_test(&shared_point_shapes(), FillRule::Negative, assert_empty);
    }

    // ---------------------------------------------------------------
    // Tests: two_shapes_touching
    // ---------------------------------------------------------------

    //         0  1  2
    //   y=2   +--+
    //         |  |
    //   y=1   +--*--+    * = touching point (1,1)
    //            |  |
    //   y=0      +--+
    fn two_shapes_touching_shapes() -> Vec<Vec<IntPoint>> {
        vec![rect(0, 1, 1, 2), rect(1, 0, 2, 1)]
    }

    // 2 shapes, each a simple rectangle (4 pts, no holes)
    //   shape 0: (0,2)→(0,1)→(1,1)→(1,2)
    //   shape 1: (1,1)→(1,0)→(2,0)→(2,1)
    fn assert_two_shapes_touching(result: &[Vec<Vec<IntPoint>>]) {
        assert_eq!(result.len(), 2, "expected 2 shapes, got {result:?}");
        assert_eq!(result[0].len(), 1, "shape 0 should have 1 contour");
        assert_eq!(result[1].len(), 1, "shape 1 should have 1 contour");
        assert_eq!(
            result[0][0],
            vec![
                IntPoint::new(0, 2),
                IntPoint::new(0, 1),
                IntPoint::new(1, 1),
                IntPoint::new(1, 2),
            ],
            "shape 0 mismatch"
        );
        assert_eq!(
            result[1][0],
            vec![
                IntPoint::new(1, 1),
                IntPoint::new(1, 0),
                IntPoint::new(2, 0),
                IntPoint::new(2, 1),
            ],
            "shape 1 mismatch"
        );
    }

    #[test]
    fn test_two_shapes_touching_even_odd() {
        run_test(
            &two_shapes_touching_shapes(),
            FillRule::EvenOdd,
            assert_two_shapes_touching,
        );
    }

    #[test]
    fn test_two_shapes_touching_non_zero() {
        run_test(
            &two_shapes_touching_shapes(),
            FillRule::NonZero,
            assert_two_shapes_touching,
        );
    }

    #[test]
    fn test_two_shapes_touching_positive() {
        run_test(
            &two_shapes_touching_shapes(),
            FillRule::Positive,
            assert_two_shapes_touching,
        );
    }

    #[test]
    fn test_two_shapes_touching_negative() {
        run_test(&two_shapes_touching_shapes(), FillRule::Negative, assert_empty);
    }

    // ---------------------------------------------------------------
    // Tests: two_holes_sharing_vertices
    // ---------------------------------------------------------------

    //      0  1  2  3  4  5  6  7
    // y=3  +--+--+        +--+--+
    //      |     |        |     |
    // y=2  +--+--*--+--+--*--+--+   * = pinch points (2,2) and (5,2)
    //      |  |##|        |##|  |   ## = holes
    // y=1  +--+--+--+--+--+--+--+
    //      |                    |
    // y=0  +--+--+--+--+--+--+--+
    fn two_holes_sharing_shapes() -> Vec<Vec<IntPoint>> {
        vec![
            rect(0, 0, 7, 1),
            rect(0, 1, 1, 2),
            rect(2, 1, 5, 2),
            rect(6, 1, 7, 2),
            rect(0, 2, 2, 3),
            rect(5, 2, 7, 3),
        ]
    }

    // 1 shape with 3 contours (exterior + 2 holes)
    //   exterior (CCW, 8 pts): (0,0)→(7,0)→(7,3)→(5,3)→(5,2)→(2,2)→(2,3)→(0,3)
    //   hole 1 (CW, 4 pts):   (2,2)→(2,1)→(1,1)→(1,2)
    //   hole 2 (CW, 4 pts):   (5,2)→(6,2)→(6,1)→(5,1)
    fn assert_two_holes_sharing(result: &[Vec<Vec<IntPoint>>]) {
        assert_eq!(result.len(), 1, "expected 1 shape, got {result:?}");
        assert_eq!(result[0].len(), 3, "expected 3 contours, got {:?}", result[0]);
        assert_eq!(
            result[0][0],
            vec![
                IntPoint::new(0, 0),
                IntPoint::new(7, 0),
                IntPoint::new(7, 3),
                IntPoint::new(5, 3),
                IntPoint::new(5, 2),
                IntPoint::new(2, 2),
                IntPoint::new(2, 3),
                IntPoint::new(0, 3),
            ],
            "exterior mismatch"
        );
        assert_eq!(
            result[0][1],
            vec![
                IntPoint::new(2, 2),
                IntPoint::new(2, 1),
                IntPoint::new(1, 1),
                IntPoint::new(1, 2),
            ],
            "hole 1 mismatch"
        );
        assert_eq!(
            result[0][2],
            vec![
                IntPoint::new(5, 2),
                IntPoint::new(6, 2),
                IntPoint::new(6, 1),
                IntPoint::new(5, 1),
            ],
            "hole 2 mismatch"
        );
    }

    #[test]
    fn test_two_holes_sharing_even_odd() {
        run_test(
            &two_holes_sharing_shapes(),
            FillRule::EvenOdd,
            assert_two_holes_sharing,
        );
    }

    #[test]
    fn test_two_holes_sharing_non_zero() {
        run_test(
            &two_holes_sharing_shapes(),
            FillRule::NonZero,
            assert_two_holes_sharing,
        );
    }

    #[test]
    fn test_two_holes_sharing_positive() {
        run_test(
            &two_holes_sharing_shapes(),
            FillRule::Positive,
            assert_two_holes_sharing,
        );
    }

    #[test]
    fn test_two_holes_sharing_negative() {
        run_test(&two_holes_sharing_shapes(), FillRule::Negative, assert_empty);
    }

    // ---------------------------------------------------------------
    // Tests: three_holes_sharing_vertices
    // ---------------------------------------------------------------

    //       0  1  2  3  4  5  6  7
    //  y=4  +--+--+        +--+--+
    //       |     |        |     |
    //  y=3  +--+--*--+--+--*--+--+   * = (2,3) and (5,3)
    //       |  |##|        |##|  |   ## = holes 1 and 2
    //  y=2  +--+--*--+--+--*--+--+   * = (2,2) and (5,2)
    //       |     |########|     |   ######## = hole 3
    //  y=1  +--+--+--+--+--+--+--+
    //       |                    |
    //  y=0  +--+--+--+--+--+--+--+
    fn three_holes_sharing_shapes() -> Vec<Vec<IntPoint>> {
        vec![
            rect(0, 0, 7, 1),
            rect(0, 1, 2, 2),
            rect(5, 1, 7, 2),
            rect(0, 2, 1, 3),
            rect(2, 2, 5, 3),
            rect(6, 2, 7, 3),
            rect(0, 3, 2, 4),
            rect(5, 3, 7, 4),
        ]
    }

    // 2 shapes, both with no holes (center block splits off at pinch points)
    //   main shape (CCW, 16 pts): complex exterior with notches
    //   center block (CCW, 4 pts): (2,3)→(2,2)→(5,2)→(5,3)
    fn assert_three_holes_sharing(result: &[Vec<Vec<IntPoint>>]) {
        assert_eq!(result.len(), 2, "expected 2 shapes, got {result:?}");
        let (main, center) = if result[0][0].len() > result[1][0].len() {
            (&result[0], &result[1])
        } else {
            (&result[1], &result[0])
        };
        assert_eq!(main.len(), 1, "main shape should have 1 contour");
        assert_eq!(
            main[0],
            vec![
                IntPoint::new(0, 0),
                IntPoint::new(7, 0),
                IntPoint::new(7, 4),
                IntPoint::new(5, 4),
                IntPoint::new(5, 3),
                IntPoint::new(6, 3),
                IntPoint::new(6, 2),
                IntPoint::new(5, 2),
                IntPoint::new(5, 1),
                IntPoint::new(2, 1),
                IntPoint::new(2, 2),
                IntPoint::new(1, 2),
                IntPoint::new(1, 3),
                IntPoint::new(2, 3),
                IntPoint::new(2, 4),
                IntPoint::new(0, 4),
            ],
            "main shape mismatch"
        );
        assert_eq!(center.len(), 1, "center block should have 1 contour");
        assert_eq!(
            center[0],
            vec![
                IntPoint::new(2, 3),
                IntPoint::new(2, 2),
                IntPoint::new(5, 2),
                IntPoint::new(5, 3),
            ],
            "center block mismatch"
        );
    }

    #[test]
    fn test_three_holes_sharing_even_odd() {
        run_test(
            &three_holes_sharing_shapes(),
            FillRule::EvenOdd,
            assert_three_holes_sharing,
        );
    }

    #[test]
    fn test_three_holes_sharing_non_zero() {
        run_test(
            &three_holes_sharing_shapes(),
            FillRule::NonZero,
            assert_three_holes_sharing,
        );
    }

    #[test]
    fn test_three_holes_sharing_positive() {
        run_test(
            &three_holes_sharing_shapes(),
            FillRule::Positive,
            assert_three_holes_sharing,
        );
    }

    #[test]
    fn test_three_holes_sharing_negative() {
        run_test(&three_holes_sharing_shapes(), FillRule::Negative, assert_empty);
    }
}
