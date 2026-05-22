use crate::test::util::{OverlayInt, Util};
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::solver::Solver;
use i_overlay::float::overlay::FloatOverlay;
use i_overlay::i_float::float::point::FloatPoint;
use std::time::Instant;

pub(crate) struct SpiralTest;

/*

// 3
// Intersection:

i16

multithreading on

2     - 0.000001
4     - 0.000002
8     - 0.000005
16     - 0.000007
32     - 0.000015
64     - 0.000032
128     - 0.000079
256     - 0.000249
512     - 0.000715
1024     - 0.001833
2048     - 0.004278
4096     - 0.004283
8192     - 0.008659
16384     - 0.017107
32768     - 0.033244
65536     - 0.066839
131072     - 0.131752
262144     - 0.291700
524288     - 0.455038
1048576     - 0.945685

multithreading off

2     - 0.000001
4     - 0.000002
8     - 0.000005
16     - 0.000007
32     - 0.000015
64     - 0.000031
128     - 0.000082
256     - 0.000258
512     - 0.000736
1024     - 0.001867
2048     - 0.004070
4096     - 0.005567
8192     - 0.010481
16384     - 0.022516
32768     - 0.045402
65536     - 0.095105
131072     - 0.198492
262144     - 0.431128
524288     - 0.661296
1048576     - 1.435810

i32

multithreading on

2     - 0.000001
4     - 0.000002
8     - 0.000005
16     - 0.000010
32     - 0.000020
64     - 0.000042
128     - 0.000104
256     - 0.000245
512     - 0.000748
1024     - 0.001984
2048     - 0.004483
4096     - 0.006270
8192     - 0.009434
16384     - 0.019526
32768     - 0.034397
65536     - 0.074229
131072     - 0.146095
262144     - 0.350941
524288     - 0.744520
1048576     - 1.626030

multithreading off

2     - 0.000001
4     - 0.000002
8     - 0.000005
16     - 0.000010
32     - 0.000021
64     - 0.000043
128     - 0.000092
256     - 0.000298
512     - 0.000790
1024     - 0.002041
2048     - 0.004207
4096     - 0.007974
8192     - 0.012796
16384     - 0.028001
32768     - 0.050997
65536     - 0.113348
131072     - 0.223653
262144     - 0.541526
524288     - 1.034178
1048576     - 2.303664

i64

multithreading on

multithreading off

 */

// Two irregular self-intersecting polygons are generated, the vertices of which are defined by a fixed radius and angle.
impl SpiralTest {
    pub(crate) fn run<I: OverlayInt>(n: usize, solver: Solver, scale: f64) {
        // 1000
        let subj_path = Util::spiral(n, 100.0);

        let it_count = ((scale / (n as f64)) as usize).max(1);
        let sq_it_count = it_count * it_count;

        let start = Instant::now();

        for _ in 0..sq_it_count {
            let _ = FloatOverlay::<FloatPoint<f64>, I>::from_subj_custom(
                &subj_path,
                Default::default(),
                solver,
            )
            .overlay(OverlayRule::Subject, FillRule::NonZero);
        }

        let duration = start.elapsed();
        let time = duration.as_secs_f64() / sq_it_count as f64;

        let polygons_count = n;

        println!("{}     - {:.6}", polygons_count, time);
    }
}
