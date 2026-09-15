use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::solver::Solver;
use i_overlay::float::overlay::{FloatOverlay, OverlayOptions};

fn square(side: f64) -> [[f64; 2]; 4] {
    [[0.0, 0.0], [side, 0.0], [side, side], [0.0, side]]
}

fn area_options() -> OverlayOptions<f64> {
    let mut options = OverlayOptions::default();
    options.min_output_area = 2.0;
    options
}

#[test]
fn reinit_subject_preserves_minimum_area_in_float_units() {
    for (old_side, new_side, expected_count) in [(1.0, 4.0, 1), (4.0, 1.0, 0)] {
        let options = area_options();
        let subject = square(new_side);
        let mut reused = FloatOverlay::with_subj_custom(&square(old_side), options, Solver::AUTO);
        reused.reinit_with_subj(&subject);

        let expected = FloatOverlay::with_subj_custom(&subject, options, Solver::AUTO)
            .overlay(OverlayRule::Subject, FillRule::NonZero);
        assert_eq!(expected.len(), expected_count);
        assert_eq!(
            reused.overlay(OverlayRule::Subject, FillRule::NonZero),
            expected,
            "reinitializing from side {old_side} to {new_side} must retain the area threshold"
        );
    }
}

#[test]
fn reinit_subject_and_clip_preserves_minimum_area_in_float_units() {
    for (old_side, new_side, expected_count) in [(1.0, 4.0, 1), (4.0, 1.0, 0)] {
        let options = area_options();
        let subject = square(new_side);
        let clip = square(new_side / 2.0);
        let mut reused = FloatOverlay::with_subj_and_clip_custom(
            &square(old_side),
            &square(old_side / 2.0),
            options,
            Solver::AUTO,
        );
        reused.reinit_with_subj_and_clip(&subject, &clip);

        let expected =
            FloatOverlay::with_subj_and_clip_custom(&subject, &clip, options, Solver::AUTO)
                .overlay(OverlayRule::Union, FillRule::NonZero);
        assert_eq!(expected.len(), expected_count);
        assert_eq!(
            reused.overlay(OverlayRule::Union, FillRule::NonZero),
            expected,
            "reinitializing from side {old_side} to {new_side} must retain the area threshold"
        );
    }
}
