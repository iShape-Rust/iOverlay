use i_float::int::point::IntPoint;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::{ContourDirection, Overlay};
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::solver::Solver;

type Shapes = Vec<Vec<Vec<IntPoint>>>;

fn area(path: &[IntPoint]) -> u64 {
    path.iter()
        .zip(path.iter().cycle().skip(1))
        .map(|(a, b)| i64::from(a.x) * i64::from(b.y) - i64::from(a.y) * i64::from(b.x))
        .sum::<i64>()
        .unsigned_abs()
        / 2
}

fn canonical(mut shapes: Shapes) -> Shapes {
    for shape in &mut shapes {
        for path in shape.iter_mut() {
            let start = (0..path.len())
                .min_by(|&a, &b| {
                    (0..path.len())
                        .map(|j| path[(a + j) % path.len()])
                        .cmp((0..path.len()).map(|j| path[(b + j) % path.len()]))
                })
                .unwrap();
            path.rotate_left(start);
        }
        shape[1..].sort();
    }
    shapes.sort();
    shapes
}

#[test]
fn ogc_area_filter_matches_filtering_resolved_contours() {
    let mut seed = 0x728a_13dd_8741_990e_u64;
    let mut next = || {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        ((seed >> 32) % 41) as i32 - 20
    };
    for case in 0..500 {
        let paths: Vec<Vec<_>> = (0..1 + case % 4)
            .map(|_| (0..3 + case % 8).map(|_| IntPoint::new(next(), next())).collect())
            .collect();
        for clockwise in [false, true] {
            let mut overlay = Overlay::from_subj(&paths);
            overlay.options.ogc = true;
            overlay.options.output_direction = if clockwise {
                ContourDirection::Clockwise
            } else {
                ContourDirection::CounterClockwise
            };
            overlay.solver = [Solver::LIST, Solver::TREE, Solver::FRAG][case % 3];
            let all = overlay.overlay(OverlayRule::Subject, FillRule::EvenOdd);
            // The last zero threshold also checks reuse after an empty extraction.
            for threshold in [0, 1, 2, 5, 10, 50, u64::MAX, 0] {
                let expected: Shapes = all
                    .iter()
                    .filter(|s| area(&s[0]) >= threshold)
                    .map(|s| {
                        let mut shape = vec![s[0].clone()];
                        shape.extend(s.iter().skip(1).filter(|p| area(p) >= threshold).cloned());
                        shape
                    })
                    .collect();
                overlay.options.min_output_area = threshold;
                let actual = overlay.overlay(OverlayRule::Subject, FillRule::EvenOdd);
                assert_eq!(
                    canonical(actual),
                    canonical(expected),
                    "case={case}, clockwise={clockwise}, threshold={threshold}, input={paths:?}"
                );
            }
        }
    }
}
