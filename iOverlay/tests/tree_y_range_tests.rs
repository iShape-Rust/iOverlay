use i_float::int::point::IntPoint;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::integer::OverlayInt;
use i_overlay::core::overlay::Overlay;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::solver::Solver;

fn check_tree_y_ranges<I: OverlayInt + From<i32> + std::fmt::Debug>() {
    let point = |x: i32, y: i32| IntPoint::<I>::new(x.into(), y.into());
    let rect = |x0, y0, x1, y1| vec![point(x0, y0), point(x0, y1), point(x1, y1), point(x1, y0)];
    // The distant contour puts all nearby, disjoint Y bands in one
    // coarse tree cell, while their X ranges overlap.
    let mut subj = vec![rect(0, 0, 40, 10), rect(0, 30, 40, 40)];
    let mut clip = vec![rect(10, 10, 30, 20), rect(40, 2, 50, 8)];
    for y in (60..300).step_by(20) {
        subj.push(rect(0, y, 40, y + 4));
        clip.push(rect(10, y + 8, 30, y + 12));
    }
    subj.push(rect(0, 1_000_000, 40, 1_000_010));
    // Partial horizontal and vertical shared edges above require
    // splitting at boundary Y values. These oblique crossings also
    // exercise rounding and subsequent split passes.
    clip.push(vec![point(-3, 29), point(43, 37), point(17, 43)]);

    for fill in [FillRule::EvenOdd, FillRule::NonZero] {
        for rule in [OverlayRule::Union, OverlayRule::Intersect, OverlayRule::Xor] {
            let list = Overlay::from_subj_and_clip_custom(&subj, &clip, Default::default(), Solver::LIST)
                .overlay(rule, fill);
            let tree = Overlay::from_subj_and_clip_custom(&subj, &clip, Default::default(), Solver::TREE)
                .overlay(rule, fill);
            assert!(!list.is_empty());
            assert_eq!(tree, list, "{rule:?}, {fill:?}");
        }
    }
}

#[test]
fn tree_y_ranges_i32() {
    check_tree_y_ranges::<i32>();
}

#[test]
fn tree_y_ranges_i64() {
    check_tree_y_ranges::<i64>();
}
