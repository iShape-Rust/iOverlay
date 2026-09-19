use i_overlay::mesh::{
    float::{
        outline::offset::OutlineOffset,
        stroke::offset::StrokeOffset,
        style::{LineCap, LineJoin, OutlineStyle, StrokeStyle},
        variable_stroke::{StrokeVertex, VariableStrokeStyle, offset::VariableStrokeOffset},
    },
    math::MathMode,
};

macro_rules! non_finite_style_angles {
    ($name:ident, $scalar:ty) => {
        #[test]
        fn $name() {
            let path = [[0.0 as $scalar, 0.0], [10.0, 0.0], [12.0, 8.0]];
            let variable = path.map(|point| StrokeVertex::new(point, 2.0));
            let minimum = (0.01 * core::f64::consts::PI) as $scalar;
            for math in [MathMode::Integer, MathMode::Float] {
                for (angle, round_control, miter_control) in [
                    (<$scalar>::NAN, minimum, minimum),
                    (<$scalar>::NEG_INFINITY, minimum, minimum),
                    (
                        <$scalar>::INFINITY,
                        (0.25 * core::f64::consts::PI) as $scalar,
                        (0.99 * core::f64::consts::PI) as $scalar,
                    ),
                ] {
                    let stroke = |a| {
                        path.stroke(
                            StrokeStyle::new(2.0)
                                .math(math)
                                .line_join(LineJoin::Round(a))
                                .start_cap(LineCap::Round(a))
                                .end_cap(LineCap::Round(a)),
                            false,
                        )
                    };
                    let expected = stroke(round_control);
                    assert!(!expected.is_empty());
                    assert_eq!(stroke(angle), expected);

                    for (join, control) in [
                        (LineJoin::Miter(angle), LineJoin::Miter(miter_control)),
                        (LineJoin::Round(angle), LineJoin::Round(round_control)),
                    ] {
                        let expected = path.outline(&OutlineStyle::new(1.0).math(math).line_join(control));
                        assert!(!expected.is_empty());
                        assert_eq!(
                            path.outline(&OutlineStyle::new(1.0).math(math).line_join(join)),
                            expected
                        );
                    }

                    let stroke =
                        |a| variable.variable_stroke(VariableStrokeStyle::new().math(math).round_angle(a));
                    let expected = stroke(round_control);
                    assert!(!expected.is_empty());
                    assert_eq!(stroke(angle), expected);
                }
            }
        }
    };
}

non_finite_style_angles!(non_finite_angles_f32, f32);
non_finite_style_angles!(non_finite_angles_f64, f64);
