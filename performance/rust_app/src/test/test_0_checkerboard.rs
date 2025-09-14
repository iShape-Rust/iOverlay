use std::time::Instant;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::Overlay;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::solver::Solver;
use i_overlay::i_float::int::point::IntPoint;
use crate::test::util::Util;

pub(crate) struct CheckerboardTest;

/*
// test 0
// Xor:

multithreading on

2(5 0.7)     - 0.000006(-5.2)
4(25 1.4)     - 0.000040(-4.4)
8(113 2.1)     - 0.000199(-3.7)
16(481 2.7)     - 0.001071(-3.0)
32(1985 3.3)     - 0.005099(-2.3)
64(8065 3.9)     - 0.019657(-1.7)
128(32513 4.5)     - 0.077459(-1.1)
256(130561 5.1)     - 0.350502(-0.5)
512(523265 5.7)     - 1.699005(0.2)
1024(2095105 6.3)     - 7.520601(0.9)
2048(8384513 6.9)     - 30.258030(1.5)


multithreading off

2(5 0.7)     - 0.000006(-5.2)
4(25 1.4)     - 0.000035(-4.5)
8(113 2.1)     - 0.000194(-3.7)
16(481 2.7)     - 0.001069(-3.0)
32(1985 3.3)     - 0.005088(-2.3)
64(8065 3.9)     - 0.021487(-1.7)
128(32513 4.5)     - 0.095950(-1.0)
256(130561 5.1)     - 0.441111(-0.4)
512(523265 5.7)     - 2.146555(0.3)
1024(2095105 6.3)     - 9.435037(1.0)
2048(8384513 6.9)     - 38.408757(1.6)
 */

// A grid of overlapping squares forming a simple checkerboard pattern.
impl CheckerboardTest {
    pub(crate) fn run(n: usize, rule: OverlayRule, solver: Solver, scale: f64, simple_geometry: bool) { // 1000
        let subj_paths = Util::many_squares(IntPoint::new(0, 0), 20, 30, n);
        let clip_paths = Util::many_squares(IntPoint::new(15, 15), 20, 30, n - 1);

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
        let count_log = (polygons_count as f64).log10();
        let time_log = time.log10();

        println!("{}({} {:.1})     - {:.6}({:.1})", n, polygons_count, count_log, time, time_log);
    }
}