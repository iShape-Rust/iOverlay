use i_float::int::{angle::Angle, point::IntPoint};
use i_overlay::mesh::float::{
    outline::offset::OutlineOffset,
    stroke::offset::StrokeOffset,
    style::{LineJoin, OutlineStyle, StrokeStyle},
};
use i_overlay::mesh::int::{
    outline::offset::IntOutlineOffset,
    stroke::offset::IntStrokeOffset,
    style::{IntLineJoin, IntOutlineStyle, IntStrokeStyle},
};
use i_overlay::mesh::math::MathMode;

#[test]
fn miter_on_nearly_collinear_path_stays_near_input() {
    let path = [
        IntPoint::new(-879_382_i32, -1_393),
        IntPoint::new(0, 0),
        IntPoint::new(7_914_439, 12_537),
    ];
    let width = 200_000;
    let style = IntStrokeStyle::new(width).line_join(IntLineJoin::Miter(Angle::from_radians(0.1).unwrap()));

    path.validate_stroke(&style).unwrap();
    let shapes = path.stroke(&style, false).unwrap();
    assert_eq!(shapes.len(), 1);

    // The path is almost straight; a full width of padding is conservative.
    // Previously release emitted the spurious point (426_437_379, 7_579_018),
    // while debug panicked when constructing the peak.
    let x_bounds = path[0].x - width..=path[2].x + width;
    let y_bounds = path[0].y - width..=path[2].y + width;
    for point in shapes.iter().flatten().flatten() {
        assert!(
            x_bounds.contains(&point.x) && y_bounds.contains(&point.y),
            "miter vertex {point:?} escaped expected bounds: x={x_bounds:?}, y={y_bounds:?}"
        );
    }
}

#[test]
fn nearly_collinear_shrink_outline_stays_inside_input_bounds() {
    let contour = [
        IntPoint::new(-879_382_i32, -1_393),
        IntPoint::new(0, 0),
        IntPoint::new(7_914_439, 12_537),
        IntPoint::new(-2_000_000, 6_000_000),
    ];
    let style =
        IntOutlineStyle::new(-100_000).line_join(IntLineJoin::Miter(Angle::from_radians(0.1).unwrap()));
    contour.validate_outline(&style).unwrap();
    let shapes = contour.outline(&style).unwrap();
    assert_eq!(shapes.len(), 1);
    for point in shapes.iter().flatten().flatten() {
        assert!((-2_000_000..=7_914_439).contains(&point.x));
        assert!((-1_393..=6_000_000).contains(&point.y));
    }
}

#[test]
fn both_math_modes_bevel_turns_below_five_degrees() {
    // Turns on either side of 5 degrees, for both traversal directions.
    for (dy, bevel_expected) in [(8_700, true), (8_800, false)] {
        let path = [
            IntPoint::new(-100_000_i32, 0),
            IntPoint::new(0, 0),
            IntPoint::new(100_000, dy),
        ];
        for path in [path, [path[2], path[1], path[0]]] {
            for math in [MathMode::Integer, MathMode::Float] {
                let base = IntStrokeStyle::new(20_000).math(math);
                let bevel = path.stroke(&base, false).unwrap();
                let miter = path
                    .stroke(
                        &base.line_join(IntLineJoin::Miter(Angle::from_radians(0.1).unwrap())),
                        false,
                    )
                    .unwrap();
                assert_eq!(miter == bevel, bevel_expected, "dy={dy}, math={math:?}");
            }
        }
    }
}

#[test]
fn nearly_collinear_float_miter_stays_in_bounds_after_translation() {
    let path = [
        IntPoint::new(-556_890_i32, -800_383),
        IntPoint::new(0, 0),
        IntPoint::new(1_670_672, 2_401_151),
    ];
    let width = 200_000;
    // The translated input passes validation, but the old Float intersection
    // produced an out-of-range peak and panicked in debug builds.
    for shift in [0, 1_071_140_673] {
        let path = path.map(|p| IntPoint::new(p.x, p.y + shift));
        for math in [MathMode::Integer, MathMode::Float] {
            let style = IntStrokeStyle::new(width)
                .math(math)
                .line_join(IntLineJoin::Miter(Angle::from_radians(3.0).unwrap()));
            path.validate_stroke(&style).unwrap();
            let shapes = path.stroke(&style, false).unwrap();
            assert_eq!(shapes.len(), 1);
            let x_bounds = path[0].x - width..=path[2].x + width;
            let y_bounds = path[0].y - width..=path[2].y + width;
            for point in shapes.iter().flatten().flatten() {
                assert!(
                    x_bounds.contains(&point.x) && y_bounds.contains(&point.y),
                    "math={math:?}, shift={shift}, unexpected miter vertex {point:?}"
                );
            }
        }
    }
}

#[test]
fn only_integer_math_clips_corners_sharper_than_five_degrees() {
    // Interior angle approximately 4.9 degrees; radius 10_000.
    // A 5-degree clipped miter reaches at most r / sin(2.5 degrees),
    // about 229_256 units. The ordinary 4.9-degree miter reaches over 233_000.
    let path = [
        IntPoint::new(-100_000_i32, 0),
        IntPoint::new(0, 0),
        IntPoint::new(-100_000, 8_573),
    ];
    for math in [MathMode::Integer, MathMode::Float] {
        let style = IntStrokeStyle::new(20_000)
            .math(math)
            .line_join(IntLineJoin::Miter(Angle::from_radians(0.01).unwrap()));
        path.validate_stroke(&style).unwrap();
        let shapes = path.stroke(&style, false).unwrap();
        let max_x = shapes.iter().flatten().flatten().map(|p| p.x).max().unwrap();
        assert!(max_x > 200_000, "the sharp corner must retain a miter");
        assert_eq!(
            max_x < 230_000,
            math == MathMode::Integer,
            "math={math:?}, max_x={max_x}"
        );
    }
}

#[test]
fn custom_miter_cutoff_applies_to_integer_stroke_and_outline() {
    // The middle turn is about 2.86 degrees, between the two custom cutoffs.
    let path = [
        IntPoint::new(-100_000_i32, 0),
        IntPoint::new(0, 0),
        IntPoint::new(100_000, 5_000),
    ];
    let contour = [path[0], path[1], path[2], IntPoint::new(0, 100_000)];
    let angle = |degrees: f64| Angle::from_radians(degrees.to_radians()).unwrap();
    for math in [MathMode::Integer, MathMode::Float] {
        let stroke = IntStrokeStyle::new(20_000)
            .math(math)
            .line_join(IntLineJoin::Miter(angle(10.0)));
        let build_stroke = |cutoff| {
            path.stroke(&stroke.clone().miter_min_turn(cutoff), false)
                .unwrap()
        };
        let low = build_stroke(angle(1.0));
        assert!(!low.is_empty());
        assert_eq!(low, build_stroke(angle(0.0)));
        assert_ne!(low, build_stroke(angle(10.0)));
        assert_eq!(
            build_stroke(angle(10.0)),
            path.stroke(&stroke.clone().line_join(IntLineJoin::Bevel), false)
                .unwrap()
        );
        assert_eq!(
            build_stroke(Angle::from_bits(u32::MAX)),
            build_stroke(angle(180.0))
        );

        let outline = IntOutlineStyle::new(10_000)
            .math(math)
            .line_join(IntLineJoin::Miter(angle(10.0)));
        let build_outline = |cutoff| contour.outline(&outline.miter_min_turn(cutoff)).unwrap();
        let low = build_outline(angle(1.0));
        assert!(!low.is_empty());
        assert_eq!(low, build_outline(angle(0.0)));
        assert_ne!(low, build_outline(angle(10.0)));
    }
}

macro_rules! float_miter_cutoff {
    ($name:ident, $scalar:ty) => {
        #[test]
        fn $name() {
            let path = [
                [-100_000.0 as $scalar, 0.0],
                [0.0, 0.0],
                [100_000.0, 5_000.0],
            ];
            let contour = [path[0], path[1], path[2], [0.0, 100_000.0]];
            let angle = |degrees: f64| degrees.to_radians() as $scalar;
            for math in [MathMode::Integer, MathMode::Float] {
                let stroke = StrokeStyle::new(20_000.0)
                    .math(math)
                    .line_join(LineJoin::Miter(angle(10.0)));
                let build_stroke = |cutoff| {
                    path.stroke_fixed_scale(stroke.clone().miter_min_turn(cutoff), false, 1.0)
                        .unwrap()
                };
                let low = build_stroke(angle(1.0));
                assert!(!low.is_empty());
                assert_eq!(low, build_stroke(0.0));
                assert_ne!(low, build_stroke(angle(10.0)));
                assert_eq!(
                    build_stroke(angle(10.0)),
                    path.stroke_fixed_scale(stroke.clone().line_join(LineJoin::Bevel), false, 1.0)
                        .unwrap()
                );
                assert_eq!(build_stroke(<$scalar>::NAN), build_stroke(angle(5.0)));
                assert_eq!(build_stroke(<$scalar>::NEG_INFINITY), build_stroke(0.0));
                assert_eq!(
                    build_stroke(<$scalar>::INFINITY),
                    build_stroke(angle(180.0))
                );

                let build_outline = |cutoff| {
                    contour
                        .outline_fixed_scale(
                            &OutlineStyle::new(10_000.0)
                                .math(math)
                                .line_join(LineJoin::Miter(angle(10.0)))
                                .miter_min_turn(cutoff),
                            1.0,
                        )
                        .unwrap()
                };
                let low = build_outline(angle(1.0));
                assert!(!low.is_empty());
                assert_eq!(low, build_outline(0.0));
                assert_ne!(low, build_outline(angle(10.0)));
            }
        }
    };
}

float_miter_cutoff!(custom_miter_cutoff_f32, f32);
float_miter_cutoff!(custom_miter_cutoff_f64, f64);
