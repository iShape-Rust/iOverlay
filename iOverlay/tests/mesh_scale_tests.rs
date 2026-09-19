use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::float::overlay::FloatOverlay;
use i_overlay::mesh::float::outline::offset::OutlineOffset;
use i_overlay::mesh::float::stroke::offset::StrokeOffset;
use i_overlay::mesh::float::style::{LineJoin, OutlineStyle, StrokeStyle};

fn normalized_area(shapes: &[Vec<Vec<[f64; 2]>>], scale: f64) -> f64 {
    shapes
        .iter()
        .flatten()
        .map(|path| {
            let mut sum = 0.0;
            let mut a = path[path.len() - 1].map(|v| v / scale);
            for p in path {
                let b = p.map(|v| v / scale);
                sum += a[0] * b[1] - a[1] * b[0];
                a = b;
            }
            sum
        })
        .sum::<f64>()
        .abs()
        * 0.5
}

#[test]
fn float_overlay_preserves_rectangles_at_extreme_scales() {
    // Leave room for the rectangle width within the inclusive 2^500 limit.
    for exponent in [496, -600] {
        let scale = 2.0_f64.powi(exponent);
        let rectangle = [
            [0.0, -scale],
            [10.0 * scale, -scale],
            [10.0 * scale, scale],
            [0.0, scale],
        ];
        let shapes = FloatOverlay::with_subj(&rectangle).overlay(OverlayRule::Subject, FillRule::NonZero);
        let area = normalized_area(&shapes, scale);
        assert!(
            (area - 20.0).abs() < 0.001,
            "exponent={exponent}, normalized area={area}"
        );
    }
}

#[test]
#[ignore = "Known extreme-scale mesh bug; deferred by request"]
fn stroke_preserves_relative_area_across_coordinate_scales() {
    for exponent in [0, 400, -400, 496, -600] {
        let scale = 2.0_f64.powi(exponent);
        let path = [[0.0, 0.0], [10.0 * scale, 0.0]];
        let shapes = path.stroke(StrokeStyle::new(2.0 * scale), false);
        let area = normalized_area(&shapes, scale);
        assert!(
            (area - 20.0).abs() < 0.001,
            "exponent={exponent}, normalized area={area}, shapes={shapes:?}"
        );
    }
}

#[test]
#[ignore = "Known extreme-scale mesh bug; deferred by request"]
fn stroke_preserves_relative_area_at_small_coordinates() {
    let scale = 2.0_f64.powi(-600);
    let path = [[0.0, 0.0], [10.0 * scale, 0.0]];
    let shapes = path.stroke(StrokeStyle::new(2.0 * scale), false);
    let area = normalized_area(&shapes, scale);
    assert!(
        (area - 20.0).abs() < 0.001,
        "normalized area={area}, shapes={shapes:?}"
    );
}

#[test]
#[ignore = "Known extreme-scale mesh bug; deferred by request"]
fn f32_stroke_preserves_relative_area_at_large_coordinates() {
    // The path and stroke padding must fit within 2^60.
    check_f32_stroke(56);
}

#[test]
#[ignore = "Known extreme-scale mesh bug; deferred by request"]
fn f32_stroke_preserves_relative_area_at_small_coordinates() {
    check_f32_stroke(-80);
}

fn check_f32_stroke(exponent: i32) {
    let scale = 2.0_f32.powi(exponent);
    let path = [[0.0, 0.0], [10.0 * scale, 0.0]];
    let shapes = path.stroke(StrokeStyle::new(2.0 * scale), false);
    let shapes: Vec<Vec<Vec<[f64; 2]>>> = shapes
        .into_iter()
        .map(|s| {
            s.into_iter()
                .map(|c| c.into_iter().map(|p| p.map(f64::from)).collect())
                .collect()
        })
        .collect();
    let area = normalized_area(&shapes, f64::from(scale));
    assert!(
        (area - 20.0).abs() < 0.001,
        "exponent={exponent}, normalized area={area}, shapes={shapes:?}"
    );
}

#[test]
#[ignore = "Known extreme-scale mesh bug; deferred by request"]
fn bevel_outline_preserves_relative_area_at_large_coordinates() {
    let scale = 2.0_f64.powi(496);
    let path = [
        [0.0, 0.0],
        [10.0 * scale, 0.0],
        [10.0 * scale, 10.0 * scale],
        [0.0, 10.0 * scale],
    ];
    let shapes = path.outline(&OutlineStyle::new(scale));
    let area = normalized_area(&shapes, scale);
    assert!(
        (area - 142.0).abs() < 0.001,
        "normalized area={area}, shapes={shapes:?}"
    );
}

#[test]
#[ignore = "Known extreme-scale mesh bug; deferred by request"]
fn outline_preserves_relative_area_across_coordinate_scales() {
    for exponent in [0, 400, -400, 496, -600] {
        let scale = 2.0_f64.powi(exponent);
        let path = [
            [0.0, 0.0],
            [10.0 * scale, 0.0],
            [10.0 * scale, 10.0 * scale],
            [0.0, 10.0 * scale],
        ];
        let shapes = path.outline(&OutlineStyle::new(scale).line_join(LineJoin::Miter(0.2)));
        let area = normalized_area(&shapes, scale);
        assert!(
            (area - 144.0).abs() < 0.001,
            "exponent={exponent}, normalized area={area}, shapes={shapes:?}"
        );
    }
}
