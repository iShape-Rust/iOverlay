use i_float::int::{angle::Angle, point::IntPoint};
use i_overlay::mesh::{
    float::{
        outline::offset::OutlineOffset,
        style::{LineJoin, OutlineStyle},
        variable_stroke::{StrokeVertex, VariableStrokeStyle, offset::VariableStrokeOffset},
    },
    int::{
        arc::ArcOptions,
        outline::offset::IntOutlineOffset,
        style::{IntLineJoin, IntOutlineStyle},
        variable_stroke::{IntStrokeVertex, IntVariableStrokeStyle, offset::IntVariableStrokeOffset},
    },
    math::MathMode,
};
use i_shape::{
    flat::{buffer::FlatContoursBuffer, float::FloatFlatContoursBuffer},
    int::area::Area,
};

#[test]
fn default_math_remains_integer() {
    assert_eq!(IntOutlineStyle::new(100).math, MathMode::Integer);
    assert_eq!(OutlineStyle::new(1.0).math, MathMode::Integer);
    assert_eq!(IntVariableStrokeStyle::new().math, MathMode::Integer);
    assert_eq!(VariableStrokeStyle::<f64>::new().math, MathMode::Integer);
}

macro_rules! area_checks {
    ($name:ident, $int:ty) => {
        #[test]
        fn $name() {
            let square = [
                IntPoint::<$int>::new(-2000, -2000),
                IntPoint::new(2000, -2000),
                IntPoint::new(2000, 2000),
                IntPoint::new(-2000, 2000),
            ];
            for math in [MathMode::Integer, MathMode::Float] {
                for offset in [-200, 0, 200] {
                    let style = IntOutlineStyle::new(offset)
                        .math(math)
                        .line_join(IntLineJoin::Miter(Angle::from_radians(0.1).unwrap()));
                    square.validate_outline(&style).unwrap();
                    let shapes = square.outline(&style).unwrap();
                    let side = 4000.0 + 2.0 * offset as f64;
                    assert!((shapes.area() as f64 - side * side).abs() <= 2.0 * side);
                    let mut output = FlatContoursBuffer::default();
                    square.outline_into(&style, &mut output).unwrap();
                    assert_eq!(
                        output.points,
                        shapes
                            .iter()
                            .flatten()
                            .flatten()
                            .copied()
                            .collect::<Vec<_>>()
                    );
                }
                let path = [
                    IntStrokeVertex::new(IntPoint::<$int>::new(0, 0), 400),
                    IntStrokeVertex::new(IntPoint::new(2400, 3200), 400),
                ];
                let style = IntVariableStrokeStyle::new().math(math).arc(ArcOptions {
                    max_step: Angle::from_radians(0.03).unwrap(),
                    ..Default::default()
                });
                path.validate_variable_stroke().unwrap();
                let shapes = path.variable_stroke(style).unwrap();
                let expected = 4000.0 * 400.0 + core::f64::consts::PI * 200.0 * 200.0;
                assert_eq!(shapes.len(), 1);
                assert!((shapes.area() as f64 - expected).abs() < 12000.0);
                let mut output = FlatContoursBuffer::default();
                path.variable_stroke_into(style, &mut output).unwrap();
                assert_eq!(
                    output.points,
                    shapes
                        .iter()
                        .flatten()
                        .flatten()
                        .copied()
                        .collect::<Vec<_>>()
                );
            }
        }
    };
}
area_checks!(areas_i16, i16);
area_checks!(areas_i32, i32);
area_checks!(areas_i64, i64);

#[test]
fn float_math_preserves_local_geometry_at_large_origins() {
    let outline = [
        IntPoint::new(0_i64, 0),
        IntPoint::new(8000, 0),
        IntPoint::new(9000, 6000),
        IntPoint::new(1000, 5000),
    ];
    let shift = 1_i64 << 60;
    let translated = outline.map(|p| IntPoint::new(p.x + shift, p.y - shift));
    for join in [
        IntLineJoin::Bevel,
        IntLineJoin::Miter(Angle::from_radians(0.1).unwrap()),
        IntLineJoin::Round(ArcOptions::default()),
    ] {
        let style = IntOutlineStyle::new(400).math(MathMode::Float).line_join(join);
        let expected = outline.outline(&style).unwrap();
        let mut actual = translated.outline(&style).unwrap();
        for p in actual.iter_mut().flatten().flatten() {
            p.x -= shift;
            p.y += shift;
        }
        assert_eq!(actual, expected);
    }
    let path = [
        IntStrokeVertex::new(outline[0], 400),
        IntStrokeVertex::new(outline[1], 1600),
        IntStrokeVertex::new(outline[2], 800),
    ];
    let translated =
        path.map(|v| IntStrokeVertex::new(IntPoint::new(v.point.x + shift, v.point.y - shift), v.width));
    let style = IntVariableStrokeStyle::new().math(MathMode::Float);
    let expected = path.variable_stroke(style).unwrap();
    let mut actual = translated.variable_stroke(style).unwrap();
    for p in actual.iter_mut().flatten().flatten() {
        p.x -= shift;
        p.y += shift;
    }
    assert_eq!(actual, expected);
}

#[test]
fn float_adapters_forward_math_and_replace_flat_output() {
    let outline = [[0.0, 0.0], [8.0, 0.0], [9.0, 6.0], [1.0, 5.0]];
    let path = [
        StrokeVertex::new([0.0, 0.0], 0.4),
        StrokeVertex::new([8.0, 0.0], 1.6),
        StrokeVertex::new([9.0, 6.0], 0.8),
    ];
    for math in [MathMode::Integer, MathMode::Float] {
        let style = OutlineStyle::new(0.4).math(math).line_join(LineJoin::Round(0.1));
        let shapes = outline.outline_fixed_scale(&style, 1000.0).unwrap();
        let mut output = FloatFlatContoursBuffer::default();
        outline
            .outline_fixed_scale_into(&style, 1000.0, &mut output)
            .unwrap();
        assert_eq!(
            output.points,
            shapes.iter().flatten().flatten().copied().collect::<Vec<_>>()
        );
        let int_outline = outline.map(|p| IntPoint::new((p[0] * 1000.0) as i32, (p[1] * 1000.0) as i32));
        let int_style = IntOutlineStyle::new(400)
            .math(math)
            .line_join(IntLineJoin::Round(ArcOptions {
                max_step: Angle::from_radians(0.1).unwrap(),
                ..Default::default()
            }));
        // The adapter may translate the origin, so compare translation-invariant areas.
        let expected = int_outline.outline(&int_style).unwrap().area() as f64 / 1e6;
        let area = |shapes: &Vec<Vec<Vec<[f64; 2]>>>| {
            shapes
                .iter()
                .flatten()
                .map(|p| {
                    (0..p.len())
                        .map(|i| p[i][0] * p[(i + 1) % p.len()][1] - p[i][1] * p[(i + 1) % p.len()][0])
                        .sum::<f64>()
                        / 2.0
                })
                .sum::<f64>()
                .abs()
        };
        assert!((area(&shapes) - expected).abs() < 0.02);
        let style = VariableStrokeStyle::new().math(math);
        let shapes = path.variable_stroke_fixed_scale(style, 1000.0).unwrap();
        path.variable_stroke_fixed_scale_into(style, 1000.0, &mut output)
            .unwrap();
        assert_eq!(
            output.points,
            shapes.iter().flatten().flatten().copied().collect::<Vec<_>>()
        );
        let int_path = path.map(|v| {
            IntStrokeVertex::new(
                IntPoint::new((v.point[0] * 1000.0) as i32, (v.point[1] * 1000.0) as i32),
                (v.width * 1000.0) as i32,
            )
        });
        let expected = int_path
            .variable_stroke(IntVariableStrokeStyle::new().math(math))
            .unwrap()
            .area() as f64
            / 1e6;
        assert!((area(&shapes) - expected).abs() < 0.02);
        let empty: [StrokeVertex<[f64; 2]>; 0] = [];
        empty.variable_stroke_into(style, &mut output);
        assert!(output.points.is_empty() && output.ranges.is_empty());
    }
}

#[cfg(feature = "variable_stroke_debug")]
#[test]
fn debug_geometry_matches_normal_build_for_both_modes() {
    let paths = vec![
        vec![],
        vec![IntStrokeVertex::new(IntPoint::new(0, 0), 1000)],
        vec![
            IntStrokeVertex::new(IntPoint::new(-2000, 0), 400),
            IntStrokeVertex::new(IntPoint::new(0, 0), 6000),
            IntStrokeVertex::new(IntPoint::new(100, 0), 100),
            IntStrokeVertex::new(IntPoint::new(2000, 2000), 4000),
        ],
    ];
    for math in [MathMode::Integer, MathMode::Float] {
        let style = IntVariableStrokeStyle::new().math(math);
        let expected = paths.variable_stroke(style).unwrap();
        let debug = paths.variable_stroke_debug(style, Default::default()).unwrap();
        assert_eq!(debug.shapes, expected);
        assert!(debug.edges.iter().any(|e| e.path_index == 1));
        assert!(debug.edges.iter().any(|e| e.path_index == 2));
        for (order, edge) in debug.edges.iter().enumerate() {
            assert_eq!(edge.order, order);
            assert_ne!(edge.a, edge.b);
        }
    }
}

#[test]
fn outline_preserves_hole_roles_in_both_modes() {
    let paths = vec![
        vec![
            IntPoint::new(-4000, -4000),
            IntPoint::new(4000, -4000),
            IntPoint::new(4000, 4000),
            IntPoint::new(-4000, 4000),
        ],
        vec![
            IntPoint::new(-1000, -1000),
            IntPoint::new(-1000, 1000),
            IntPoint::new(1000, 1000),
            IntPoint::new(1000, -1000),
        ],
    ];
    for math in [MathMode::Integer, MathMode::Float] {
        for offset in [-200, 0, 200] {
            let style = IntOutlineStyle::new(offset)
                .math(math)
                .line_join(IntLineJoin::Miter(Angle::from_radians(0.1).unwrap()));
            let shapes = paths.outline(&style).unwrap();
            assert_eq!(shapes.len(), 1);
            assert_eq!(shapes[0].len(), 2);
            let outer = 8000_i64 + 2 * offset as i64;
            let inner = 2000_i64 - 2 * offset as i64;
            assert_eq!(shapes.area(), outer * outer - inner * inner);
        }
    }
}

#[test]
fn outline_bounds_use_the_selected_miter_limit() {
    let path = [
        IntPoint::new(0_i16, 0),
        IntPoint::new(1000, 0),
        IntPoint::new(0, 1000),
    ];
    // Integer math clamps to 5 degrees; Float uses the requested 1.8 degrees.
    let style = IntOutlineStyle::new(300).line_join(IntLineJoin::Miter(Angle::from_radians(0.01).unwrap()));
    assert!(path.validate_outline(&style.math(MathMode::Integer)).is_ok());
    assert!(path.validate_outline(&style.math(MathMode::Float)).is_err());
}
