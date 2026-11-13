//! Rounded crossings must not move segments further than rounding does.
//!
//! A crossing that rounds next to a segment endpoint is snapped to that endpoint, and the other
//! segment is split there. With integer inputs, that used to bend the other segment by up to a
//! unit or more, e.g. outside the polygon it bounds.

use i_float::int::point::IntPoint;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::{IntOverlayOptions, Overlay};
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::solver::Solver;
use i_shape::int::shape::IntShapes;

const SOLVERS: [Solver; 4] = [Solver::LIST, Solver::TREE, Solver::FRAG, Solver::AUTO];

fn contour(points: &[[i32; 2]]) -> Vec<IntPoint> {
    points.iter().map(|&[x, y]| IntPoint::new(x, y)).collect()
}

/// Shapes as sorted contours, each starting at its smallest point, for order-independent
/// comparison.
fn normalized(shapes: &IntShapes<i32>) -> Vec<Vec<Vec<IntPoint>>> {
    let mut shapes: Vec<Vec<Vec<IntPoint>>> = shapes
        .iter()
        .map(|shape| {
            shape
                .iter()
                .map(|path| {
                    let mut path = path.clone();
                    let start = path.iter().enumerate().min_by_key(|(_, p)| **p).unwrap().0;
                    path.rotate_left(start);
                    path
                })
                .collect()
        })
        .collect();
    shapes.sort();
    shapes
}

fn overlay(
    subj: &[Vec<IntPoint>],
    clip: &[Vec<IntPoint>],
    rule: OverlayRule,
    solver: Solver,
) -> IntShapes<i32> {
    Overlay::from_subj_and_clip_custom(&subj.to_vec(), &clip.to_vec(), IntOverlayOptions::ogc(), solver)
        .overlay(rule, FillRule::EvenOdd)
}

#[test]
fn box_minus_collapsed_sliver_stays_inside_the_box() {
    // The sliver's top edge crosses the box at (1, 2 1/11) and (2, 2 2/11), which round onto its
    // bottom edge, so the sliver collapses over the box. Snapping these crossings to the sliver's
    // vertex (0, 2) and the box's corner (2, 1) produced a triangle outside the box.
    let boxes = [contour(&[[1, 1], [2, 1], [2, 4], [1, 4]])];
    let sliver = [contour(&[[0, 2], [11, 2], [11, 3]])];
    for solver in SOLVERS {
        let difference = overlay(&boxes, &sliver, OverlayRule::Difference, solver);
        assert_eq!(
            normalized(&difference),
            vec![vec![contour(&[[1, 1], [2, 1], [2, 4], [1, 4]])]],
            "{:?}",
            solver.strategy
        );
        let intersection = overlay(&boxes, &sliver, OverlayRule::Intersect, solver);
        assert!(intersection.is_empty(), "{:?}: {intersection:?}", solver.strategy);
    }
}

#[test]
fn boxes_minus_sliver_match_jts_snap_rounding() {
    // JTS TestOverlayAAPrec.xml, "AA - sliver triangle with multiple intersecting boxes",
    // difference with a fixed precision model of scale 1.
    let boxes: Vec<_> = (0..5)
        .map(|i| {
            let x = 1 + 2 * i;
            contour(&[[x, 1], [x + 1, 1], [x + 1, 4], [x, 4]])
        })
        .collect();
    let sliver = [contour(&[[0, 2], [11, 2], [11, 3]])];
    let expected = vec![
        vec![contour(&[[1, 1], [2, 1], [2, 4], [1, 4]])],
        vec![contour(&[[3, 1], [4, 1], [4, 4], [3, 4]])],
        vec![contour(&[[5, 1], [6, 1], [6, 2], [5, 2]])],
        vec![contour(&[[5, 2], [6, 3], [6, 4], [5, 4]])],
        vec![contour(&[[7, 1], [8, 1], [8, 2], [7, 2]])],
        vec![contour(&[[7, 3], [8, 3], [8, 4], [7, 4]])],
        vec![contour(&[[9, 1], [10, 1], [10, 2], [9, 2]])],
        vec![contour(&[[9, 3], [10, 3], [10, 4], [9, 4]])],
    ];
    for solver in SOLVERS {
        let difference = overlay(&boxes, &sliver, OverlayRule::Difference, solver);
        assert_eq!(normalized(&difference), expected, "{:?}", solver.strategy);
    }
}
