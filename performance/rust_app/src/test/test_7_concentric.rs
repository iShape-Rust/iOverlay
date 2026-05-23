use crate::test::util::OverlayInt;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::solver::Solver;
use i_overlay::float::overlay::FloatOverlay;
use std::f64::consts::PI;
use std::time::Instant;

pub(crate) struct ConcentricTest;

/*
test 7
ConcentricTest
Difference:

i16

// multithreading on
1     - 0.000007
2     - 0.000020
4     - 0.000082
8     - 0.000610
16     - 0.005260
32     - 0.013028
64     - 0.053713
128     - 0.318682
256     - 1.384741
512     - 6.981723

// multithreading off
1     - 0.000007
2     - 0.000020
4     - 0.000083
8     - 0.000569
16     - 0.005350
32     - 0.009203
64     - 0.035059
128     - 0.170760
256     - 0.712446
512     - 3.317967

i32

// multithreading on
1     - 0.000010
2     - 0.000028
4     - 0.000113
8     - 0.000660
16     - 0.004171
32     - 0.008281
64     - 0.034357
128     - 0.134210
256     - 0.563524
512     - 2.367582

// multithreading off
1     - 0.000010
2     - 0.000029
4     - 0.000112
8     - 0.000709
16     - 0.004211
32     - 0.012234
64     - 0.052943
128     - 0.223251
256     - 0.955805
512     - 3.692530

i64

// multithreading on
1     - 0.000016
2     - 0.000049
4     - 0.000189
8     - 0.000955
16     - 0.005389
32     - 0.010158
64     - 0.042141
128     - 0.171089
256     - 0.726401
512     - 3.116469

// multithreading off
1     - 0.000016
2     - 0.000049
4     - 0.000186
8     - 0.001010
16     - 0.005452
32     - 0.017694
64     - 0.075916
128     - 0.325436
256     - 1.386162
512     - 5.722743


*/

// A series of concentric squares, each progressively larger than the last.
impl ConcentricTest {
    pub(crate) fn run<I: OverlayInt>(n: usize, rule: OverlayRule, solver: Solver, scale: f64) {
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
        let mut clip = Vec::with_capacity(count);
        let mut subj = Vec::with_capacity(count);

        let mut r = size;
        let scale = 0.8 / size;
        let mut angle = 0.0;
        let rr = 0.5 * size;
        for i in 0..count {
            let body = Self::shape([0.0, 0.0], angle, r, i + 3, -1.0);
            let hole = Self::shape([0.0, 0.0], angle, r + size, i + 3, 1.0);
            subj.push(body);
            subj.push(hole);

            let l = 2.0 * PI * r;
            let n = l * scale;
            let clip_count = n as usize;
            let da = 2.0 * PI / n;
            let mut a = angle;
            for j in 0..clip_count {
                let x = a.cos() * r;
                let y = a.sin() * r;

                let shape = Self::shape([x, y], 0.0, rr, j % 5 + 3, 1.0);
                clip.push(shape);
                a += da;
            }

            r += 2.0 * size;
            angle += 0.05;
        }

        (subj, clip)
    }

    fn shape(center: [f64; 2], angle: f64, radius: f64, count: usize, dir: f64) -> Vec<[f64; 2]> {
        let da: f64 = dir * 2.0 * PI / (count as f64);
        let mut points = Vec::with_capacity(count);

        let mut a = angle;

        for _ in 0..count {
            let x = a.cos() * radius + center[0];
            let y = a.sin() * radius + center[1];
            points.push([x, y]);
            a += da;
        }

        points
    }
}
