//! Regressions from https://github.com/iShape-Rust/iOverlay/issues/91.
//! Collinear output simplification must not change hole ownership or hierarchy.

use i_float::int::point::IntPoint;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::{ContourDirection, Overlay};
use i_overlay::core::overlay_rule::OverlayRule;
use i_shape::int::shape::IntShapes;
use i_shape::int::simple::Simplify;

const PANIC_INPUT: &[&[[i32; 2]]] = &[
    &[[4079, 3454], [4090, 3462], [4083, 3471], [4073, 3464]],
    &[[4078, 3464], [4084, 3463], [4081, 3460]],
    &[[4072, 3463], [4080, 3471], [4061, 3471]],
];

const OWNER_INPUT: &[&[[i32; 2]]] = &[
    &[[-4, 3], [-7, 6], [-4, 6]],
    &[[-4, 2], [-1, 5], [-4, 5]],
    &[[-3, 2], [-10, 5], [-6, 5]],
    &[[-3, 0], [-4, 1], [-1, 0]],
    &[[-3, 2], [-3, 5], [-2, 5]],
    &[[-4, 2], [-4, 4], [0, 4]],
    &[[-2, 1], [-5, 1], [-2, 4]],
];

const HIERARCHY_INPUT: &[&[[i32; 2]]] = &[
    &[[-3, -1], [-3, -4], [-7, -4]],
    &[[0, -4], [-1, -4], [-1, 0]],
    &[[0, -4], [-3, -7], [-3, -4]],
    &[[0, -2], [-1, -3], [-1, -1]],
    &[[-1, -1], [-2, -2], [-2, -1]],
    &[[-3, -4], [-3, -7], [-6, -4]],
    &[[0, -2], [-3, -5], [-3, -2]],
];

fn contour(points: &[[i32; 2]]) -> Vec<IntPoint> {
    points.iter().map(|&[x, y]| IntPoint::new(x, y)).collect()
}

fn overlay(
    input: &[&[[i32; 2]]],
    squares: usize,
    ogc: bool,
    preserve_collinear: bool,
    clockwise: bool,
) -> Overlay<i32> {
    let mut contours: Vec<_> = input.iter().map(|p| contour(p)).collect();
    // Disjoint squares force the hole binder across its list/tree threshold.
    for i in 0..squares {
        let x = 100 + 40 * i as i32;
        contours.push(contour(&[[x, 100], [x + 10, 100], [x + 10, 110], [x, 110]]));
    }
    let mut overlay = Overlay::with_contours(&contours, &[]);
    overlay.options.ogc = ogc;
    overlay.options.preserve_output_collinear = preserve_collinear;
    overlay.options.output_direction = if clockwise {
        ContourDirection::Clockwise
    } else {
        ContourDirection::CounterClockwise
    };
    overlay
}

// Compare geometry independently of start vertex, winding, and retained
// collinear vertices. These tests check binding for both output options.
fn canonical(path: &[IntPoint]) -> Vec<IntPoint> {
    let mut path = path.to_vec();
    path.simplify_contour();
    assert!(path.len() >= 3, "unexpected degenerate contour: {path:?}");
    let start = path.iter().enumerate().min_by_key(|(_, p)| **p).unwrap().0;
    path.rotate_left(start);
    if path[1] > path[path.len() - 1] {
        path[1..].reverse();
    }
    path
}

fn assert_hole_owner(shapes: &IntShapes<i32>, hole: &[[i32; 2]], outer: &[[i32; 2]]) {
    let hole = canonical(&contour(hole));
    let owners: Vec<_> = shapes
        .iter()
        .filter(|shape| shape.iter().skip(1).any(|p| canonical(p) == hole))
        .collect();
    assert_eq!(owners.len(), 1, "expected exactly one owner: {shapes:?}");
    assert_eq!(
        canonical(&owners[0][0]),
        canonical(&contour(outer)),
        "hole attached to the wrong shape: {shapes:?}"
    );
    assert_eq!(shapes.iter().map(|s| s.len() - 1).sum::<usize>(), 1);
}

fn check_panic_case(ogc: bool, preserve_collinear: bool, vectors: bool) {
    for clockwise in [false, true] {
        for squares in [29, 30] {
            let mut overlay = overlay(PANIC_INPUT, squares, ogc, preserve_collinear, clockwise);
            let shapes = if vectors {
                overlay
                    .build_shape_vectors(FillRule::NonZero, OverlayRule::Union)
                    .into_iter()
                    .map(|s| {
                        s.into_iter()
                            .map(|p| p.into_iter().map(|e| e.a).collect())
                            .collect()
                    })
                    .collect()
            } else {
                overlay.overlay(OverlayRule::Union, FillRule::NonZero)
            };
            assert_eq!(shapes.len(), squares + 2);
            assert_hole_owner(&shapes, PANIC_INPUT[1], PANIC_INPUT[0]);
        }
    }
}

fn check_owner_case(ogc: bool, preserve_collinear: bool, vectors: bool) {
    for clockwise in [false, true] {
        for squares in [0, 31] {
            let mut overlay = overlay(OWNER_INPUT, squares, ogc, preserve_collinear, clockwise);
            let shapes = if vectors {
                overlay
                    .build_shape_vectors(FillRule::NonZero, OverlayRule::Union)
                    .into_iter()
                    .map(|s| {
                        s.into_iter()
                            .map(|p| p.into_iter().map(|e| e.a).collect())
                            .collect()
                    })
                    .collect()
            } else {
                overlay.overlay(OverlayRule::Union, FillRule::NonZero)
            };
            assert_eq!(shapes.len(), squares + 2);
            assert_hole_owner(
                &shapes,
                &[[-4, 3], [-4, 4], [-3, 2]],
                &[
                    [-6, 5],
                    [-10, 5],
                    [-4, 2],
                    [-5, 1],
                    [-2, 1],
                    [-2, 3],
                    [0, 4],
                    [-2, 4],
                    [-3, 3],
                    [-3, 5],
                    [-4, 5],
                    [-4, 6],
                    [-7, 6],
                ],
            );
        }
    }
}

fn check_hierarchy_case(ogc: bool, preserve_collinear: bool) {
    for clockwise in [false, true] {
        let hierarchy = overlay(HIERARCHY_INPUT, 0, ogc, preserve_collinear, clockwise)
            .overlay_hierarchy(OverlayRule::Union, FillRule::NonZero);
        let shapes = hierarchy.shapes.to_shapes();
        assert_eq!(shapes.len(), 2);
        let triangle = canonical(&contour(HIERARCHY_INPUT[4]));
        let triangle = shapes.iter().find(|s| canonical(&s[0]) == triangle).unwrap();
        assert_eq!(triangle.len(), 1);
        assert_eq!(shapes.iter().map(|s| s.len() - 1).sum::<usize>(), 1);
        assert!(
            hierarchy.links.is_empty(),
            "the separate triangle must have no parent: {hierarchy:?}"
        );
    }
}

mod ordinary {
    #[test]
    fn touching_shapes_bind_hole_across_tree_threshold() {
        super::check_panic_case(false, false, false);
    }

    #[test]
    fn hole_stays_with_containing_shape() {
        super::check_owner_case(false, false, false);
    }

    #[test]
    fn separate_triangle_has_no_hierarchy_parent() {
        super::check_hierarchy_case(false, false);
    }
}

mod ogc {
    #[test]
    fn touching_shapes_bind_hole_across_tree_threshold() {
        super::check_panic_case(true, false, false);
    }

    #[test]
    fn hole_stays_with_containing_shape() {
        super::check_owner_case(true, false, false);
    }

    #[test]
    fn separate_triangle_has_no_hierarchy_parent() {
        super::check_hierarchy_case(true, false);
    }
}

mod ordinary_preserved {
    #[test]
    fn touching_shapes_bind_hole_across_tree_threshold() {
        super::check_panic_case(false, true, false);
    }

    #[test]
    fn hole_stays_with_containing_shape() {
        super::check_owner_case(false, true, false);
    }

    #[test]
    fn separate_triangle_has_no_hierarchy_parent() {
        super::check_hierarchy_case(false, true);
    }
}

mod ogc_preserved {
    #[test]
    fn touching_shapes_bind_hole_across_tree_threshold() {
        super::check_panic_case(true, true, false);
    }

    #[test]
    fn hole_stays_with_containing_shape() {
        super::check_owner_case(true, true, false);
    }

    #[test]
    fn separate_triangle_has_no_hierarchy_parent() {
        super::check_hierarchy_case(true, true);
    }
}

#[test]
fn vector_touching_shapes_bind_hole_across_tree_threshold() {
    check_panic_case(false, false, true);
}

#[test]
fn vector_hole_stays_with_containing_shape() {
    check_owner_case(false, false, true);
}
