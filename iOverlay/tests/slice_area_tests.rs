use i_float::int::point::IntPoint;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::IntOverlayOptions;
use i_overlay::float::overlay::OverlayOptions;
use i_overlay::float::string_overlay::FloatStringOverlay;
use i_overlay::string::overlay::StringOverlay;
use i_overlay::string::rule::StringRule;

fn square() -> [IntPoint; 4] {
    [
        IntPoint::new(0, 0),
        IntPoint::new(10, 0),
        IntPoint::new(10, 10),
        IntPoint::new(0, 10),
    ]
}

fn area_two(path: &[IntPoint]) -> i64 {
    path.iter()
        .zip(path.iter().cycle().skip(1))
        .map(|(a, b)| i64::from(a.x) * i64::from(b.y) - i64::from(a.y) * i64::from(b.x))
        .sum()
}

#[test]
fn slice_minimum_area_removes_small_piece() {
    let mut overlay = StringOverlay::from_shape(&square());
    overlay.add_string_line([IntPoint::new(1, -1), IntPoint::new(1, 11)]);
    let graph = overlay.build_graph_view(FillRule::NonZero).unwrap();
    let unfiltered = graph.extract_shapes(StringRule::Slice);
    let mut areas: Vec<_> = unfiltered.iter().map(|s| area_two(&s[0])).collect();
    areas.sort();
    assert_eq!(areas, [20, 180]);

    for (threshold, expected) in [
        (0, vec![20, 180]),
        (10, vec![20, 180]),
        (11, vec![180]),
        (50, vec![180]),
        (90, vec![180]),
        (91, vec![]),
    ] {
        let options = IntOverlayOptions {
            min_output_area: threshold,
            ..Default::default()
        };
        let filtered = graph.extract_shapes_custom(StringRule::Slice, options);
        let mut areas: Vec<_> = filtered.iter().map(|s| area_two(&s[0])).collect();
        areas.sort();
        assert_eq!(areas, expected, "minimum area {threshold} must include equality");
    }
}

#[test]
fn slice_minimum_area_above_subject_area_returns_empty() {
    let mut overlay = StringOverlay::from_shape(&square());
    let graph = overlay.build_graph_view(FillRule::NonZero).unwrap();
    let options = IntOverlayOptions {
        min_output_area: 1000,
        ..Default::default()
    };
    let result = graph.extract_shapes_custom(StringRule::Slice, options);
    assert!(
        result.is_empty(),
        "a square of area 100 must be filtered out at threshold 1000"
    );
}

#[test]
fn slice_small_area_threshold_preserves_large_loops_and_holes() {
    let mut overlay = StringOverlay::from_shape(&square());
    overlay.add_string_path(&[
        IntPoint::new(0, 0),
        IntPoint::new(3, 3),
        IntPoint::new(7, 3),
        IntPoint::new(7, 7),
        IntPoint::new(3, 7),
        IntPoint::new(3, 3),
    ]);
    let graph = overlay.build_graph_view(FillRule::NonZero).unwrap();
    let expected = graph.extract_shapes(StringRule::Slice);
    assert_eq!(expected.len(), 2);
    let total_area: i64 = expected.iter().flatten().map(|p| area_two(p)).sum();
    assert_eq!(total_area, 200);
    for threshold in [1, 16] {
        let options = IntOverlayOptions {
            min_output_area: threshold,
            ..Default::default()
        };
        let actual = graph.extract_shapes_custom(StringRule::Slice, options);
        assert_eq!(
            actual, expected,
            "areas 16 and 84 must survive minimum area {threshold}"
        );
    }
}

#[test]
fn float_slice_small_area_threshold_preserves_large_loops_and_holes() {
    let subject = [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]];
    let cut = [
        [0.0, 0.0],
        [3.0, 3.0],
        [7.0, 3.0],
        [7.0, 7.0],
        [3.0, 7.0],
        [3.0, 3.0],
    ];
    let mut overlay = FloatStringOverlay::with_shape_and_string_fixed_scale(&subject, &cut, 100.0).unwrap();
    let graph = overlay.build_graph_view(FillRule::NonZero).unwrap();
    let expected = graph.extract_shapes(StringRule::Slice);
    assert_eq!(expected.len(), 2);
    assert_eq!(expected.iter().map(|s| s.len() - 1).sum::<usize>(), 1);
    let mut options = OverlayOptions::default();
    options.min_output_area = 1.0;
    let actual = graph.extract_shapes_custom(StringRule::Slice, options);
    assert_eq!(
        actual, expected,
        "a low area threshold must preserve the annulus and its inner piece"
    );
}
