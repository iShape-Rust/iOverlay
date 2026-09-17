use i_float::int::{angle::Angle, point::IntPoint};
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
    let style = IntStrokeStyle::new(width).line_join(IntLineJoin::Miter(Angle::from_radians(0.1)));

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
    let style = IntOutlineStyle::new(-100_000).line_join(IntLineJoin::Miter(Angle::from_radians(0.1)));
    contour.validate_outline(&style).unwrap();
    let shapes = contour.outline(&style).unwrap();
    assert_eq!(shapes.len(), 1);
    for point in shapes.iter().flatten().flatten() {
        assert!((-2_000_000..=7_914_439).contains(&point.x));
        assert!((-1_393..=6_000_000).contains(&point.y));
    }
}

#[test]
fn only_integer_math_bevels_turns_below_five_degrees() {
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
                        &base.line_join(IntLineJoin::Miter(Angle::from_radians(0.1))),
                        false,
                    )
                    .unwrap();
                assert_eq!(
                    miter == bevel,
                    math == MathMode::Integer && bevel_expected,
                    "dy={dy}, math={math:?}"
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
            .line_join(IntLineJoin::Miter(Angle::from_radians(0.01)));
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
