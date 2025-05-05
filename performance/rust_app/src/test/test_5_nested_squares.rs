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
4     - 0.000009
8     - 0.000017
16     - 0.000034
32     - 0.000081
64     - 0.000217
128     - 0.000608
256     - 0.002016
512     - 0.002641
1024     - 0.005925
2048     - 0.018777
4096     - 0.044756
8192     - 0.165539
16384     - 0.331655
32768     - 1.148905
65536     - 2.197493
131072     - 8.194153
262144     - 15.285741

// multithreading off
4     - 0.000009
8     - 0.000018
16     - 0.000039
32     - 0.000089
64     - 0.000219
128     - 0.000638
256     - 0.002004
512     - 0.002683
1024     - 0.008269
2048     - 0.028743
4096     - 0.058336
8192     - 0.237410
16384     - 0.460193
32768     - 1.816680
65536     - 4.008331
131072     - 15.923762
262144     - 31.619456


geom multithreading off

4     - 0.000009
8     - 0.000017
16     - 0.000033
32     - 0.000079
64     - 0.000141
128     - 0.000322
256     - 0.000728
512     - 0.001900
1024     - 0.002963
2048     - 0.006745
4096     - 0.014768
8192     - 0.032764
16384     - 0.073155
32768     - 0.179328
65536     - 0.419386
131072     - 1.044835
262144     - 2.463408
524288     - 6.518206

geom map

4     - 0.000009
8     - 0.000019
16     - 0.000044
32     - 0.000089
64     - 0.000167
128     - 0.000387
256     - 0.000791
512     - 0.001944
1024     - 0.003577
2048     - 0.006597
4096     - 0.014917
8192     - 0.031029
16384     - 0.068286
32768     - 0.149406
65536     - 0.351961
131072     - 0.777655
262144     - 2.034850
524288     - 4.438061


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
                let _ = Overlay::with_contours(&subj_paths, &clip_paths)
                    .overlay_custom(rule, FillRule::NonZero, solver);
            }
        }
        let duration = start.elapsed();
        let time = duration.as_secs_f64() / sq_it_count as f64;

        let polygons_count = 2 * n;

        println!("{}     - {:.6}", polygons_count, time);
    }
}