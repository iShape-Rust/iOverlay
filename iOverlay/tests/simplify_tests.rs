#[cfg(test)]
mod tests {
    use i_float::int::point::IntPoint;
    use i_shape::int::shape::IntShape;
    use i_overlay::core::fill_rule::FillRule;
    use i_overlay::core::overlay::{ContourDirection, Overlay, ShapeType};
    use i_overlay::core::overlay_rule::OverlayRule;
    use i_overlay::core::simplify::Simplify;
    use i_overlay::core::solver::{Precision, Solver};

    #[test]
    fn test_0() {
        let paths =
            [
                [
                    IntPoint::new(10614, 4421),
                    IntPoint::new(10609, 4421),
                    IntPoint::new(10609, 4415),
                    IntPoint::new(10614, 4415)
                ].to_vec()
            ].to_vec();

        let simplified = paths.simplify(FillRule::NonZero, ContourDirection::CounterClockwise, true, 0);

        assert_eq!(simplified.len(), 1);
        assert_eq!(simplified[0].len(), 1);
    }

    #[test]
    fn test_1() {
        let paths =
            [
                square(IntPoint::new(-10, -10)),
                square(IntPoint::new(-10, 0)),
                square(IntPoint::new(-10, 10)),
                square(IntPoint::new(0, -10)),
                square(IntPoint::new(0, 10)),
                square(IntPoint::new(10, -10)),
                square(IntPoint::new(10, 0)),
                square(IntPoint::new(10, 10))
            ].to_vec();

        let simplified = paths.simplify(FillRule::NonZero, ContourDirection::CounterClockwise, true, 0);

        assert_eq!(simplified.len(), 1);
        assert_eq!(simplified[0].len(), 2);
    }

    #[test]
    fn test_2() {
        let shapes =
            [
                square_shape(IntPoint::new(-10, -10)),
                square_shape(IntPoint::new(-10, 0)),
                square_shape(IntPoint::new(-10, 10)),
                square_shape(IntPoint::new(0, -10)),
                square_shape(IntPoint::new(0, 10)),
                square_shape(IntPoint::new(10, -10)),
                square_shape(IntPoint::new(10, 0)),
                square_shape(IntPoint::new(10, 10))
            ].to_vec();

        let simplified = shapes.simplify(FillRule::NonZero, ContourDirection::CounterClockwise, true, 0);

        assert_eq!(simplified.len(), 1);
        assert_eq!(simplified[0].len(), 2);
    }

    #[test]
    fn test_3() {
        let path =
            [
                IntPoint::new(0, 0),
                IntPoint::new(3, 1),
                IntPoint::new(0, 3),
                IntPoint::new(3, 0)
            ].to_vec();


        let mut overlay = Overlay::new(4);
        overlay.add_contour(&path, ShapeType::Subject);

        let mut solver = Solver::default();
        solver.precision = Precision::ABSOLUTE;

        let simple_0 = overlay.clone().into_graph_with_solver(FillRule::NonZero, solver).extract_shapes(OverlayRule::Subject);

        solver.precision = Precision::MEDIUM;
        let simple_1 = overlay.into_graph_with_solver(FillRule::NonZero, solver).extract_shapes(OverlayRule::Subject);

        assert_eq!(simple_0.len(), 1);
        assert_eq!(simple_0[0].len(), 1);

        assert_eq!(simple_1.len(), 1);
    }

    #[test]
    fn test_4() {
        let paths = vec![
            vec![
                IntPoint::new(-5, 0),
                IntPoint::new(0, 0),
                IntPoint::new(0, 5)
            ],
            vec![
                IntPoint::new(-3, 2),
                IntPoint::new(-1, 2),
                IntPoint::new(-1, 1)
            ]
        ];

        let simple = paths.simplify(FillRule::NonZero, ContourDirection::CounterClockwise, false, 0);

        assert_eq!(simple.len(), 1);
        assert_eq!(simple[0].len(), 2);
        assert_eq!(simple[0][0].len(), 4);
        assert_eq!(simple[0][1].len(), 3);
    }

    fn square(pos: IntPoint) -> Vec<IntPoint> {
        [
            IntPoint::new(-5 + pos.x, -5 + pos.y),
            IntPoint::new(-5 + pos.x, 5 + pos.y),
            IntPoint::new(5 + pos.x, 5 + pos.y),
            IntPoint::new(5 + pos.x, -5 + pos.y)
        ].to_vec()
    }

    fn square_shape(pos: IntPoint) -> IntShape {
        [square(pos)].to_vec()
    }
}