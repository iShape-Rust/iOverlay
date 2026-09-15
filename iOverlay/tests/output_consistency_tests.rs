use i_float::int::point::IntPoint;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::{ContourDirection, Overlay};
use i_overlay::core::overlay_rule::OverlayRule;
use i_shape::flat::buffer::FlatContoursBuffer;

fn canonical(mut paths: Vec<Vec<IntPoint>>) -> Vec<Vec<IntPoint>> {
    for path in &mut paths {
        let start = (0..path.len())
            .min_by(|&a, &b| {
                (0..path.len())
                    .map(|j| path[(a + j) % path.len()])
                    .cmp((0..path.len()).map(|j| path[(b + j) % path.len()]))
            })
            .unwrap();
        path.rotate_left(start);
    }
    paths.sort();
    paths
}

#[test]
fn flat_and_vector_outputs_match_shapes_with_area_filtering() {
    let mut seed = 0x61ab_117c_0018_23ad_u64;
    let mut next = || {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        ((seed >> 32) % 41) as i32 - 20
    };
    for case in 0..1000 {
        let a: Vec<_> = (0..3 + case % 8).map(|_| IntPoint::new(next(), next())).collect();
        let b: Vec<_> = (0..3 + case % 9).map(|_| IntPoint::new(next(), next())).collect();
        for threshold in [0, 1, 10, 50] {
            let mut regular = Overlay::with_contour(&a, &b);
            regular.options.min_output_area = threshold;
            regular.options.preserve_output_collinear = case % 2 == 0;
            regular.options.output_direction = if case % 3 == 0 {
                ContourDirection::Clockwise
            } else {
                ContourDirection::CounterClockwise
            };
            for rule in [
                OverlayRule::Union,
                OverlayRule::Intersect,
                OverlayRule::Difference,
                OverlayRule::Xor,
            ] {
                let fill = [
                    FillRule::EvenOdd,
                    FillRule::NonZero,
                    FillRule::Positive,
                    FillRule::Negative,
                ][case % 4];
                let shapes = regular.overlay(rule, fill);
                let expected = canonical(shapes.into_iter().flatten().collect());
                let mut flat = FlatContoursBuffer::default();
                regular.overlay_into(rule, fill, &mut flat);
                let actual = canonical(
                    flat.ranges
                        .iter()
                        .map(|r| flat.points[r.clone()].to_vec())
                        .collect(),
                );
                assert_eq!(
                    actual, expected,
                    "flat: case={case}, threshold={threshold}, rule={rule:?}, a={a:?}, b={b:?}"
                );
                let shapes = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    regular.build_shape_vectors(fill, rule)
                }))
                .unwrap_or_else(|_| {
                    panic!("vector extraction panicked: case={case}, threshold={threshold}, rule={rule:?}, fill={fill:?}, a={a:?}, b={b:?}")
                });
                let actual = canonical(
                    shapes
                        .into_iter()
                        .flatten()
                        .map(|p| p.into_iter().map(|e| e.a).collect())
                        .collect(),
                );
                assert_eq!(
                    actual, expected,
                    "vectors: case={case}, threshold={threshold}, rule={rule:?}, a={a:?}, b={b:?}"
                );
            }
        }
    }
}
