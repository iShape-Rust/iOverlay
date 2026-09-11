use i_float::int::point::IntPoint;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::Overlay;
use i_overlay::core::overlay_rule::OverlayRule;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

#[test]
fn ogc_contour_marking_completes_a_closed_tour() {
    const CHILD_ENV: &str = "I_OVERLAY_OGC_CLOSED_TOUR_CHILD";
    if std::env::var_os(CHILD_ENV).is_some() {
        let paths = [
            [
                [8, -14],
                [-12, -13],
                [7, -10],
                [1, -3],
                [-14, 8],
                [6, -16],
                [8, 20],
                [7, -20],
                [-7, 6],
                [11, -6],
            ],
            [
                [-17, -2],
                [-11, -15],
                [-18, 2],
                [-6, 19],
                [15, 3],
                [-2, -19],
                [-6, -4],
                [11, -14],
                [17, 10],
                [-19, -17],
            ],
            [
                [8, -18],
                [-8, -11],
                [9, 3],
                [11, -5],
                [-16, 1],
                [-1, -1],
                [-2, 13],
                [0, -11],
                [-17, -18],
                [-18, 12],
            ],
        ]
        .map(|path| path.map(|p| IntPoint::new(p[0], p[1])).to_vec());
        let mut overlay = Overlay::with_contours(&paths, &[]);
        overlay.options.ogc = true;
        assert!(
            !overlay
                .overlay(OverlayRule::Subject, FillRule::EvenOdd)
                .is_empty()
        );
        return;
    }

    // An unreachable final edge can keep the marking loop running forever.
    // Run the regression in a child so failure does not hang the test suite.
    let mut child = Command::new(std::env::current_exe().unwrap())
        .args([
            "--exact",
            "ogc_contour_marking_completes_a_closed_tour",
            "--nocapture",
        ])
        .env(CHILD_ENV, "1")
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(3);
    loop {
        if child.try_wait().unwrap().is_some() {
            let output = child.wait_with_output().unwrap();
            assert!(
                output.status.success(),
                "{}",
                String::from_utf8_lossy(&output.stderr)
            );
            return;
        }
        if Instant::now() >= deadline {
            child.kill().unwrap();
            child.wait().unwrap();
            panic!("OGC extraction of 30 input vertices did not finish within 3 seconds");
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}
