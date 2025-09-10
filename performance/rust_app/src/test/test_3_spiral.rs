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
8     - 0.000009
16     - 0.000020
32     - 0.000048
64     - 0.000127
128     - 0.000305
256     - 0.000669
512     - 0.001606
1024     - 0.003560
2048     - 0.004930
4096     - 0.009528
8192     - 0.018779
16384     - 0.040263
32768     - 0.076609
65536     - 0.181387
131072     - 0.331046
262144     - 0.833816
524288     - 1.472624
1048576     - 3.232834

multithreading off

2     - 0.000002
4     - 0.000005
8     - 0.000009
16     - 0.000020
32     - 0.000048
64     - 0.000129
128     - 0.000304
256     - 0.000668
512     - 0.001599
1024     - 0.003572
2048     - 0.005000
4096     - 0.009576
8192     - 0.017583
16384     - 0.040145
32768     - 0.076642
65536     - 0.181912
131072     - 0.343917
262144     - 0.781770
524288     - 1.417144
1048576     - 3.188509
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