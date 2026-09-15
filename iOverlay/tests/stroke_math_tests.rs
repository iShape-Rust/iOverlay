use i_float::int::angle::Angle;
use i_float::int::point::IntPoint;
use i_overlay::mesh::float::{
    stroke::offset::StrokeOffset,
    style::{LineCap, LineJoin, StrokeStyle},
};
use i_overlay::mesh::int::{
    arc::ArcOptions,
    stroke::offset::IntStrokeOffset,
    style::{IntLineCap, IntLineJoin, IntStrokeStyle},
};
use i_overlay::mesh::math::MathMode;
use i_shape::flat::float::FloatFlatContoursBuffer;
use i_shape::int::area::Area;

#[test]
fn integer_default_is_preserved() {
    assert_eq!(IntStrokeStyle::new(1024).math, MathMode::Integer);
    assert_eq!(StrokeStyle::<[f64; 2]>::new(1.0).math, MathMode::Integer);
}

macro_rules! stroke_area {
    ($name:ident, $int:ty) => {
        #[test]
        fn $name() {
            let path = [IntPoint::<$int>::new(0, 0), IntPoint::new(2400, 3200)];
            for math in [MathMode::Integer, MathMode::Float] {
                for (cap, expected) in [
                    (IntLineCap::Butt, 1600000.0),
                    (IntLineCap::Square, 1760000.0),
                ] {
                    let style = IntStrokeStyle::new(400)
                        .math(math)
                        .start_cap(cap.clone())
                        .end_cap(cap);
                    path.validate_stroke(&style).unwrap();
                    let shapes = path.stroke(&style, false).unwrap();
                    assert_eq!(shapes.len(), 1);
                    let actual = shapes.area() as f64;
                    assert!(
                        (actual - expected).abs() < 12000.0,
                        "{math:?}: {actual} vs {expected}"
                    );
                }
            }
        }
    };
}
stroke_area!(diagonal_area_i16, i16);
stroke_area!(diagonal_area_i32, i32);
stroke_area!(diagonal_area_i64, i64);

#[test]
fn float_math_preserves_local_geometry_at_large_i64_origins() {
    let path = [
        IntPoint::new(0_i64, 0),
        IntPoint::new(3000, 4000),
        IntPoint::new(9000, 4000),
    ];
    let shift = 1_i64 << 60;
    let translated = path.map(|p| IntPoint::new(p.x + shift, p.y - shift));
    for join in [
        IntLineJoin::Bevel,
        IntLineJoin::Miter(Angle::from_radians(0.1)),
        IntLineJoin::Round(ArcOptions::default()),
    ] {
        let style = IntStrokeStyle::new(400)
            .math(MathMode::Float)
            .line_join(join)
            .start_cap(IntLineCap::Round(ArcOptions::default()))
            .end_cap(IntLineCap::Square);
        let expected = path.stroke(&style, false).unwrap();
        let mut actual = translated.stroke(&style, false).unwrap();
        for p in actual.iter_mut().flatten().flatten() {
            p.x -= shift;
            p.y += shift;
        }
        assert_eq!(actual, expected);
    }
}

#[test]
fn both_modes_support_lazy_float_input_and_reused_flat_output() {
    let paths = vec![
        vec![],
        vec![[0.0, 0.0], [3.0, 4.0], [3.0, 4.0], [7.0, 2.0]],
        vec![[1.0, 2.0], [5.0, 0.0]],
    ];
    let mut output = FloatFlatContoursBuffer::default();
    for math in [MathMode::Integer, MathMode::Float] {
        for closed in [false, true] {
            for join in [LineJoin::Bevel, LineJoin::Miter(0.1), LineJoin::Round(0.05)] {
                let style = StrokeStyle::new(1.0)
                    .math(math)
                    .line_join(join)
                    .start_cap(LineCap::Round(0.05))
                    .end_cap(LineCap::Square);
                let expected = paths.stroke_fixed_scale(style.clone(), closed, 10000.0).unwrap();
                paths
                    .stroke_fixed_scale_into(style.clone(), closed, 10000.0, &mut output)
                    .unwrap();
                let contours: Vec<_> = expected.iter().flatten().collect();
                assert_eq!(output.ranges.len(), contours.len());
                for (range, contour) in output.ranges.iter().zip(contours) {
                    assert_eq!(&output.points[range.clone()], contour);
                }
                let empty: Vec<[f64; 2]> = vec![];
                empty
                    .stroke_fixed_scale_into(style, closed, 10000.0, &mut output)
                    .unwrap();
                assert!(output.points.is_empty() && output.ranges.is_empty());
            }
        }
    }
}

#[test]
fn asymmetric_custom_end_cap_is_included_in_automatic_bounds() {
    let path = [[0.0, 0.0], [1.0, 0.0]];
    for math in [MathMode::Integer, MathMode::Float] {
        let style = StrokeStyle::new(2.0).math(math).end_cap(LineCap::Custom(
            vec![[0.0, -1.0], [1000.0, 0.0], [0.0, 1.0]].into(),
        ));
        let shapes = path.stroke(style, false);
        let max_x = shapes
            .iter()
            .flatten()
            .flatten()
            .map(|p| p[0])
            .fold(f64::NEG_INFINITY, f64::max);
        assert!((max_x - 1001.0).abs() < 0.01);
    }
}
