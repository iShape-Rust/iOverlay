use std::time::Instant;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::Overlay;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::solver::Solver;
use i_overlay::i_float::int::point::IntPoint;
use crate::test::util::Util;

pub(crate) struct WindowsTest;
/*
// 4
// Difference:

// multithreading on
8     - 0.000006
32     - 0.000024
128     - 0.000102
512     - 0.000554
2048     - 0.002276
8192     - 0.007199
32768     - 0.036992
131072     - 0.159541
524288     - 0.701553
2097152     - 2.941974
8388608     - 12.287573

// multithreading off
8     - 0.000006
32     - 0.000022
128     - 0.000099
512     - 0.000546
2048     - 0.002241
8192     - 0.008248
32768     - 0.042241
131072     - 0.202890
524288     - 0.880672
2097152     - 3.708358
8388608     - 15.611615

*/

// A grid of square frames, each with a smaller square cutout in the center.
impl WindowsTest {
    pub(crate) fn run(n: usize, rule: OverlayRule, solver: Solver, scale: f64, simple_geometry: bool) { // 500
        let offset = 30;
        let x = (n as i32) * offset / 2;
        let origin = IntPoint::new(-x, -x);
        let (subj_paths, clip_paths) = Util::many_windows(origin, 20, 10, offset, n);

        let it_count = ((scale / (n as f64)) as usize).max(1);
        let sq_it_count= it_count * it_count;

        let start = Instant::now();

        if simple_geometry {
            // for _ in 0..sq_it_count {
            //     let _ = Overlay::with_contours(&subj_paths, &clip_paths)
            //         .overlay_45geom_with_min_area_and_solver(rule, FillRule::NonZero, 0, solver);
            // }
        } else {
            for _ in 0..sq_it_count {
                let _ = Overlay::with_contours_custom(&subj_paths, &clip_paths, Default::default(), solver)
                    .overlay(rule, FillRule::NonZero);
            }
        }

        let duration = start.elapsed();
        let time = duration.as_secs_f64() / sq_it_count as f64;

        let polygons_count = 2 * n * n;

        println!("{}     - {:.6}", polygons_count, time);
    }
}