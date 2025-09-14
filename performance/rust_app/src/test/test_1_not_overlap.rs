use std::time::Instant;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::Overlay;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::solver::Solver;
use i_overlay::i_float::int::point::IntPoint;
use crate::test::util::Util;

pub(crate) struct NotOverlapTest;
/*

// 1
// Union:

multithreading on

5     - 0.000003
25     - 0.000016
113     - 0.000066
481     - 0.000371
1985     - 0.001692
8065     - 0.005704
32513     - 0.026937
130561     - 0.106385
523265     - 0.549426
2095105     - 2.380725
8384513     - 10.040016

multithreading off

5     - 0.000003
25     - 0.000013
113     - 0.000064
481     - 0.000368
1985     - 0.001671
8065     - 0.006311
32513     - 0.030795
130561     - 0.133008
523265     - 0.680722
2095105     - 3.050487
8384513     - 12.764988

*/

// A grid of not overlapping squares.
impl NotOverlapTest {
    pub(crate) fn run(n: usize, rule: OverlayRule, solver: Solver, scale: f64, simple_geometry: bool) { // 1000
        let subj_paths = Util::many_squares(IntPoint::new(0, 0), 10, 30, n);
        let clip_paths = Util::many_squares(IntPoint::new(15, 15), 10, 30, n - 1);

        let it_count = ((scale / (n as f64)) as usize).max(1);
        let sq_it_count= it_count * it_count;

        let start = Instant::now();

        if simple_geometry {
            // for _i in 0..sq_it_count {
            //     let _ = Overlay::with_contours(&subj_paths, &clip_paths)
            //         .overlay_45geom_with_min_area_and_solver(rule, FillRule::NonZero, 0, solver);
            // }
        } else {
            for _i in 0..sq_it_count {
                let _ = Overlay::with_contours_custom(&subj_paths, &clip_paths, Default::default(), solver)
                    .overlay(rule, FillRule::NonZero);
            }
        }

        let duration = start.elapsed();
        let time = duration.as_secs_f64() / sq_it_count as f64;

        let polygons_count = n * n + (n - 1) * (n - 1);

        println!("{:.1}     - {:.6}", polygons_count, time);
    }
}