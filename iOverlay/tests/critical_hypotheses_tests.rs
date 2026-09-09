use i_float::int::point::IntPoint;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::integer::OverlayInt;
use i_overlay::core::overlay::Overlay;
use i_overlay::core::overlay::ShapeType;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::relate::PredicateOverlay;
use i_overlay::core::solver::Solver;
use i_overlay::mesh::stroke::offset::StrokeOffset;
use i_overlay::mesh::style::StrokeStyle;

fn rectangle(x0: i32, y0: i32, x1: i32, y1: i32) -> Vec<IntPoint> {
    vec![
        IntPoint::new(x0, y0),
        IntPoint::new(x1, y0),
        IntPoint::new(x1, y1),
        IntPoint::new(x0, y1),
    ]
}

#[test]
fn hypothesis_closed_stroke_of_empty_path_returns_empty() {
    let path: Vec<[f64; 2]> = Vec::new();
    assert!(path.stroke(StrokeStyle::new(1.0), true).is_empty());
}

#[test]
fn hypothesis_closed_stroke_of_empty_contours_returns_empty() {
    let paths: Vec<Vec<[f64; 2]>> = vec![Vec::new(), Vec::new()];
    assert!(paths.stroke(StrokeStyle::new(1.0), true).is_empty());
}

#[test]
fn hypothesis_fixed_scale_closed_stroke_of_empty_path_returns_empty() {
    let path: Vec<[f64; 2]> = Vec::new();
    assert!(
        path.stroke_fixed_scale(StrokeStyle::new(1.0), true, 100.0)
            .unwrap()
            .is_empty()
    );
}

#[test]
fn hypothesis_closed_stroke_into_of_empty_path_returns_empty() {
    let path: Vec<[f64; 2]> = Vec::new();
    let mut output = Default::default();
    path.stroke_into(StrokeStyle::new(1.0), true, &mut output);
    assert!(output.points.is_empty());
    assert!(output.ranges.is_empty());
}

#[test]
fn empty_stroke_controls() {
    let path: Vec<[f64; 2]> = Vec::new();
    assert!(path.stroke(StrokeStyle::new(1.0), false).is_empty());
    let paths: Vec<Vec<[f64; 2]>> = Vec::new();
    assert!(paths.stroke(StrokeStyle::new(1.0), true).is_empty());
    let point = [[0.0, 0.0]];
    assert!(point.stroke(StrokeStyle::new(1.0), true).is_empty());
}

fn normalized_overlay<I: OverlayInt + TryFrom<i32> + Into<i64>>(
    subj: &[[i32; 2]],
    clip: &[[i32; 2]],
    rule: OverlayRule,
    solver: Solver,
) -> Vec<Vec<Vec<[i64; 2]>>> {
    let convert = |path: &[[i32; 2]]| -> Vec<IntPoint<I>> {
        path.iter()
            .map(|p| IntPoint::new(I::try_from(p[0]).ok().unwrap(), I::try_from(p[1]).ok().unwrap()))
            .collect()
    };
    let output = Overlay::with_contour_custom(&convert(subj), &convert(clip), Default::default(), solver)
        .overlay(rule, FillRule::EvenOdd);
    let mut output: Vec<Vec<Vec<[i64; 2]>>> = output
        .into_iter()
        .map(|shape| {
            let mut shape: Vec<Vec<[i64; 2]>> = shape
                .into_iter()
                .map(|path| {
                    let mut path: Vec<_> = path.into_iter().map(|p| [p.x.into(), p.y.into()]).collect();
                    let start = path.iter().enumerate().min_by_key(|(_, p)| **p).unwrap().0;
                    path.rotate_left(start);
                    path
                })
                .collect();
            shape[1..].sort();
            shape
        })
        .collect();
    output.sort();
    output
}

#[test]
fn hypothesis_integer_engines_agree_near_i16_limits() {
    let mut state = 0x47a3_9012_771b_81d1_u64;
    let mut next = || {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        ((state >> 32) % 32768) as i32 - 16384
    };
    for case in 0..2000 {
        let subj: Vec<_> = (0..3 + case % 8).map(|_| [next(), next()]).collect();
        let clip: Vec<_> = (0..3 + case % 7).map(|_| [next(), next()]).collect();
        for rule in [
            OverlayRule::Intersect,
            OverlayRule::Union,
            OverlayRule::Difference,
            OverlayRule::Xor,
        ] {
            let expected = normalized_overlay::<i64>(&subj, &clip, rule, Solver::LIST);
            for solver in [Solver::LIST, Solver::TREE, Solver::FRAG] {
                assert_eq!(
                    normalized_overlay::<i16>(&subj, &clip, rule, solver),
                    expected,
                    "i16 case={case}, rule={rule:?}, solver={:?}, subj={subj:?}, clip={clip:?}",
                    solver.strategy
                );
                assert_eq!(
                    normalized_overlay::<i32>(&subj, &clip, rule, solver),
                    expected,
                    "i32 case={case}, rule={rule:?}, solver={:?}, subj={subj:?}, clip={clip:?}",
                    solver.strategy
                );
            }
        }
    }
}

#[test]
fn hypothesis_predicates_remain_correct_after_early_exit_and_clear() {
    for solver in [Solver::LIST, Solver::TREE, Solver::FRAG] {
        let mut overlay = PredicateOverlay::new(8);
        overlay.solver = solver;
        for x0 in -2..=2 {
            for y0 in -2..=2 {
                for x1 in x0 + 1..=3 {
                    for y1 in y0 + 1..=3 {
                        overlay.clear();
                        overlay.add_contour(&rectangle(0, 0, 2, 2), ShapeType::Subject);
                        overlay.add_contour(&rectangle(x0, y0, x1, y1), ShapeType::Clip);
                        let intersects = x0 <= 2 && y0 <= 2 && x1 >= 0 && y1 >= 0;
                        let interiors = x0 < 2 && y0 < 2 && x1 > 0 && y1 > 0;
                        let within = x0 <= 0 && y0 <= 0 && x1 >= 2 && y1 >= 2;
                        for repeat in 0..2 {
                            let actual = (
                                overlay.intersects(),
                                overlay.interiors_intersect(),
                                overlay.touches(),
                                overlay.within(),
                            );
                            assert_eq!(
                                actual,
                                (intersects, interiors, intersects && !interiors, within),
                                "clip=({x0},{y0})..({x1},{y1}), solver={:?}, repeat={repeat}",
                                solver.strategy,
                            );
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn hypothesis_predicates_agree_with_boolean_results_on_dense_inputs() {
    let mut state = 0xb641_7890_a113_3145_u64;
    let mut next = || {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        ((state >> 32) % 17) as i32 - 8
    };
    for case in 0..5000 {
        let subj: Vec<_> = (0..3 + case % 8).map(|_| IntPoint::new(next(), next())).collect();
        let clip: Vec<_> = (0..3 + case % 7).map(|_| IntPoint::new(next(), next())).collect();
        for fill_rule in [
            FillRule::EvenOdd,
            FillRule::NonZero,
            FillRule::Positive,
            FillRule::Negative,
        ] {
            let mut overlay = Overlay::with_contour(&subj, &clip);
            let intersection = overlay.overlay(OverlayRule::Intersect, fill_rule);
            let difference = overlay.overlay(OverlayRule::Difference, fill_rule);
            let subject = overlay.overlay(OverlayRule::Subject, fill_rule);
            let mut predicates = PredicateOverlay::new(subj.len() + clip.len());
            predicates.fill_rule = fill_rule;
            predicates.add_contour(&subj, ShapeType::Subject);
            predicates.add_contour(&clip, ShapeType::Clip);
            assert_eq!(
                predicates.interiors_intersect(),
                !intersection.is_empty(),
                "interiors: case={case}, fill={fill_rule:?}, subj={subj:?}, clip={clip:?}"
            );
            assert_eq!(
                predicates.within(),
                !subject.is_empty() && difference.is_empty(),
                "within: case={case}, fill={fill_rule:?}, subj={subj:?}, clip={clip:?}"
            );
            if !intersection.is_empty() {
                assert!(
                    predicates.intersects(),
                    "intersects: case={case}, fill={fill_rule:?}"
                );
                assert!(!predicates.touches(), "touches: case={case}, fill={fill_rule:?}");
                assert!(
                    !predicates.point_intersects(),
                    "point: case={case}, fill={fill_rule:?}"
                );
            }
        }
    }
}
