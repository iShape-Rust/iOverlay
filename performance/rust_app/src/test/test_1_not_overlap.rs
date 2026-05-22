use crate::test::util::{OverlayInt, Util};
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::Overlay;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::solver::Solver;
use i_overlay::i_float::int::point::IntPoint;
use std::time::Instant;

pub(crate) struct NotOverlapTest;
/*

// 1
// Union:

i16

multithreading on

5     - 0.000001
25     - 0.000005
113     - 0.000024
481     - 0.000138
1985     - 0.000908
8065     - 0.002176
32513     - 0.008377
130561     - 0.034438
523265     - 0.158055
2095105     - 0.687777

multithreading off

5     - 0.000001
25     - 0.000005
113     - 0.000024
481     - 0.000136
1985     - 0.000901
8065     - 0.002461
32513     - 0.011272
130561     - 0.048214
523265     - 0.220364
2095105     - 0.927191

i32

multithreading on

5     - 0.000001
25     - 0.000005
113     - 0.000024
481     - 0.000132
1985     - 0.000908
8065     - 0.002322
32513     - 0.008956
130561     - 0.036901
523265     - 0.177118
2095105     - 0.745187
8384513     - 3.282416

multithreading off

5     - 0.000001
25     - 0.000005
113     - 0.000024
481     - 0.000133
1985     - 0.000912
8065     - 0.002725
32513     - 0.012319
130561     - 0.053420
523265     - 0.240664
2095105     - 1.037521
8384513     - 4.580672

i64

multithreading on

5     - 0.000001
25     - 0.000006
113     - 0.000028
481     - 0.000151
1985     - 0.001053
8065     - 0.002579
32513     - 0.010409
130561     - 0.044266
523265     - 0.217189
2095105     - 0.920391
8384513     - 3.948376

multithreading off

5     - 0.000001
25     - 0.000006
113     - 0.000027
481     - 0.000152
1985     - 0.001046
8065     - 0.003178
32513     - 0.014719
130561     - 0.065100
523265     - 0.301577
2095105     - 1.300260
8384513     - 5.667204

*/

// A grid of not overlapping squares.
impl NotOverlapTest {
    pub(crate) fn run<I: OverlayInt>(n: usize, rule: OverlayRule, solver: Solver, scale: f64) {
        // 1000
        if Util::skip_if_out_of_range::<I>(n, 30 * n + 10) {
            return;
        }

        let subj_paths = Util::many_squares(
            IntPoint::new(I::ZERO, I::ZERO),
            I::from_usize(10),
            I::from_usize(30),
            n,
        );
        let clip_paths = Util::many_squares(
            IntPoint::new(I::from_usize(15), I::from_usize(15)),
            I::from_usize(10),
            I::from_usize(30),
            n - 1,
        );

        let it_count = ((scale / (n as f64)) as usize).max(1);
        let sq_it_count = it_count * it_count;

        let start = Instant::now();

        for _i in 0..sq_it_count {
            let _ =
                Overlay::with_contours_custom(&subj_paths, &clip_paths, Default::default(), solver)
                    .overlay(rule, FillRule::NonZero);
        }

        let duration = start.elapsed();
        let time = duration.as_secs_f64() / sq_it_count as f64;

        let polygons_count = n * n + (n - 1) * (n - 1);

        println!("{:.1}     - {:.6}", polygons_count, time);
    }
}
