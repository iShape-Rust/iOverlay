use std::time::Instant;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::Overlay;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::solver::Solver;
use crate::test::util::Util;

pub(crate) struct CrossTest;

/*

// 5
// Union:

// multithreading on
4     - 0.000011
8     - 0.000023
16     - 0.000042
32     - 0.000091
64     - 0.000220
128     - 0.000625
256     - 0.002053
512     - 0.005073
1024     - 0.013714
2048     - 0.021411
4096     - 0.048197
8192     - 0.157209
16384     - 0.347116
32768     - 1.280415
65536     - 2.305738
131072     - 9.871802
262144     - 16.526387

// multithreading off
4     - 0.000010
8     - 0.000018
16     - 0.000039
32     - 0.000086
64     - 0.000215
128     - 0.000620
256     - 0.002037
512     - 0.004784
1024     - 0.013536
2048     - 0.031879
4096     - 0.066320
8192     - 0.253849
16384     - 0.448397
32768     - 1.870073
65536     - 3.989876
131072     - 15.874618
262144     - 31.306648

*/

// A series of concentric squares, each progressively larger than the last.
impl CrossTest {
    pub(crate) fn run(n: usize, rule: OverlayRule, solver: Solver, scale: f64, simple_geometry: bool) { // 500
        let (subj_paths, clip_paths) = Util::concentric_squares(4, n);

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
                let _ = Overlay::with_contours_custom(&subj_paths, &clip_paths, Default::default(), solver)
                    .overlay(rule, FillRule::NonZero);
            }
        }
        let duration = start.elapsed();
        let time = duration.as_secs_f64() / sq_it_count as f64;

        let polygons_count = 2 * n;

        println!("{}     - {:.6}", polygons_count, time);
    }
}