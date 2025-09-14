use std::time::Instant;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::float::simplify::SimplifyShape;
use crate::test::util::Util;

pub(crate) struct SpiralTest;

/*

// 3
// Intersection:


multithreading on

2     - 0.000002
4     - 0.000005
8     - 0.000010
16     - 0.000022
32     - 0.000053
64     - 0.000138
128     - 0.000318
256     - 0.000698
512     - 0.001734
1024     - 0.003680
2048     - 0.008326
4096     - 0.011125
8192     - 0.019165
16384     - 0.041279
32768     - 0.082171
65536     - 0.179237
131072     - 0.341959
262144     - 0.785235
524288     - 1.345209
1048576     - 3.039988

multithreading off

2     - 0.000003
4     - 0.000005
8     - 0.000010
16     - 0.000022
32     - 0.000054
64     - 0.000138
128     - 0.000316
256     - 0.000727
512     - 0.001767
1024     - 0.003605
2048     - 0.008468
4096     - 0.010696
8192     - 0.019332
16384     - 0.041967
32768     - 0.078037
65536     - 0.175368
131072     - 0.323782
262144     - 0.823820
524288     - 1.425306
1048576     - 3.029298
 */

// Two irregular self-intersecting polygons are generated, the vertices of which are defined by a fixed radius and angle.
impl SpiralTest {
    pub(crate) fn run(n: usize, scale: f64) { // 1000
        let subj_path = Util::spiral(n, 100.0);

        let it_count = ((scale / (n as f64)) as usize).max(1);
        let sq_it_count= it_count * it_count;

        let start = Instant::now();

        for _ in 0..sq_it_count {
            let _ = subj_path.simplify_shape(FillRule::NonZero);
        }

        let duration = start.elapsed();
        let time = duration.as_secs_f64() / sq_it_count as f64;

        let polygons_count = n;

        println!("{}     - {:.6}", polygons_count, time);
    }
}