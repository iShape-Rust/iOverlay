use i_overlay::float::overlay::OverlayOptions;
use i_overlay::mesh::float::outline::offset::OutlineOffset;
use i_overlay::mesh::float::stroke::offset::StrokeOffset;
use i_overlay::mesh::float::style::{LineJoin, OutlineStyle, StrokeStyle};
use i_shape::flat::float::FloatFlatContoursBuffer;

fn rectangle(x0: f64, y0: f64, x1: f64, y1: f64) -> Vec<[f64; 2]> {
    vec![[x0, y0], [x1, y0], [x1, y1], [x0, y1]]
}

#[test]
fn reused_mesh_output_matches_fresh_buffer_after_empty_results() {
    let paths = [
        rectangle(0.0, 0.0, 10.0, 10.0),
        vec![],
        vec![[1.0, 1.0]],
        vec![[1.0, 1.0]; 4],
        vec![[0.0, 0.0], [1.0, 0.0], [2.0, 0.0]],
        rectangle(20.0, 20.0, 21.0, 21.0),
        rectangle(30.0, 30.0, 40.0, 40.0),
    ];
    let mut reused = FloatFlatContoursBuffer::default();
    for fixed in [false, true] {
        for offset in [-1.0, 0.0, 1.0] {
            let outline = OutlineStyle::new(offset);
            for path in &paths {
                let mut fresh = FloatFlatContoursBuffer::default();
                for output in [&mut reused, &mut fresh] {
                    if fixed {
                        path.outline_fixed_scale_into(&outline, 100.0, output).unwrap();
                    } else {
                        path.outline_into(&outline, output);
                    }
                }
                assert_eq!(
                    reused.points, fresh.points,
                    "outline offset={offset}, path={path:?}"
                );
                assert_eq!(reused.ranges, fresh.ranges);
                for closed in [false, true] {
                    let mut fresh = FloatFlatContoursBuffer::default();
                    for output in [&mut reused, &mut fresh] {
                        if fixed {
                            path.stroke_fixed_scale_into(StrokeStyle::new(2.0), closed, 100.0, output)
                                .unwrap();
                        } else {
                            path.stroke_into(StrokeStyle::new(2.0), closed, output);
                        }
                    }
                    assert_eq!(
                        reused.points, fresh.points,
                        "stroke closed={closed}, path={path:?}"
                    );
                    assert_eq!(reused.ranges, fresh.ranges);
                }
            }
        }
    }
}

#[test]
fn collapsed_offsets_do_not_change_surviving_contours_when_reordered() {
    let mut hole = rectangle(10.0, 10.0, 20.0, 20.0);
    hole.reverse();
    let mut small_hole = rectangle(3.0, 3.0, 5.0, 5.0);
    small_hole.reverse();
    let paths = [rectangle(0.0, 0.0, 30.0, 30.0), small_hole, hole];
    for offset in [1.0, 2.0, 5.0, 6.0] {
        let style = OutlineStyle::new(offset);
        let expected = paths.outline_fixed_scale(&style, 100.0).unwrap();
        for order in [[0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]] {
            let source: Vec<_> = order.iter().map(|&i| paths[i].clone()).collect();
            let actual = source.outline_fixed_scale(&style, 100.0).unwrap();
            assert_eq!(actual, expected, "offset={offset}, order={order:?}");
        }
    }
}

#[test]
fn outline_area_filter_keeps_small_offsets_that_merge_into_a_large_shape() {
    check_area_filter_after_union(true);
}

#[test]
fn automatic_scale_outline_area_filter_keeps_merged_shape() {
    check_area_filter_after_union(false);
}

fn check_area_filter_after_union(fixed: bool) {
    // Disjoint input squares become overlapping 4x4 squares. Their union is
    // a 7x4 rectangle: the final area 28 exceeds the threshold 20.
    let source = [rectangle(0.0, 0.0, 2.0, 2.0), rectangle(3.0, 0.0, 5.0, 2.0)];
    let style = OutlineStyle::new(1.0).line_join(LineJoin::Miter(0.1));
    let extract = |options| {
        if fixed {
            source.outline_custom_fixed_scale(&style, options, 100.0).unwrap()
        } else {
            source.outline_custom(&style, options)
        }
    };
    let expected = extract(OverlayOptions::default());
    assert_eq!(expected.len(), 1);
    let area: f64 = expected
        .iter()
        .flatten()
        .map(|path| {
            path.iter()
                .zip(path.iter().cycle().skip(1))
                .map(|(a, b)| 0.5 * (a[0] * b[1] - a[1] * b[0]))
                .sum::<f64>()
        })
        .sum();
    assert!((area - 28.0).abs() < 1e-6, "unfiltered area={area}");
    let mut options = OverlayOptions::default();
    options.min_output_area = 29.0;
    assert!(
        extract(options).is_empty(),
        "the final area filter must still remove the union below threshold"
    );
    options.min_output_area = 20.0;
    let actual = extract(options);
    assert_eq!(
        actual, expected,
        "minimum output area must be applied to the merged shape of area 28"
    );
}
