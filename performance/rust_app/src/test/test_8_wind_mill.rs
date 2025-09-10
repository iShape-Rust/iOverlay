use std::time::Instant;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::Overlay;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::solver::Solver;
use i_overlay::i_float::int::point::IntPoint;
use i_overlay::i_shape::int::shape::IntContour;

pub(crate) struct WindMillTest;

/*
// 7
// Difference:

// multithreading on
1     - 0.000007
2     - 0.000028
4     - 0.000136
8     - 0.000702
16     - 0.003199
32     - 0.010574
64     - 0.049129
128     - 0.202190
256     - 0.887956
512     - 3.830878
1024     - 15.643612

// multithreading off
1     - 0.000007
2     - 0.000027
4     - 0.000136
8     - 0.000708
16     - 0.003072
32     - 0.010975
64     - 0.054829
128     - 0.243694
256     - 1.180819
512     - 4.130558
1024     - 17.840079
 */

impl WindMillTest {
    pub(crate) fn run(n: usize, rule: OverlayRule, solver: Solver, scale: f64, simple_geometry: bool) {
        let (subj_paths, clip_paths) = Self::geometry(80, n);

        let it_count = ((scale / (n as f64)) as usize).max(1);
        let sq_it_count = it_count * it_count;

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

        println!("{}     - {:.6}", n, time);
    }

    pub(crate) fn validate(n: usize, rule: OverlayRule, solver: Solver) {
        let (subj_paths, clip_paths) = Self::geometry(80, n);

        let res = Overlay::with_contours_custom(&subj_paths, &clip_paths, Default::default(), solver)
            .overlay(rule, FillRule::NonZero);

        assert_eq!(res.len(), n * n);
        println!("result validation PASS");
    }

    fn geometry(size: i32, count: usize) -> (Vec<IntContour>, Vec<IntContour>) {
        let mut subj_paths = Vec::with_capacity(4 * count * count);
        let mut clip_paths = Vec::with_capacity(4 * count * count);

        let a = size / 8;

        let mut x = size / 2;
        for _ in 0..count {
            let mut y = size / 2;
            for _ in 0..count {
                let (subj, clip) = Self::shapes(IntPoint::new(x, y), a);

                subj_paths.extend(subj);
                clip_paths.extend(clip);

                y += size
            }

            x += size
        }

        (subj_paths, clip_paths)
    }

    fn shapes(center: IntPoint, a: i32) -> (Vec<IntContour>, Vec<IntContour>) {
        let clip_paths = vec![
            vec![
                IntPoint::new(-3 * a + center.x, 1 * a + center.y),
                IntPoint::new(-3 * a + center.x, 3 * a + center.y),
                IntPoint::new(-1 * a + center.x, 3 * a + center.y),
                IntPoint::new(-1 * a + center.x, 1 * a + center.y),
            ],
            vec![
                IntPoint::new(1 * a + center.x, 2 * a + center.y),
                IntPoint::new(1 * a + center.x, 4 * a + center.y),
                IntPoint::new(3 * a + center.x, 4 * a + center.y),
                IntPoint::new(3 * a + center.x, 2 * a + center.y),
            ],
            vec![
                IntPoint::new(-2 * a + center.x, -3 * a + center.y),
                IntPoint::new(-2 * a + center.x, -1 * a + center.y),
                IntPoint::new(0 * a + center.x, -1 * a + center.y),
                IntPoint::new(0 * a + center.x, -3 * a + center.y),
            ],
            vec![
                IntPoint::new(2 * a + center.x, -2 * a + center.y),
                IntPoint::new(2 * a + center.x, 0 * a + center.y),
                IntPoint::new(4 * a + center.x, 0 * a + center.y),
                IntPoint::new(4 * a + center.x, -2 * a + center.y),
            ],
        ];

        let subj_paths = vec![
            vec![
                IntPoint::new(0 * a + center.x, 0 * a + center.y),
                IntPoint::new(-3 * a + center.x, 0 * a + center.y),
                IntPoint::new(0 * a + center.x, 3 * a + center.y),
            ],
            vec![
                IntPoint::new(0 * a + center.x, 1 * a + center.y),
                IntPoint::new(0 * a + center.x, 4 * a + center.y),
                IntPoint::new(3 * a + center.x, 1 * a + center.y),
            ],
            vec![
                IntPoint::new(1 * a + center.x, 0 * a + center.y),
                IntPoint::new(1 * a + center.x, -3 * a + center.y),
                IntPoint::new(-2 * a + center.x, 0 * a + center.y),
            ],
            vec![
                IntPoint::new(1 * a + center.x, 1 * a + center.y),
                IntPoint::new(4 * a + center.x, 1 * a + center.y),
                IntPoint::new(1 * a + center.x, -2 * a + center.y),
            ],
        ];

        (subj_paths, clip_paths)
    }
}