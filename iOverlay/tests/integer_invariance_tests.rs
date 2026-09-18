use i_float::int::point::IntPoint;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::{Overlay, ShapeType};
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::solver::Solver;

type Shapes = Vec<Vec<Vec<IntPoint>>>;

fn canonical(mut shapes: Shapes) -> Shapes {
    for shape in &mut shapes {
        for path in shape.iter_mut() {
            let start = (0..path.len())
                .min_by(|&a, &b| {
                    (0..path.len())
                        .map(|i| path[(a + i) % path.len()])
                        .cmp((0..path.len()).map(|i| path[(b + i) % path.len()]))
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
fn boolean_results_ignore_contour_start_direction_and_operand_order() {
    let mut state = 0x7798_117b_13ac_7331_u64;
    let mut next = || {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        ((state >> 32) % 17) as i32 - 8
    };
    for case in 0..2000 {
        let a: Vec<_> = (0..3 + case % 9).map(|_| IntPoint::new(next(), next())).collect();
        let b: Vec<_> = (0..3 + case % 7).map(|_| IntPoint::new(next(), next())).collect();
        let mut reversed_a = a.clone();
        let mut reversed_b = b.clone();
        reversed_a.reverse();
        reversed_b.reverse();
        reversed_a.rotate_left(case % a.len());
        reversed_b.rotate_left(case % b.len());
        for fill in [FillRule::EvenOdd, FillRule::NonZero] {
            for rule in [OverlayRule::Intersect, OverlayRule::Union, OverlayRule::Xor] {
                let expected = canonical(Overlay::from_subj_and_clip(&a, &b).overlay(rule, fill));
                let actual =
                    canonical(Overlay::from_subj_and_clip(&reversed_b, &reversed_a).overlay(rule, fill));
                assert_eq!(
                    actual, expected,
                    "case={case}, fill={fill:?}, rule={rule:?}, a={a:?}, b={b:?}"
                );
            }
        }
    }
}

#[test]
fn adding_rectangles_after_extraction_matches_a_fresh_overlay() {
    let mut state = 0x43ba_2846_7311_9133_u64;
    let mut next = || {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        ((state >> 32) % 33) as i32 - 16
    };
    for case in 0..1000 {
        // Keep intersections exact: sequential snapping can legitimately differ
        // from splitting all arbitrary edges in a single pass.
        let mut rectangle = || {
            let x = next();
            let y = next();
            let w = next().abs() + 1;
            let h = next().abs() + 1;
            vec![
                IntPoint::new(x, y),
                IntPoint::new(x + w, y),
                IntPoint::new(x + w, y + h),
                IntPoint::new(x, y + h),
            ]
        };
        let a = rectangle();
        let b = rectangle();
        let c = rectangle();
        for solver in [Solver::LIST, Solver::TREE, Solver::FRAG] {
            let mut reused = Overlay::from_subj_and_clip(&a, &b);
            reused.solver = solver;
            reused.overlay(OverlayRule::Intersect, FillRule::EvenOdd);
            reused.add_contour(&c, ShapeType::Subject);
            let mut fresh = Overlay::from_subj_and_clip(&a, &b);
            fresh.solver = solver;
            fresh.add_contour(&c, ShapeType::Subject);
            assert_eq!(
                canonical(reused.overlay(OverlayRule::Union, FillRule::EvenOdd)),
                canonical(fresh.overlay(OverlayRule::Union, FillRule::EvenOdd)),
                "case={case}, solver={:?}, a={a:?}, b={b:?}, c={c:?}",
                solver.strategy,
            );
        }
    }
}
