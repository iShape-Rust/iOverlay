use crate::test::util::{OverlayInt, Util};
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::Overlay;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::solver::Solver;
use std::time::Instant;

pub(crate) struct CrossTest;

/*

// 5
// Union:

i16

// multithreading on
4     - 0.000004
8     - 0.000007
16     - 0.000015
32     - 0.000034
64     - 0.000083
128     - 0.000228
256     - 0.000698
512     - 0.002015
1024     - 0.005271
2048     - 0.007070
4096     - 0.058529

// multithreading off
4     - 0.000004
8     - 0.000007
16     - 0.000015
32     - 0.000034
64     - 0.000082
128     - 0.000224
256     - 0.000651
512     - 0.002065
1024     - 0.005356
2048     - 0.009238
4096     - 0.057300

i32

// multithreading on
4     - 0.000004
8     - 0.000008
16     - 0.000015
32     - 0.000033
64     - 0.000080
128     - 0.000214
256     - 0.000672
512     - 0.002109
1024     - 0.005426
2048     - 0.007757
4096     - 0.014683
8192     - 0.046014
16384     - 0.087646
32768     - 0.322707
65536     - 0.653741
131072     - 2.410910

// multithreading off
4     - 0.000004
8     - 0.000007
16     - 0.000015
32     - 0.000033
64     - 0.000079
128     - 0.000213
256     - 0.000659
512     - 0.002060
1024     - 0.005223
2048     - 0.011609
4096     - 0.023309
8192     - 0.081856
16384     - 0.181416
32768     - 0.667394
65536     - 1.401006
131072     - 5.445065

*/

// A series of concentric squares, each progressively larger than the last.
impl CrossTest {
    pub(crate) fn run<I: OverlayInt>(n: usize, rule: OverlayRule, solver: Solver, scale: f64) {
        // 500
        if Util::skip_if_out_of_range::<I>(n, 8 * n + 8) {
            return;
        }

        let (subj_paths, clip_paths) = Util::concentric_squares(I::from_usize(4), n);

        let it_count = ((scale / (n as f64)) as usize).max(1);
        let sq_it_count = it_count * it_count;

        let start = Instant::now();

        for _ in 0..sq_it_count {
            let _ =
                Overlay::with_contours_custom(&subj_paths, &clip_paths, Default::default(), solver)
                    .overlay(rule, FillRule::NonZero);
        }
        let duration = start.elapsed();
        let time = duration.as_secs_f64() / sq_it_count as f64;

        let polygons_count = 2 * n;

        println!("{}     - {:.6}", polygons_count, time);
    }
}
