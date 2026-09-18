use i_float::int::point::IntPoint;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::{ContourDirection, Overlay};
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::solver::Solver;

fn touching_hole() -> Vec<IntPoint> {
    // Outer area 6, hole area 1. The hole touches the leftmost outer vertex.
    [[0, 1], [-1, 0], [0, 0], [0, -2], [-1, 0], [0, -3], [2, -1]]
        .map(|p| IntPoint::new(p[0], p[1]))
        .to_vec()
}

fn double_area(path: &[IntPoint]) -> i64 {
    path.iter()
        .zip(path.iter().cycle().skip(1))
        .map(|(a, b)| i64::from(a.x) * i64::from(b.y) - i64::from(a.y) * i64::from(b.x))
        .sum()
}

fn check_hole_direction(clockwise: bool) {
    for shift in 0..7 {
        let mut path = touching_hole();
        path.rotate_left(shift);
        let mut overlay = Overlay::from_subj(&path);
        overlay.options.ogc = true;
        overlay.options.output_direction = if clockwise {
            ContourDirection::Clockwise
        } else {
            ContourDirection::CounterClockwise
        };
        let shapes = overlay.overlay(OverlayRule::Subject, FillRule::NonZero);
        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].len(), 2);
        assert_eq!(double_area(&shapes[0][0]) < 0, clockwise);
        assert_eq!(
            double_area(&shapes[0][1]) > 0,
            clockwise,
            "shift={shift}, shapes={shapes:?}"
        );
        let area: i64 = shapes.iter().flatten().map(|p| double_area(p)).sum();
        assert_eq!(area, if clockwise { -10 } else { 10 });
    }
}

#[test]
fn ogc_leftmost_touching_hole_has_opposite_winding() {
    check_hole_direction(false);
}

#[test]
fn clockwise_ogc_leftmost_touching_hole_has_opposite_winding() {
    check_hole_direction(true);
}

#[test]
fn ogc_leftmost_touching_hole_survives_nonzero_roundtrip() {
    let mut overlay = Overlay::from_subj(&touching_hole());
    overlay.options.ogc = true;
    let shapes = overlay.overlay(OverlayRule::Subject, FillRule::NonZero);
    let contours: Vec<_> = shapes.into_iter().flatten().collect();
    let result = Overlay::from_subj(&contours).overlay(OverlayRule::Subject, FillRule::NonZero);
    let area: i64 = result.iter().flatten().map(|p| double_area(p)).sum();
    assert_eq!(
        area, 10,
        "area must remain 6 - 1 after reusing OGC output: {result:?}"
    );
}

#[test]
fn ordinary_extraction_preserves_leftmost_touching_hole_area() {
    let shapes = Overlay::from_subj(&touching_hole()).overlay(OverlayRule::Subject, FillRule::NonZero);
    let area: i64 = shapes.iter().flatten().map(|p| double_area(p)).sum();
    assert_eq!(area, 10);
}

fn holes_at_leftmost_vertex(count: usize) -> Vec<Vec<IntPoint>> {
    let mut contours = vec![vec![
        IntPoint::new(0, 0),
        IntPoint::new(48, -48),
        IntPoint::new(48, 48),
    ]];
    for i in 0..count {
        let bottom = -20 + 5 * i as i32;
        let top = bottom + 1 + (i % 3) as i32;
        // Disjoint angular sectors, separated by filled wedges. Every hole
        // shares exactly the outer contour's unique leftmost vertex.
        contours.push(vec![
            IntPoint::new(0, 0),
            IntPoint::new(24, top),
            IntPoint::new(24, bottom),
        ]);
    }
    contours
}

fn canonical_contours(mut contours: Vec<Vec<IntPoint>>) -> Vec<Vec<IntPoint>> {
    for contour in &mut contours {
        let start = contour.iter().enumerate().min_by_key(|(_, p)| **p).unwrap().0;
        contour.rotate_left(start);
    }
    contours.sort();
    contours
}

fn check_shared_leftmost_holes(input: &[Vec<IntPoint>], expected: &[Vec<IntPoint>], case: &str) {
    let expected_area: i64 = expected.iter().map(|p| double_area(p)).sum();
    for solver in [Solver::LIST, Solver::TREE, Solver::FRAG] {
        for fill in [FillRule::EvenOdd, FillRule::NonZero] {
            for clockwise in [false, true] {
                let mut overlay = Overlay::from_subj(input);
                overlay.options.ogc = true;
                overlay.options.output_direction = if clockwise {
                    ContourDirection::Clockwise
                } else {
                    ContourDirection::CounterClockwise
                };
                overlay.solver = solver;
                let shapes = overlay.overlay(OverlayRule::Subject, fill);
                let context = format!(
                    "{case}, solver={:?}, fill={fill:?}, clockwise={clockwise}",
                    solver.strategy
                );
                assert_eq!(shapes.len(), 1, "{context}: {shapes:?}");
                assert_eq!(shapes[0].len(), expected.len(), "{context}: {shapes:?}");
                for (i, contour) in shapes[0].iter().enumerate() {
                    assert_eq!(
                        double_area(contour) < 0,
                        clockwise != (i > 0),
                        "{context}: {contour:?}"
                    );
                }
                let area: i64 = shapes[0].iter().map(|p| double_area(p)).sum();
                assert_eq!(
                    area,
                    if clockwise { -expected_area } else { expected_area },
                    "{context}"
                );
                let mut expected_direction = expected.to_vec();
                if clockwise {
                    for contour in &mut expected_direction {
                        contour.reverse();
                    }
                }
                // Exact boundaries catch missing, duplicated, or incorrectly split holes.
                assert_eq!(
                    canonical_contours(shapes[0].clone()),
                    canonical_contours(expected_direction),
                    "{context}"
                );

                let mut reused = Overlay::from_subj(&shapes[0]);
                reused.options.ogc = true;
                reused.solver = solver;
                let roundtrip = reused.overlay(OverlayRule::Subject, FillRule::NonZero);
                assert_eq!(roundtrip.len(), 1, "roundtrip: {context}");
                assert_eq!(
                    canonical_contours(roundtrip[0].clone()),
                    canonical_contours(expected.to_vec()),
                    "roundtrip: {context}"
                );
            }
        }
    }
}

#[test]
fn ogc_multiple_holes_share_the_outer_leftmost_vertex() {
    for count in [2, 3, 8] {
        let expected = holes_at_leftmost_vertex(count);
        for reversed in [false, true] {
            for shift in 0..expected.len() {
                let mut input = expected.clone();
                input.rotate_left(shift);
                if reversed {
                    input.reverse();
                }
                // Change the starting vertex of each separate contour as well.
                for (i, contour) in input.iter_mut().enumerate() {
                    contour.rotate_left((shift + i) % 3);
                }
                check_shared_leftmost_holes(
                    &input,
                    &expected,
                    &format!("holes={count}, shift={shift}, reversed_order={reversed}"),
                );
            }
        }
    }
}

#[test]
fn ogc_one_contour_visits_multiple_holes_at_its_leftmost_vertex() {
    for count in [2, 3, 8] {
        let expected = holes_at_leftmost_vertex(count);
        for reversed_hole_order in [false, true] {
            let mut holes = expected[1..].to_vec();
            if reversed_hole_order {
                holes.reverse();
            }
            let mut path = vec![IntPoint::new(0, 0)];
            for hole in &holes {
                path.extend_from_slice(&hole[1..]);
                path.push(IntPoint::new(0, 0));
            }
            path.extend_from_slice(&expected[0][1..]);
            for reversed_winding in [false, true] {
                let mut input = path.clone();
                if reversed_winding {
                    input.reverse();
                }
                for shift in 0..input.len() {
                    check_shared_leftmost_holes(
                        &[input.clone()],
                        &expected,
                        &format!(
                            "holes={count}, shift={shift}, reversed_hole_order={reversed_hole_order}, reversed_winding={reversed_winding}"
                        ),
                    );
                    input.rotate_left(1);
                }
            }
        }
    }
}
