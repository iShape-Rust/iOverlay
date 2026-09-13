use i_float::int::{angle::Angle, point::IntPoint};
use i_overlay::mesh::int::{
    arc::ArcOptions,
    outline::offset::IntOutlineOffset,
    stroke::offset::IntStrokeOffset,
    style::{IntLineCap, IntLineJoin, IntOutlineStyle, IntStrokeStyle},
    variable_stroke::{
        IntStrokeVertex, IntVariableStrokeSource, IntVariableStrokeStyle, offset::IntVariableStrokeOffset,
    },
};
use i_shape::{flat::buffer::FlatContoursBuffer, int::area::Area};

macro_rules! engines {
    ($name:ident,$int:ty) => {
        #[test]
        fn $name() {
            let path = [IntPoint::<$int>::new(-4096, 0), IntPoint::new(4096, 0)];
            let mut style = IntStrokeStyle::new(2048);
            let butt = path.stroke(&style, false).unwrap();
            assert_eq!(butt.len(), 1);
            assert_eq!(butt.area() as i128, 8192 * 2048);
            style.start_cap = IntLineCap::Square;
            style.end_cap = IntLineCap::Square;
            assert_eq!(
                path.stroke(&style, false).unwrap().area() as i128,
                10240 * 2048
            );
            style.start_cap = IntLineCap::Round(ArcOptions::default());
            style.end_cap = style.start_cap.clone();
            let round = path.stroke(&style, false).unwrap();
            let capsule_area = round.area() as i128;
            assert!((8192 * 2048 + 2800000..8192 * 2048 + 3400000).contains(&capsule_area));
            let variable = path.map(|p| IntStrokeVertex::new(p, 2048));
            let variable_style = IntVariableStrokeStyle::new().arc(ArcOptions::default());
            assert_eq!(variable.variable_stroke(variable_style).unwrap(), round);
            let tapered = [
                IntStrokeVertex::new(path[0], 1024),
                IntStrokeVertex::new(path[1], 3072),
            ];
            assert_eq!(tapered.variable_stroke(variable_style).unwrap().len(), 1);
            let corner = [path[0], path[1], IntPoint::new(4096, 4096)];
            for join in [
                IntLineJoin::Bevel,
                IntLineJoin::Miter(Angle::from_bits(1 << 28)),
                IntLineJoin::Round(ArcOptions::default()),
            ] {
                style.join = join;
                let shapes = corner.stroke(&style, false).unwrap();
                assert_eq!(shapes.len(), 1);
                assert!(shapes.area() as i128 > 12000 * 2048);
            }
        }
    };
}
engines!(mesh_i16, i16);
engines!(mesh_i32, i32);
engines!(mesh_i64, i64);

#[test]
fn integer_strokes_preserve_precision_above_f64_exact_integers() {
    let shift = (1i64 << 57) + 123;
    let base = [
        IntPoint::new(-4096i64, 0),
        IntPoint::new(4096, 0),
        IntPoint::new(2048, 8192),
    ];
    let translated = base.map(|p| IntPoint::new(p.x + shift, p.y - shift));
    let undo = |mut shapes: Vec<Vec<Vec<IntPoint<i64>>>>| {
        for p in shapes.iter_mut().flatten().flatten() {
            p.x -= shift;
            p.y += shift;
        }
        shapes
    };
    for join in [
        IntLineJoin::Bevel,
        IntLineJoin::Miter(Angle::from_bits(1 << 28)),
        IntLineJoin::Round(ArcOptions::default()),
    ] {
        let style = IntStrokeStyle::new(2048)
            .line_join(join)
            .start_cap(IntLineCap::Round(ArcOptions::default()))
            .end_cap(IntLineCap::Round(ArcOptions::default()));
        assert_eq!(
            base.stroke(&style, false).unwrap(),
            undo(translated.stroke(&style, false).unwrap())
        );
        let outline = IntOutlineStyle::new(1024).line_join(join);
        assert_eq!(
            base.outline(&outline).unwrap(),
            undo(translated.outline(&outline).unwrap())
        );
    }
    let variable = base
        .into_iter()
        .zip([1024, 4096, 2048])
        .map(|(p, w)| IntStrokeVertex::new(p, w))
        .collect::<Vec<_>>();
    let shifted = translated
        .into_iter()
        .zip([1024, 4096, 2048])
        .map(|(p, w)| IntStrokeVertex::new(p, w))
        .collect::<Vec<_>>();
    assert_eq!(
        variable.variable_stroke(Default::default()).unwrap(),
        undo(shifted.variable_stroke(Default::default()).unwrap())
    );
}

#[test]
fn flat_stroke_outputs_replace_previous_geometry() {
    let path = [IntPoint::new(0, 0), IntPoint::new(8192, 0)];
    let style = IntStrokeStyle::new(2048);
    let mut output = FlatContoursBuffer::default();
    let mut expected = FlatContoursBuffer::default();
    expected.set_with_shapes(&path.stroke(&style, false).unwrap());
    path.stroke_into(&style, false, &mut output).unwrap();
    assert_eq!(output, expected);
    let empty: Vec<IntPoint> = vec![];
    empty.stroke_into(&style, false, &mut output).unwrap();
    assert!(output.points.is_empty() && output.ranges.is_empty());
    let path = path.map(|p| IntStrokeVertex::new(p, 2048));
    expected.set_with_shapes(&path.variable_stroke(Default::default()).unwrap());
    path.variable_stroke_into(Default::default(), &mut output)
        .unwrap();
    assert_eq!(output, expected);
    let empty: Vec<IntStrokeVertex> = vec![];
    empty
        .variable_stroke_into(Default::default(), &mut output)
        .unwrap();
    assert!(output.points.is_empty() && output.ranges.is_empty());
}

#[test]
fn validate_expanded_stroke_bounds_without_building() {
    let edge = (1i32 << 30) - 512;
    let path = [IntPoint::new(edge, 0), IntPoint::new(edge, 8192)];
    assert!(path.validate_stroke(&IntStrokeStyle::new(512)).is_ok());
    assert!(path.validate_stroke(&IntStrokeStyle::new(2048)).is_err());
    assert!(
        path.map(|p| IntStrokeVertex::new(p, 2048))
            .validate_variable_stroke()
            .is_err()
    );
    assert!(
        path.map(|p| IntStrokeVertex::new(p, 512))
            .validate_variable_stroke()
            .is_ok()
    );
}

#[test]
fn variable_resources_and_remaining_iterator_count() {
    let path = vec![
        IntStrokeVertex::new(IntPoint::new(0, 0), 2048),
        IntStrokeVertex::new(IntPoint::new(8192, 0), 4096),
    ];
    let expected = path.variable_stroke(Default::default()).unwrap();
    assert_eq!(
        path.as_slice().variable_stroke(Default::default()).unwrap(),
        expected
    );
    let paths = vec![path.clone(), Vec::new()];
    assert_eq!(paths.variable_stroke(Default::default()).unwrap(), expected);
    let mut iter = path.iter_variable_paths();
    assert!(iter.next().is_some());
    assert_eq!(iter.count(), 0);
    let mut iter = paths.iter_variable_paths();
    assert!(iter.next().is_some());
    assert_eq!(iter.count(), 1);
}

#[test]
fn i64_large_spans_use_wide_products_for_contacts_and_miters() {
    let unit = 1i64 << 56;
    let path = [
        IntPoint::new(-16 * unit, 0),
        IntPoint::new(0, 0),
        IntPoint::new(15 * unit, unit),
    ];
    let style = IntStrokeStyle::new(2 * unit).line_join(IntLineJoin::Miter(Angle::from_bits(1 << 28)));
    assert_eq!(path.stroke(&style, false).unwrap().len(), 1);
    let variable = path
        .into_iter()
        .zip([unit, 8 * unit, 2 * unit])
        .map(|(p, w)| IntStrokeVertex::new(p, w))
        .collect::<Vec<_>>();
    assert_eq!(variable.variable_stroke(Default::default()).unwrap().len(), 1);
}

#[test]
fn float_fixed_scale_matches_direct_integer_geometry() {
    use i_overlay::mesh::float::{
        outline::offset::OutlineOffset,
        stroke::offset::StrokeOffset,
        style::{LineCap, LineJoin, OutlineStyle, StrokeStyle},
        variable_stroke::{StrokeVertex, VariableStrokeStyle, offset::VariableStrokeOffset},
    };
    let int_path = [
        IntPoint::new(-4096, -4096),
        IntPoint::new(4096, -4096),
        IntPoint::new(4096, 4096),
        IntPoint::new(-4096, 4096),
    ];
    let float_path = int_path.map(|p| [p.x as f64, p.y as f64]);
    let points = |shapes: Vec<Vec<Vec<IntPoint>>>| {
        let mut pts = shapes
            .into_iter()
            .flatten()
            .flatten()
            .map(|p| (p.x, p.y))
            .collect::<Vec<_>>();
        pts.sort_unstable();
        pts
    };
    let float_points = |shapes: Vec<Vec<Vec<[f64; 2]>>>| {
        let mut pts = shapes
            .into_iter()
            .flatten()
            .flatten()
            .map(|p| {
                assert_eq!(p[0], p[0].round());
                assert_eq!(p[1], p[1].round());
                (p[0] as i32, p[1] as i32)
            })
            .collect::<Vec<_>>();
        pts.sort_unstable();
        pts
    };
    for (int_join, float_join) in [
        (IntLineJoin::Bevel, LineJoin::Bevel),
        (
            IntLineJoin::Miter(Angle::from_bits(1 << 29)),
            LineJoin::Miter(core::f64::consts::FRAC_PI_4),
        ),
        (
            IntLineJoin::Round(ArcOptions::default()),
            LineJoin::Round(core::f64::consts::FRAC_PI_4),
        ),
    ] {
        let int_style = IntOutlineStyle::new(1024).line_join(int_join);
        let float_style = OutlineStyle::new(1024.0).line_join(float_join.clone());
        assert_eq!(
            points(int_path.outline(&int_style).unwrap()),
            float_points(float_path.outline_fixed_scale(&float_style, 1.0).unwrap())
        );
        let int_style = IntStrokeStyle::new(2048)
            .line_join(int_join)
            .start_cap(IntLineCap::Round(ArcOptions::default()))
            .end_cap(IntLineCap::Square);
        let float_style = StrokeStyle::new(2048.0)
            .line_join(float_join)
            .start_cap(LineCap::Round(core::f64::consts::FRAC_PI_4))
            .end_cap(LineCap::Square);
        assert_eq!(
            points(int_path.stroke(&int_style, false).unwrap()),
            float_points(float_path.stroke_fixed_scale(float_style, false, 1.0).unwrap())
        );
    }
    let int_variable = int_path
        .into_iter()
        .zip([1024, 4096, 2048, 1024])
        .map(|(p, w)| IntStrokeVertex::new(p, w))
        .collect::<Vec<_>>();
    let float_variable = float_path
        .into_iter()
        .zip([1024.0, 4096.0, 2048.0, 1024.0])
        .map(|(p, w)| StrokeVertex::new(p, w))
        .collect::<Vec<_>>();
    assert_eq!(
        points(
            int_variable
                .variable_stroke(IntVariableStrokeStyle::new().arc(ArcOptions::default()))
                .unwrap()
        ),
        float_points(
            float_variable
                .variable_stroke_fixed_scale(
                    VariableStrokeStyle::new().round_angle(core::f64::consts::FRAC_PI_4),
                    1.0
                )
                .unwrap()
        )
    );
}
