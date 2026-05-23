use crate::test::util::{OverlayInt, Util};
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::Overlay;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::solver::Solver;
use std::time::Instant;

pub(crate) struct LinesNetTest;

/*
test 2
LinesNetTest
Intersection:

i16

multithreading on

4     - 0.000002
8     - 0.000006
16     - 0.000021
32     - 0.000092
64     - 0.000403
128     - 0.001716
256     - 0.007047
512     - 0.030516
1024     - 0.130099
2048     - 0.569064

multithreading off

4     - 0.000002
8     - 0.000006
16     - 0.000020
32     - 0.000091
64     - 0.000392
128     - 0.001717
256     - 0.008280
512     - 0.037633
1024     - 0.168466
2048     - 0.751282

i32

multithreading on

4     - 0.000002
8     - 0.000006
16     - 0.000020
32     - 0.000087
64     - 0.000423
128     - 0.001829
256     - 0.007510
512     - 0.032208
1024     - 0.145159
2048     - 0.622211
4096     - 2.687778

multithreading off

4     - 0.000002
8     - 0.000006
16     - 0.000020
32     - 0.000087
64     - 0.000425
128     - 0.001791
256     - 0.008733
512     - 0.039941
1024     - 0.181488
2048     - 0.806123
4096     - 3.557761

i64

multithreading on

4     - 0.000002
8     - 0.000007
16     - 0.000025
32     - 0.000114
64     - 0.000497
128     - 0.002118
256     - 0.008971
512     - 0.041422
1024     - 0.181670
2048     - 0.780686
4096     - 3.220613

multithreading off

4     - 0.000002
8     - 0.000007
16     - 0.000025
32     - 0.000113
64     - 0.000486
128     - 0.002161
256     - 0.010305
512     - 0.048515
1024     - 0.219835
2048     - 0.991209
4096     - 4.420209

*/

// A grid is formed by the intersection of a set of vertical and horizontal lines.
impl LinesNetTest {
    pub(crate) fn run<I: OverlayInt>(n: usize, rule: OverlayRule, solver: Solver, scale: f64) {
        // 500
        if Util::skip_if_out_of_range::<I>(n, 10 * n + 10) {
            return;
        }

        let subj_paths = Util::many_lines_x(I::from_usize(20), n);
        let clip_paths = Util::many_lines_y(I::from_usize(20), n);

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
