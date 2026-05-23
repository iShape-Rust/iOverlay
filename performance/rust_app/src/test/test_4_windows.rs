use crate::test::util::{OverlayInt, Util};
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::Overlay;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::solver::Solver;
use i_overlay::i_float::int::point::IntPoint;
use std::time::Instant;

pub(crate) struct WindowsTest;
/*
test 4
WindowsTest
Difference:

i16

// multithreading on
8     - 0.000003
32     - 0.000009
128     - 0.000038
512     - 0.000204
2048     - 0.001129
8192     - 0.002633
32768     - 0.010163
131072     - 0.045737
524288     - 0.199035
2097152     - 0.909131

// multithreading off
8     - 0.000003
32     - 0.000009
128     - 0.000038
512     - 0.000205
2048     - 0.001127
8192     - 0.003188
32768     - 0.013917
131072     - 0.063098
524288     - 0.277322
2097152     - 1.211561

i32

// multithreading on
8     - 0.000002
32     - 0.000009
128     - 0.000038
512     - 0.000194
2048     - 0.001138
8192     - 0.002809
32768     - 0.010892
131072     - 0.049875
524288     - 0.224817
2097152     - 1.007965

// multithreading off
8     - 0.000003
32     - 0.000008
128     - 0.000038
512     - 0.000195
2048     - 0.001133
8192     - 0.003421
32768     - 0.015584
131072     - 0.069925
524288     - 0.312258
2097152     - 1.382459

i64

// multithreading on
8     - 0.000003
32     - 0.000010
128     - 0.000043
512     - 0.000224
2048     - 0.001340
8192     - 0.003138
32768     - 0.012617
131072     - 0.061721
524288     - 0.267889
2097152     - 1.175990

// multithreading off
8     - 0.000003
32     - 0.000010
128     - 0.000043
512     - 0.000225
2048     - 0.001338
8192     - 0.004013
32768     - 0.018626
131072     - 0.088967
524288     - 0.384434
2097152     - 1.709374

*/

// A grid of square frames, each with a smaller square cutout in the center.
impl WindowsTest {
    pub(crate) fn run<I: OverlayInt>(n: usize, rule: OverlayRule, solver: Solver, scale: f64) {
        // 500
        if Util::skip_if_out_of_range::<I>(n, 15 * n + 20) {
            return;
        }

        let offset = I::from_usize(30);
        let x = I::from_usize(n) * offset / I::TWO;
        let origin = IntPoint::new(-x, -x);
        let (subj_paths, clip_paths) =
            Util::many_windows(origin, I::from_usize(20), I::from_usize(10), offset, n);

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

        let polygons_count = 2 * n * n;

        println!("{}     - {:.6}", polygons_count, time);
    }
}
