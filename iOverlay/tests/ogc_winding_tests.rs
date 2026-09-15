use i_float::int::point::IntPoint;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::{ContourDirection, Overlay};
use i_overlay::core::overlay_rule::OverlayRule;

fn touching_hole() -> Vec<IntPoint> {
    [
        [0, 0],
        [10, 0],
        [10, 10],
        [0, 10],
        [0, 5],
        [2, 7],
        [4, 5],
        [2, 3],
        [0, 5],
    ]
    .map(|p| IntPoint::new(p[0], p[1]))
    .to_vec()
}

fn double_area(path: &[IntPoint]) -> i64 {
    path.iter()
        .zip(path.iter().cycle().skip(1))
        .map(|(a, b)| i64::from(a.x) * i64::from(b.y) - i64::from(a.y) * i64::from(b.x))
        .sum()
}

#[test]
fn ogc_touching_hole_has_opposite_winding() {
    check_touching_hole(false);
}

#[test]
fn clockwise_ogc_touching_hole_has_opposite_winding() {
    check_touching_hole(true);
}

fn check_touching_hole(clockwise: bool) {
    let mut overlay = Overlay::with_contour(&touching_hole(), &[]);
    overlay.options.ogc = true;
    overlay.options.output_direction = if clockwise {
        ContourDirection::Clockwise
    } else {
        ContourDirection::CounterClockwise
    };
    let shapes = overlay.overlay(OverlayRule::Subject, FillRule::NonZero);
    assert_eq!(shapes.len(), 1);
    assert_eq!(shapes[0].len(), 2);
    assert_eq!(double_area(&shapes[0][0]) > 0, !clockwise);
    assert_eq!(
        double_area(&shapes[0][1]) > 0,
        clockwise,
        "clockwise={clockwise}, shapes={shapes:?}"
    );
}

#[test]
fn ordinary_extraction_preserves_touching_hole_area() {
    let shapes =
        Overlay::with_contour(&touching_hole(), &[]).overlay(OverlayRule::Subject, FillRule::NonZero);
    let area: i64 = shapes.iter().flatten().map(|p| double_area(p)).sum();
    assert_eq!(area, 184);
}

#[test]
fn ogc_output_preserves_holes_when_reused_with_nonzero_fill() {
    let path = touching_hole();
    let mut overlay = Overlay::with_contour(&path, &[]);
    overlay.options.ogc = true;
    let shapes = overlay.overlay(OverlayRule::Subject, FillRule::NonZero);
    let contours: Vec<_> = shapes.into_iter().flatten().collect();
    let result = Overlay::with_contours(&contours, &[]).overlay(OverlayRule::Subject, FillRule::NonZero);
    let area: i64 = result.iter().flatten().map(|p| double_area(p)).sum();
    assert_eq!(
        area, 184,
        "the area must remain 100 - 8 after reusing OGC output; result={result:?}"
    );
}
