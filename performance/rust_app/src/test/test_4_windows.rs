use std::time::Instant;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::Overlay;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::solver::Solver;
use i_overlay::i_float::int::point::IntPoint;
use crate::test::util::Util;

pub(crate) struct WindowsTest;
/*
// 4
// Difference:

// multithreading on
8     - 0.000006
32     - 0.000021
128     - 0.000097
512     - 0.000516
2048     - 0.001548
8192     - 0.006780
32768     - 0.034149
131072     - 0.159685
524288     - 0.703147
2097152     - 3.182362
8388608     - 12.058687

// multithreading off
8     - 0.000005
32     - 0.000021
128     - 0.000099
512     - 0.000541
2048     - 0.001745
8192     - 0.007748
32768     - 0.038207
131072     - 0.190786
524288     - 0.862933
2097152     - 3.832362
8388608     - 15.595299

geom multithreading off

8     - 0.000006
32     - 0.000021
128     - 0.000080
512     - 0.000330
2048     - 0.001413
8192     - 0.006229
32768     - 0.030391
131072     - 0.146480
524288     - 0.632447
2097152     - 2.674580
8388608     - 11.443802

*/

// A grid of square frames, each with a smaller square cutout in the center.
impl WindowsTest {
    pub(crate) fn run(n: usize, rule: OverlayRule, solver: Solver, scale: f64, simple_geometry: bool) { // 500
        let offset = 30;
        let x = (n as i32) * offset / 2;
        let origin = IntPoint::new(-x, -x);
        let (subj_paths, clip_paths) = Util::many_windows(origin, 20, 10, offset, n);

        let it_count = ((scale / (n as f64)) as usize).max(1);
        let sq_it_count= it_count * it_count;

        let start = Instant::now();

        if simple_geometry {
            // for _ in 0..sq_it_count {
            //     let _ = Overlay::with_contours(&subj_paths, &clip_paths)
            //         .overlay_45geom_with_min_area_and_solver(rule, FillRule::NonZero, 0, solver);
            // }
        } else {
            for _ in 0..sq_it_count {
                let _ = Overlay::with_contours(&subj_paths, &clip_paths)
                    .overlay_custom(rule, FillRule::NonZero, solver);
            }
        }

        let duration = start.elapsed();
        let time = duration.as_secs_f64() / sq_it_count as f64;

        let polygons_count = 2 * n * n;

        println!("{}     - {:.6}", polygons_count, time);
    }
}