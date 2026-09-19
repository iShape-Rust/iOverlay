use i_overlay::float::overlay::OverlayOptions;
use i_overlay::mesh::float::stroke::offset::StrokeOffset;
use i_overlay::mesh::float::style::{LineCap, StrokeStyle};
use i_overlay::mesh::math::MathMode;

#[test]
fn round_caps_survive_coarse_integer_grid() {
    for math in [MathMode::Integer, MathMode::Float] {
        let path = [[0.0, 0.0], [10.0, 0.0]];
        let style = StrokeStyle::new(4.0)
            .math(math)
            .start_cap(LineCap::Round(0.01))
            .end_cap(LineCap::Round(0.01));
        let output = path.stroke_fixed_scale(style, false, 1.0).unwrap();
        assert_eq!(output.len(), 1);
        assert_eq!(output[0].len(), 1);
        assert!(output[0][0].len() >= 4);
    }
}

#[test]
fn round_caps_survive_coarse_grid_without_cleanup() {
    for math in [MathMode::Integer, MathMode::Float] {
        let path = [[0.0, 0.0], [10.0, 0.0]];
        let style = StrokeStyle::new(4.0)
            .math(math)
            .start_cap(LineCap::Round(0.01))
            .end_cap(LineCap::Round(0.01));
        let mut options = OverlayOptions::default();
        options.clean_result = false;
        let output = path
            .stroke_custom_fixed_scale(style, false, options, 1.0)
            .unwrap();
        assert_eq!(output.len(), 1);
    }
}

#[test]
fn coarse_butt_and_square_caps_and_fine_round_caps_control() {
    for math in [MathMode::Integer, MathMode::Float] {
        let path = [[0.0, 0.0], [10.0, 0.0]];
        for (cap, scale) in [
            (LineCap::Butt, 1.0),
            (LineCap::Square, 1.0),
            (LineCap::Round(0.01), 100.0),
        ] {
            let style = StrokeStyle::new(4.0)
                .math(math)
                .start_cap(cap.clone())
                .end_cap(cap.clone());
            let output = path.stroke_fixed_scale(style, false, scale).unwrap();
            assert_eq!(output.len(), 1, "cap={cap:?}, scale={scale}");
        }
    }
}

#[test]
fn custom_cap_without_repeated_points_control() {
    for math in [MathMode::Integer, MathMode::Float] {
        let path = [[0.0, 0.0], [10.0, 0.0]];
        let cap = LineCap::Custom(std::rc::Rc::from([[1.0, -1.0], [1.0, 1.0]]));
        let style = StrokeStyle::new(4.0)
            .math(math)
            .start_cap(cap.clone())
            .end_cap(cap);
        let output = path.stroke(style, false);
        assert_eq!(output.len(), 1);
    }
}

#[test]
fn repeated_custom_cap_points_preserve_stroke() {
    const TEST_NAME: &str = "repeated_custom_cap_points_preserve_stroke";
    const CHILD_ENV: &str = "I_OVERLAY_REPEATED_CAP_TEST_CHILD";
    if std::env::var_os(CHILD_ENV).is_none() {
        // A malformed graph may never finish extracting a contour. Keep this
        // regression bounded even when the underlying bug is present.
        let mut child = std::process::Command::new(std::env::current_exe().unwrap())
            .args(["--exact", TEST_NAME, "--nocapture"])
            .env(CHILD_ENV, "1")
            .spawn()
            .unwrap();
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(3);
        loop {
            if let Some(status) = child.try_wait().unwrap() {
                assert!(status.success(), "custom cap child failed: {status}");
                return;
            }
            if std::time::Instant::now() >= deadline {
                child.kill().unwrap();
                child.wait().unwrap();
                panic!("a two-point stroke with a repeated custom cap point did not finish within 3 seconds");
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
    let path = [[0.0, 0.0], [10.0, 0.0]];
    let cap = LineCap::Custom(std::rc::Rc::from([[1.0, -1.0], [1.0, -1.0], [1.0, 1.0]]));
    let style = StrokeStyle::new(4.0).start_cap(cap.clone()).end_cap(cap);
    let output = path.stroke(style, false);
    assert_eq!(
        output.len(),
        1,
        "duplicating a cap template point must not erase a stroke"
    );
}
