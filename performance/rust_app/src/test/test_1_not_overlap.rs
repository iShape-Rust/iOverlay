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
25     - 0.000012
113     - 0.000062
481     - 0.000344
1985     - 0.001668
8065     - 0.005425
32513     - 0.024718
130561     - 0.107485
523265     - 0.538060
2095105     - 2.470210
8384513     - 9.601191

multithreading off

5     - 0.000003
25     - 0.000013
113     - 0.000065
481     - 0.000356
1985     - 0.001646
8065     - 0.005963
32513     - 0.028155
130561     - 0.125444
523265     - 0.640918
2095105     - 2.696089
8384513     - 12.902138

geom multithreading off

5     - 0.000004
25     - 0.000014
113     - 0.000059
481     - 0.000267
1985     - 0.001084
8065     - 0.005110
32513     - 0.023544
130561     - 0.102948
523265     - 0.506193
2095105     - 2.137119
8384513     - 9.766767

geom swipe line hash

5     - 0.000003
25     - 0.000011
113     - 0.000048
481     - 0.000214
1985     - 0.000965
8065     - 0.004601
32513     - 0.021743
130561     - 0.089794
523265     - 0.452441
2095105     - 1.907293
8384513     - 8.501941

geom map

5     - 0.000003
25     - 0.000012
113     - 0.000049
481     - 0.000215
1985     - 0.000970
8065     - 0.004646
32513     - 0.021622
130561     - 0.092652
523265     - 0.462316
2095105     - 1.946025
8384513     - 8.754714


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