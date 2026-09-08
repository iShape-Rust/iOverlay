use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::integer::OverlayInt;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::float::overlay::FloatOverlay;

fn check_subject_near_scale_boundary<I: OverlayInt>(side: f64, grid_step: f64) {
    let subject = [[0.0, 0.0], [side, side], [0.0, side]];
    let clip = [
        [side - grid_step, side],
        [side, side - grid_step],
        [side, side],
    ];

    // Both inputs have the same combined bounds as the subject alone. The
    // small clip crosses its diagonal near the top-right corner. Extracting
    // Subject must preserve that triangle, regardless of the clip.
    let expected = FloatOverlay::<[f64; 2], I>::from_subj(&subject)
        .overlay(OverlayRule::Subject, FillRule::NonZero);
    assert_eq!(expected.len(), 1);
    let actual = FloatOverlay::<[f64; 2], I>::from_subj_and_clip(&subject, &clip)
        .overlay(OverlayRule::Subject, FillRule::NonZero);
    assert_eq!(actual, expected);
}

#[test]
fn automatic_i16_scale_handles_intersection_near_extreme_corner() {
    check_subject_near_scale_boundary::<i16>(3.9999, 1.0 / 8192.0);
}

#[test]
fn automatic_i32_scale_handles_intersection_near_extreme_corner() {
    check_subject_near_scale_boundary::<i32>(3.999999999, 1.0 / 536870912.0);
}
