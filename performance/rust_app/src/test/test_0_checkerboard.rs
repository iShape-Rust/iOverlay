use crate::test::util::{OverlayInt, Util};
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::Overlay;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::solver::Solver;
use i_overlay::i_float::int::point::IntPoint;
use std::time::Instant;

pub(crate) struct CheckerboardTest;

/*
test 0
CheckerboardTest
Xor:

i16

multithreading on

2(5 0.7)     - 0.000003(-5.6)
4(25 1.4)     - 0.000015(-4.8)
8(113 2.1)     - 0.000083(-4.1)
16(481 2.7)     - 0.000437(-3.4)
32(1985 3.3)     - 0.002305(-2.6)
64(8065 3.9)     - 0.007140(-2.1)
128(32513 4.5)     - 0.027857(-1.6)
256(130561 5.1)     - 0.120606(-0.9)
512(523265 5.7)     - 0.568957(-0.2)
1024(2095105 6.3)     - 2.413621(0.4)

multithreading off

2(5 0.7)     - 0.000002(-5.6)
4(25 1.4)     - 0.000015(-4.8)
8(113 2.1)     - 0.000082(-4.1)
16(481 2.7)     - 0.000426(-3.4)
32(1985 3.3)     - 0.002236(-2.7)
64(8065 3.9)     - 0.008070(-2.1)
128(32513 4.5)     - 0.037559(-1.4)
256(130561 5.1)     - 0.167375(-0.8)
512(523265 5.7)     - 0.776731(-0.1)
1024(2095105 6.3)     - 3.385874(0.5)

i32

multithreading on

2(5 0.7)     - 0.000002(-5.6)
4(25 1.4)     - 0.000015(-4.8)
8(113 2.1)     - 0.000081(-4.1)
16(481 2.7)     - 0.000409(-3.4)
32(1985 3.3)     - 0.002299(-2.6)
64(8065 3.9)     - 0.007243(-2.1)
128(32513 4.5)     - 0.028986(-1.5)
256(130561 5.1)     - 0.125711(-0.9)
512(523265 5.7)     - 0.592768(-0.2)
1024(2095105 6.3)     - 2.610431(0.4)

multithreading off

2(5 0.7)     - 0.000002(-5.6)
4(25 1.4)     - 0.000015(-4.8)
8(113 2.1)     - 0.000082(-4.1)
16(481 2.7)     - 0.000412(-3.4)
32(1985 3.3)     - 0.002318(-2.6)
64(8065 3.9)     - 0.008659(-2.1)
128(32513 4.5)     - 0.040512(-1.4)
256(130561 5.1)     - 0.178425(-0.7)
512(523265 5.7)     - 0.827264(-0.1)
1024(2095105 6.3)     - 3.611125(0.6)

i64

multithreading on

2(5 0.7)     - 0.000003(-5.6)
4(25 1.4)     - 0.000017(-4.8)
8(113 2.1)     - 0.000099(-4.0)
16(481 2.7)     - 0.000501(-3.3)
32(1985 3.3)     - 0.002752(-2.6)
64(8065 3.9)     - 0.008616(-2.1)
128(32513 4.5)     - 0.035981(-1.4)
256(130561 5.1)     - 0.158665(-0.8)
512(523265 5.7)     - 0.724087(-0.1)
1024(2095105 6.3)     - 3.127404(0.5)

multithreading off

2(5 0.7)     - 0.000003(-5.6)
4(25 1.4)     - 0.000017(-4.8)
8(113 2.1)     - 0.000099(-4.0)
16(481 2.7)     - 0.000498(-3.3)
32(1985 3.3)     - 0.002733(-2.6)
64(8065 3.9)     - 0.010388(-2.0)
128(32513 4.5)     - 0.050085(-1.3)
256(130561 5.1)     - 0.219622(-0.7)
512(523265 5.7)     - 1.011703(0.0)
1024(2095105 6.3)     - 4.393880(0.6)
 */

// A grid of overlapping squares forming a simple checkerboard pattern.
impl CheckerboardTest {
    pub(crate) fn run<I: OverlayInt>(n: usize, rule: OverlayRule, solver: Solver, scale: f64) {
        // 1000
        if Util::skip_if_out_of_range::<I>(n, 15 * n + 10) {
            return;
        }

        let subj_paths = Util::many_squares(
            IntPoint::new(I::ZERO, I::ZERO),
            I::from_usize(20),
            I::from_usize(30),
            n,
        );
        let clip_paths = Util::many_squares(
            IntPoint::new(I::from_usize(15), I::from_usize(15)),
            I::from_usize(20),
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
        let count_log = (polygons_count as f64).log10();
        let time_log = time.log10();

        println!(
            "{}({} {:.1})     - {:.6}({:.1})",
            n, polygons_count, count_log, time, time_log
        );
    }
}
