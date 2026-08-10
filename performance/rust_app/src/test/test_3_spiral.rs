use crate::test::util::{OverlayInt, Util};
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::solver::Solver;
use i_overlay::float::overlay::FloatOverlay;
use i_overlay::i_float::float::point::FloatPoint;
use std::time::Instant;

pub(crate) struct SpiralTest;

/*
test 3
SpiralTest
Intersection:

i16

multithreading on

2     - 0.000001
4     - 0.000002
8     - 0.000005
16     - 0.000007
32     - 0.000015
64     - 0.000031
128     - 0.000076
256     - 0.000279
512     - 0.000764
1024     - 0.001962
2048     - 0.003950
4096     - 0.004374
8192     - 0.008446
16384     - 0.017347
32768     - 0.033183
65536     - 0.065167
131072     - 0.131789
262144     - 0.301777
524288     - 0.456391

multithreading off

2     - 0.000001
4     - 0.000002
8     - 0.000005
16     - 0.000007
32     - 0.000015
64     - 0.000031
128     - 0.000086
256     - 0.000251
512     - 0.000698
1024     - 0.001818
2048     - 0.004124
4096     - 0.005286
8192     - 0.010566
16384     - 0.022061
32768     - 0.045513
65536     - 0.094961
131072     - 0.201415
262144     - 0.431597
524288     - 0.666660

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

i64

multithreading on

2     - 0.000001
4     - 0.000003
8     - 0.000008
16     - 0.000016
32     - 0.000033
64     - 0.000069
128     - 0.000154
256     - 0.000380
512     - 0.001008
1024     - 0.002594
2048     - 0.005705
4096     - 0.005941
8192     - 0.010722
16384     - 0.023228
32768     - 0.042546
65536     - 0.091817
131072     - 0.192015
262144     - 0.449105
524288     - 0.922172

multithreading off

2     - 0.000001
4     - 0.000003
8     - 0.000007
16     - 0.000016
32     - 0.000034
64     - 0.000071
128     - 0.000159
256     - 0.000376
512     - 0.001018
1024     - 0.002658
2048     - 0.005736
4096     - 0.009687
8192     - 0.017713
16384     - 0.037901
32768     - 0.075060
65536     - 0.165686
131072     - 0.336338
262144     - 0.780771
524288     - 1.536626

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
