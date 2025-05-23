#[cfg(test)]
mod tests {
    use i_float::int::point::IntPoint;
    use i_shape::int::shape::IntShape;
    use i_overlay::core::fill_rule::FillRule;
    use i_overlay::core::overlay::{ContourDirection, IntOverlayOptions, Overlay, ShapeType};
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

        let op = IntOverlayOptions {
            preserve_input_collinear: true,
            output_direction: ContourDirection::CounterClockwise,
            preserve_output_collinear: true,
            min_output_area: 0,
        };

        let simplified = paths.simplify(FillRule::NonZero, op);

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

        let op = IntOverlayOptions {
            preserve_input_collinear: true,
            output_direction: ContourDirection::CounterClockwise,
            preserve_output_collinear: true,
            min_output_area: 0,
        };

        let simplified = paths.simplify(FillRule::NonZero, op);

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

        let op = IntOverlayOptions {
            preserve_input_collinear: true,
            output_direction: ContourDirection::CounterClockwise,
            preserve_output_collinear: true,
            min_output_area: 0,
        };

        let simplified = shapes.simplify(FillRule::NonZero, op);

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

        let mut buffer = Default::default();

        let mut solver_0 = Solver::default();
        solver_0.precision = Precision::ABSOLUTE;

        let mut overlay_0 = Overlay::new_custom(4, Default::default(), solver_0);
        overlay_0.add_contour(&path, ShapeType::Subject);

        let mut solver_1 = Solver::default();
        solver_1.precision = Precision::MEDIUM;

        let mut overlay_1 = Overlay::new_custom(4, Default::default(), solver_1);
        overlay_1.add_contour(&path, ShapeType::Subject);

        let simple_0 = overlay_0
            .build_graph_view(FillRule::NonZero)
            .map_or(Default::default(), |graph|graph.extract_shapes(OverlayRule::Subject, &mut buffer));

        let simple_1 = overlay_1
            .build_graph_view(FillRule::NonZero)
            .map_or(Default::default(), |graph|graph.extract_shapes(OverlayRule::Subject, &mut buffer));

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

        let op = IntOverlayOptions {
            preserve_input_collinear: false,
            output_direction: ContourDirection::CounterClockwise,
            preserve_output_collinear: true,
            min_output_area: 0,
        };

        let simple = paths.simplify(FillRule::NonZero, op);

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