use i_float::int::point::IntPoint;
use i_overlay::core::overlay::{ContourDirection, IntOverlayOptions};
use i_overlay::mesh::int::outline::offset::{IntOutlineError, IntOutlineOffset};
use i_overlay::mesh::int::style::{IntLineJoin, IntOutlineStyle};
use i_shape::flat::buffer::{FlatContoursBuffer, FlatShapesBuffer};
use i_shape::int::area::Area;
use i_shape::int::shape::IntShapes;

fn rectangle(x0: i32, y0: i32, x1: i32, y1: i32) -> Vec<IntPoint> {
    vec![
        IntPoint::new(x0, y0),
        IntPoint::new(x1, y0),
        IntPoint::new(x1, y1),
        IntPoint::new(x0, y1),
    ]
}

macro_rules! check_engine {
    ($name:ident, $int:ty) => {
        #[test]
        fn $name() {
            let path = [
                IntPoint::new(0 as $int, 0),
                IntPoint::new(8192, 0),
                IntPoint::new(8192, 8192),
                IntPoint::new(0, 8192),
            ];
            for offset in [-1024, 0, 1024] {
                let result = path.outline(&IntOutlineStyle::new(offset)).unwrap();
                assert_eq!(result.len(), 1);
                let expected: i64 = if offset > 0 {
                    8192 * 8192 + 4 * 8192 * 1024 + 2 * 1024 * 1024
                } else if offset < 0 {
                    6144 * 6144
                } else {
                    8192 * 8192
                };
                assert_eq!(result.area() as i64, expected);
                assert_eq!(result[0][0].len(), if offset > 0 { 8 } else { 4 });
            }
            assert!(
                path.outline(&IntOutlineStyle::new(-4096))
                    .unwrap()
                    .is_empty()
            );
        }
    };
}

check_engine!(bevel_i16, i16);
check_engine!(bevel_i32, i32);
check_engine!(bevel_i64, i64);

#[test]
fn offsets_in_the_expected_i32_width_range() {
    for radius in [1 << 10, 1 << 12, 1 << 16] {
        let side = radius * 16;
        let result = rectangle(0, 0, side, side)
            .outline(&IntOutlineStyle::new(radius))
            .unwrap();
        let (s, r) = (i64::from(side), i64::from(radius));
        assert_eq!(result.area(), s * s + 4 * s * r + 2 * r * r);
    }
}

#[test]
fn holes_use_their_own_offset_and_collapse_without_affecting_other_holes() {
    let outer = rectangle(0, 0, 32768, 32768);
    let mut hole = rectangle(8192, 8192, 16384, 16384);
    hole.reverse();
    let mut small_hole = rectangle(2048, 2048, 4096, 4096);
    small_hole.reverse();
    let style = IntOutlineStyle::new(2048).inner_offset(1024);
    let expected = vec![outer.clone(), hole.clone()].outline(&style).unwrap();
    assert_eq!(expected[0].len(), 2);
    assert_eq!(
        expected.area(),
        32768_i64.pow(2) + 4 * 32768 * 2048 + 2 * 2048_i64.pow(2) - 6144_i64.pow(2)
    );
    for source in [
        vec![outer.clone(), hole.clone(), small_hole.clone()],
        vec![small_hole, outer, hole],
    ] {
        assert_eq!(source.outline(&style).unwrap(), expected);
    }
}

#[test]
fn area_filter_runs_after_union() {
    // Each overlapping input rectangle has area 8*1024²; their union is 12*1024².
    let source = [rectangle(0, 0, 4096, 2048), rectangle(2048, 0, 6144, 2048)];
    let style = IntOutlineStyle::new(0);
    let expected = source.outline(&style).unwrap();
    assert_eq!(expected.area(), 12 * 1024 * 1024);
    let mut options = IntOverlayOptions::default();
    options.min_output_area = 10 * 1024 * 1024;
    assert_eq!(source.outline_custom(&style, options).unwrap(), expected);
    options.min_output_area = 13 * 1024 * 1024;
    assert!(source.outline_custom(&style, options).unwrap().is_empty());
}

#[test]
fn resource_forms_and_flat_output_agree() {
    let path = rectangle(0, 0, 8192, 8192);
    let shape = vec![path.clone()];
    let shapes = vec![shape.clone()];
    let mut contours = FlatContoursBuffer::default();
    contours.set_with_shapes(&shapes);
    let flat_shapes = FlatShapesBuffer {
        points: path.clone(),
        contour_ranges: vec![0..4],
        shape_ranges: vec![0..1],
    };
    let style = IntOutlineStyle::new(1024);
    let expected = path.outline(&style).unwrap();
    assert_eq!(path.as_slice().outline(&style).unwrap(), expected);
    let borrowed_paths = [path.as_slice()];
    assert_eq!(borrowed_paths.outline(&style).unwrap(), expected);
    assert_eq!(shape.outline(&style).unwrap(), expected);
    assert_eq!(shapes.outline(&style).unwrap(), expected);
    assert_eq!(contours.outline(&style).unwrap(), expected);
    assert_eq!(flat_shapes.outline(&style).unwrap(), expected);
    let mut output = FlatContoursBuffer::default();
    shapes.outline_into(&style, &mut output).unwrap();
    let mut expected_buffer = FlatContoursBuffer::default();
    expected_buffer.set_with_shapes(&expected);
    assert_eq!(output, expected_buffer);
    // Empty success must replace the previous output.
    let empty: Vec<IntPoint> = vec![];
    empty.outline_into(&style, &mut output).unwrap();
    assert!(output.points.is_empty() && output.ranges.is_empty());
}

#[test]
fn zero_offset_cleans_repeated_points_and_preserves_holes() {
    let mut outer = rectangle(0, 0, 8192, 8192);
    outer.insert(1, outer[0]);
    outer.push(outer[0]);
    let mut hole = rectangle(2048, 2048, 4096, 4096);
    hole.reverse();
    let source = vec![outer, hole];
    let result = source.outline(&IntOutlineStyle::new(0)).unwrap();
    assert_eq!(result[0].len(), 2);
    assert_eq!(result.area(), 8192_i64.pow(2) - 2048_i64.pow(2));
    assert!(result[0].iter().all(|path| path.len() == 4));
    let empty = vec![
        vec![],
        vec![IntPoint::new(0, 0); 4],
        vec![IntPoint::new(0, 0), IntPoint::new(1, 1), IntPoint::new(2, 2)],
    ];
    assert!(empty.outline(&IntOutlineStyle::new(1024)).unwrap().is_empty());
}

#[test]
fn output_direction_is_configurable() {
    let path = rectangle(0, 0, 8192, 8192);
    let options = IntOverlayOptions {
        output_direction: ContourDirection::Clockwise,
        ..Default::default()
    };
    let result = path.outline_custom(&IntOutlineStyle::new(1024), options).unwrap();
    assert!(result.area() < 0);
}

#[test]
fn errors_preserve_output_and_stubs_are_explicit() {
    let path = rectangle(0, 0, 8192, 8192);
    let mut output = FlatContoursBuffer::default();
    output.set_with_contour(&path);
    let saved = output.clone();
    for join in [IntLineJoin::Miter, IntLineJoin::Round] {
        let result = path.outline_into(&IntOutlineStyle::new(1024).line_join(join), &mut output);
        assert_eq!(result, Err(IntOutlineError::UnsupportedJoin(join)));
        assert_eq!(output, saved);
    }
}

#[test]
fn validation_rejects_out_of_range_operations_without_building() {
    for (path, offset) in [
        (rectangle(0, 0, i32::MAX, 8192), 0),
        (rectangle((1 << 30) - 8192, 0, (1 << 30) - 1, 8192), 1024),
        (rectangle(0, 0, 8192, 8192), i32::MIN),
    ] {
        assert_eq!(
            path.validate_outline(&IntOutlineStyle::new(offset)),
            Err(IntOutlineError::CoordinateOutOfRange)
        );
    }
}

#[test]
fn i64_translation_above_float_precision_preserves_geometry() {
    let path = [
        IntPoint::new(0_i64, 0),
        IntPoint::new(8192, 0),
        IntPoint::new(8192, 8192),
        IntPoint::new(0, 8192),
    ];
    let style = IntOutlineStyle::new(1024);
    let expected = path.outline(&style).unwrap();
    let shift = (1_i64 << 60) + 17;
    let translated = path.map(|p| IntPoint::new(p.x + shift, p.y - shift));
    let mut actual = translated.outline(&style).unwrap();
    for p in actual.iter_mut().flatten().flatten() {
        p.x -= shift;
        p.y += shift;
    }
    assert_eq!(actual, expected);
}

fn canonical(mut shapes: IntShapes<i32>) -> IntShapes<i32> {
    for shape in &mut shapes {
        for path in shape.iter_mut() {
            let first = path.iter().enumerate().min_by_key(|(_, p)| **p).unwrap().0;
            path.rotate_left(first);
        }
        shape[1..].sort();
    }
    shapes.sort();
    shapes
}

#[test]
fn diagonal_bevel_uses_scaled_perpendiculars() {
    // Every edge is a 3-4-5 triangle: the outward displacement is (±3072, ±4096).
    let path = [
        IntPoint::new(0, -12288),
        IntPoint::new(16384, 0),
        IntPoint::new(0, 12288),
        IntPoint::new(-16384, 0),
    ];
    let expected = vec![vec![vec![
        IntPoint::new(3072, -16384),
        IntPoint::new(19456, -4096),
        IntPoint::new(19456, 4096),
        IntPoint::new(3072, 16384),
        IntPoint::new(-3072, 16384),
        IntPoint::new(-19456, 4096),
        IntPoint::new(-19456, -4096),
        IntPoint::new(-3072, -16384),
    ]]];
    let result = path.outline(&IntOutlineStyle::new(5120)).unwrap();
    assert_eq!(canonical(result), canonical(expected));
}

#[test]
fn bevel_matches_existing_float_pipeline_on_exact_axis_normals() {
    use i_overlay::float::overlay::OverlayOptions;
    use i_overlay::mesh::float::outline::offset::OutlineOffset;
    use i_overlay::mesh::float::style::OutlineStyle;
    let path = vec![
        IntPoint::new(0, 0),
        IntPoint::new(8192, 0),
        IntPoint::new(8192, 4096),
        IntPoint::new(4096, 4096),
        IntPoint::new(4096, 8192),
        IntPoint::new(0, 8192),
    ];
    let float: Vec<_> = path.iter().map(|p| [p.x as f64, p.y as f64]).collect();
    for offset in [-1024, 0, 1024] {
        let actual = path.outline(&IntOutlineStyle::new(offset)).unwrap();
        let mut options = OverlayOptions::default();
        options.clean_result = false;
        let expected = float
            .outline_custom_fixed_scale(&OutlineStyle::new(offset as f64), options, 1.0)
            .unwrap();
        let expected: IntShapes<i32> = expected
            .iter()
            .map(|shape| {
                shape
                    .iter()
                    .map(|path| {
                        path.iter()
                            .map(|p| IntPoint::new(p[0].round() as i32, p[1].round() as i32))
                            .collect()
                    })
                    .collect()
            })
            .collect();
        assert_eq!(canonical(actual), canonical(expected));
    }
}
