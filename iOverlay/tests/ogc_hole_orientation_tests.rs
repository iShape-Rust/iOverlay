use i_float::int::point::IntPoint;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::{ContourDirection, Overlay};
use i_overlay::core::overlay_rule::OverlayRule;

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
        let mut overlay = Overlay::with_contour(&path, &[]);
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
    let mut overlay = Overlay::with_contour(&touching_hole(), &[]);
    overlay.options.ogc = true;
    let shapes = overlay.overlay(OverlayRule::Subject, FillRule::NonZero);
    let contours: Vec<_> = shapes.into_iter().flatten().collect();
    let result = Overlay::with_contours(&contours, &[]).overlay(OverlayRule::Subject, FillRule::NonZero);
    let area: i64 = result.iter().flatten().map(|p| double_area(p)).sum();
    assert_eq!(
        area, 10,
        "area must remain 6 - 1 after reusing OGC output: {result:?}"
    );
}

#[test]
fn ordinary_extraction_preserves_leftmost_touching_hole_area() {
    let shapes =
        Overlay::with_contour(&touching_hole(), &[]).overlay(OverlayRule::Subject, FillRule::NonZero);
    let area: i64 = shapes.iter().flatten().map(|p| double_area(p)).sum();
    assert_eq!(area, 10);
}
