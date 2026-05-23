use crate::test::util::OverlayInt;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::solver::Solver;
use i_overlay::float::overlay::FloatOverlay;
use std::f64::consts::PI;
use std::time::Instant;

pub(crate) struct CorrosionTest;

/*
test 6
CorrosionTest
Difference:

i16

// multithreading on
1     - 0.000006
2     - 0.000018
4     - 0.000082
8     - 0.000578
16     - 0.003789
32     - 0.007703
64     - 0.030151
128     - 0.134113
256     - 0.553680
512     - 2.020489

// multithreading off
1     - 0.000006
2     - 0.000018
4     - 0.000082
8     - 0.000583
16     - 0.003799
32     - 0.009568
64     - 0.041341
128     - 0.209341
256     - 0.878897
512     - 2.869231

i32

// multithreading on
1     - 0.000007
2     - 0.000024
4     - 0.000103
8     - 0.000645
16     - 0.004095
32     - 0.008387
64     - 0.033293
128     - 0.133794
256     - 0.594231
512     - 2.297538

// multithreading off
1     - 0.000007
2     - 0.000023
4     - 0.000104
8     - 0.000636
16     - 0.004134
32     - 0.011785
64     - 0.050564
128     - 0.199536
256     - 0.812732
512     - 3.383901

i64

// multithreading on
1     - 0.000009
2     - 0.000036
4     - 0.000158
8     - 0.000914
16     - 0.005314
32     - 0.010206
64     - 0.042668
128     - 0.175621
256     - 0.737713
512     - 3.101692

// multithreading off
1     - 0.000009
2     - 0.000036
4     - 0.000159
8     - 0.000918
16     - 0.005234
32     - 0.016559
64     - 0.072355
128     - 0.304704
256     - 1.280368
512     - 5.351266
*/

// A series of concentric squares, each progressively larger than the last.
impl CorrosionTest {
    pub(crate) fn run<I: OverlayInt>(n: usize, rule: OverlayRule, solver: Solver, scale: f64) {
        // 500
        let (subj_paths, clip_paths) = Self::geometry(100.0, n);

        let it_count = ((scale / (n as f64)) as usize).max(1);
        let sq_it_count = it_count * it_count;

        let start = Instant::now();

        for _ in 0..sq_it_count {
            let mut overlay = FloatOverlay::<[f64; 2], I>::from_subj_and_clip_custom(
                &subj_paths,
                &clip_paths,
                Default::default(),
                solver,
            );
            let _res = overlay.overlay(rule, FillRule::NonZero);
        }
        let duration = start.elapsed();
        let time = duration.as_secs_f64() / sq_it_count as f64;

        println!("{}     - {:.6}", n, time);
    }

    fn geometry(size: f64, count: usize) -> (Vec<Vec<[f64; 2]>>, Vec<Vec<[f64; 2]>>) {
        let subj_radius = 0.4 * size;
        let subj_step = size;

        let clip_radius = 0.4 * subj_radius;
        let clip_step = 0.4 * subj_step;
        let clip_count = ((count as f64) * 2.5).round() as usize;

        let subj = Self::shapes(0.0, subj_step, subj_radius, count);
        let clip = Self::shapes(subj_radius, clip_step, clip_radius, clip_count);

        (subj, clip)
    }

    fn shapes(offset: f64, step: f64, radius: f64, count: usize) -> Vec<Vec<[f64; 2]>> {
        let mut y = -offset;

        let mut paths = Vec::with_capacity(count);
        for i in 0..count {
            let mut index = i;
            let mut x = -offset;
            for _ in 0..count {
                let count = (index % 5) + 3;
                paths.push(Self::shape([x, y], radius, count));
                x += step;
                index += 1;
            }
            y += step;
        }

        paths
    }

    fn shape(center: [f64; 2], radius: f64, count: usize) -> Vec<[f64; 2]> {
        let da: f64 = 2.0 * PI / (count as f64);
        let mut points = Vec::with_capacity(count);

        let mut a = 0.0f64;

        for _ in 0..count {
            let x = a.cos() * radius + center[0];
            let y = a.sin() * radius + center[1];
            points.push([x, y]);
            a += da;
        }

        points
    }
}
