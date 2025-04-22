#[cfg(test)]
mod tests {
    use i_float::int::point::IntPoint;
    use i_overlay::core::fill_rule::FillRule;
    use i_overlay::core::overlay::{ContourDirection, IntOverlayOptions, Overlay};
    use i_overlay::core::simplify::Simplify;
    use i_shape::int::area::Area;
    use i_overlay::core::overlay_rule::OverlayRule;

    #[test]
    fn test_0() {
        let path = vec![
            IntPoint::new(-5, 0),
            IntPoint::new(0, -5),
            IntPoint::new(5, 0),
            IntPoint::new(0, 5),
        ];

        let op0 = IntOverlayOptions {
            preserve_input_collinear: false,
            output_direction: ContourDirection::CounterClockwise,
            preserve_output_collinear: false,
            min_output_area: 0,
        };

        let op1 = IntOverlayOptions {
            preserve_input_collinear: false,
            output_direction: ContourDirection::Clockwise,
            preserve_output_collinear: false,
            min_output_area: 0,
        };

        let r0 = &path.simplify(FillRule::NonZero, op0)[0][0];
        debug_assert!(r0.area_two() < 0);

        let r1 = &path.simplify(FillRule::NonZero, op1)[0][0];
        debug_assert!(r1.area_two() > 0);
    }

    #[test]
    fn test_1() {
        let path = vec![
            vec![
                IntPoint::new(-10, 0),
                IntPoint::new(0, -10),
                IntPoint::new(10, 0),
                IntPoint::new(0, 10),
            ],
            vec![
                IntPoint::new(-5, 0),
                IntPoint::new(0, 5),
                IntPoint::new(5, 0),
                IntPoint::new(0, -5),
            ],
        ];

        let op0 = IntOverlayOptions {
            preserve_input_collinear: false,
            output_direction: ContourDirection::CounterClockwise,
            preserve_output_collinear: false,
            min_output_area: 0,
        };

        let op1 = IntOverlayOptions {
            preserve_input_collinear: false,
            output_direction: ContourDirection::Clockwise,
            preserve_output_collinear: false,
            min_output_area: 0,
        };
        
        let r0 = &path.simplify(FillRule::NonZero, op0)[0];
        debug_assert!(r0[0].area_two() < 0);
        debug_assert!(r0[1].area_two() > 0);

        let r1 = &path.simplify(FillRule::NonZero, op1)[0];
        debug_assert!(r1[0].area_two() > 0);
        debug_assert!(r1[1].area_two() < 0);
    }

    #[test]
    fn test_2() {
        let path = vec![
            vec![
                IntPoint::new(-10, 0),
                IntPoint::new(0, -10),
                IntPoint::new(10, 0),
                IntPoint::new(0, 10),
            ],
            vec![
                IntPoint::new(-5, 0),
                IntPoint::new(0, 5),
                IntPoint::new(5, 0),
                IntPoint::new(0, -5),
            ],
        ];

        // test default behavior
        let r = Overlay::with_contours(&path, &[]).overlay(OverlayRule::Subject, FillRule::NonZero);
        debug_assert!(r[0][0].area_two() < 0);
        debug_assert!(r[0][1].area_two() > 0);
    }
}
