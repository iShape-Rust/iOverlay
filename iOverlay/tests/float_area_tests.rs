use i_float::adapter::FloatPointAdapter;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::solver::Solver;
use i_overlay::float::overlay::{FloatOverlay, OverlayOptions};

#[test]
fn f32_i64_area_filter_preserves_shapes_above_threshold() {
    let square = [[0.0_f32, 0.0], [0.01, 0.0], [0.01, 0.01], [0.0, 0.01]];
    // Square area is 1e-4, one hundred times the filtering threshold.
    let expected =
        FloatOverlay::<[f32; 2], i64>::from_subj(&square).overlay(OverlayRule::Subject, FillRule::NonZero);
    assert_eq!(expected.len(), 1);
    let mut options = OverlayOptions::<f32, i64>::default();
    options.min_output_area = 1e-6;
    let actual = FloatOverlay::<[f32; 2], i64>::from_subj_custom(&square, options, Solver::AUTO)
        .overlay(OverlayRule::Subject, FillRule::NonZero);
    assert_eq!(
        actual, expected,
        "a positive area threshold below the square area must retain it"
    );
}

#[test]
fn adapter_area_conversion_avoids_intermediate_overflow() {
    let square = [[0.0_f32, 0.0], [0.01, 0.0], [0.01, 0.01], [0.0, 0.01]];
    let adapter = FloatPointAdapter::<[f32; 2], i64>::with_iter(square.iter());
    let threshold = 1e-6_f32;
    let scale = f64::from(adapter.dir_scale());
    let expected = (scale * scale * f64::from(threshold)).round() as i128;
    assert!(expected > 0 && expected < i128::MAX);
    assert_eq!(adapter.round_sqr_len_to_int(threshold), expected);
}

#[test]
fn f32_i32_area_filter_preserves_shapes_above_threshold() {
    let square = [[0.0_f32, 0.0], [0.01, 0.0], [0.01, 0.01], [0.0, 0.01]];
    let mut options = OverlayOptions::<f32, i32>::default();
    options.min_output_area = 1e-6;
    let actual = FloatOverlay::<[f32; 2], i32>::from_subj_custom(&square, options, Solver::AUTO)
        .overlay(OverlayRule::Subject, FillRule::NonZero);
    assert_eq!(actual.len(), 1);
}

#[test]
fn f64_i64_area_filter_preserves_shapes_above_threshold() {
    let square = [[0.0_f64, 0.0], [0.01, 0.0], [0.01, 0.01], [0.0, 0.01]];
    let mut options = OverlayOptions::<f64, i64>::default();
    options.min_output_area = 1e-6;
    let actual = FloatOverlay::<[f64; 2], i64>::from_subj_custom(&square, options, Solver::AUTO)
        .overlay(OverlayRule::Subject, FillRule::NonZero);
    assert_eq!(actual.len(), 1);
}

#[test]
fn f32_i32_area_filter_preserves_tiny_shapes() {
    let square = [[0.0_f32, 0.0], [1e-12, 0.0], [1e-12, 1e-12], [0.0, 1e-12]];
    let expected =
        FloatOverlay::<[f32; 2], i32>::from_subj(&square).overlay(OverlayRule::Subject, FillRule::NonZero);
    assert_eq!(expected.len(), 1);
    let mut options = OverlayOptions::<f32, i32>::default();
    options.min_output_area = 1e-26;
    let actual = FloatOverlay::<[f32; 2], i32>::from_subj_custom(&square, options, Solver::AUTO)
        .overlay(OverlayRule::Subject, FillRule::NonZero);
    assert_eq!(actual, expected);
}
