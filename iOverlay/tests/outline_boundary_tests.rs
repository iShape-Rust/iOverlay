use i_overlay::mesh::outline::offset::OutlineOffset;
use i_overlay::mesh::style::{LineJoin, OutlineStyle};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

// A broken subdivision count can make an outline loop effectively forever.
// Keep the regression bounded in both debug and release builds.
fn check_round_outline_in_subprocess(test_name: &str, angle: f64) {
    const CHILD_ENV: &str = "I_OVERLAY_OUTLINE_BOUNDARY_TEST_CHILD";
    if std::env::var(CHILD_ENV).as_deref() == Ok(test_name) {
        let square = [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]];
        let style = OutlineStyle::new(1.0).line_join(LineJoin::Round(angle));
        let result = square.outline(&style);
        assert_eq!(result.len(), 1);
        assert!(
            result
                .iter()
                .flatten()
                .flatten()
                .all(|p| p[0].is_finite() && p[1].is_finite())
        );
        return;
    }

    let mut child = Command::new(std::env::current_exe().unwrap())
        .args(["--exact", test_name, "--nocapture"])
        .env(CHILD_ENV, test_name)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(3);
    loop {
        if child.try_wait().unwrap().is_some() {
            let output = child.wait_with_output().unwrap();
            assert!(
                output.status.success(),
                "outline with round angle {angle} failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            return;
        }
        if Instant::now() >= deadline {
            child.kill().unwrap();
            child.wait().unwrap();
            panic!(
                "outline of a four-vertex square with round angle {angle} did not finish within 3 seconds"
            );
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}

#[test]
fn hypothesis_outline_zero_round_angle_terminates() {
    check_round_outline_in_subprocess("hypothesis_outline_zero_round_angle_terminates", 0.0);
}

#[test]
fn hypothesis_outline_tiny_positive_round_angle_terminates() {
    check_round_outline_in_subprocess("hypothesis_outline_tiny_positive_round_angle_terminates", 1e-20);
}

#[test]
fn outline_normal_round_angle_control() {
    check_round_outline_in_subprocess("outline_normal_round_angle_control", 0.1);
}

fn square(lo: f64, hi: f64, clockwise: bool) -> Vec<[f64; 2]> {
    let mut path = vec![[lo, lo], [hi, lo], [hi, hi], [lo, hi]];
    if clockwise {
        path.reverse();
    }
    path
}

#[test]
fn hypothesis_collapsed_hole_does_not_erase_other_holes() {
    let outer = square(0.0, 30.0, false);
    let collapsed_hole = square(3.0, 5.0, true);
    let surviving_hole = square(10.0, 20.0, true);
    let style = OutlineStyle::new(1.0);

    let expected = vec![outer.clone(), surviving_hole.clone()]
        .outline_fixed_scale(&style, 100.0)
        .unwrap();
    assert_eq!(expected.len(), 1);
    assert_eq!(expected[0].len(), 2, "control must retain the large hole");

    let actual = vec![outer, collapsed_hole, surviving_hole]
        .outline_fixed_scale(&style, 100.0)
        .unwrap();
    assert_eq!(
        actual, expected,
        "a collapsed hole must not change any other contour"
    );
}
