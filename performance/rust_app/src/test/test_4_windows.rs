use super::data::Fixture;
use super::util::Util;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::i_float::int::point::IntPoint;

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

impl WindowsTest {
    pub(crate) fn geometry(n: usize) -> Fixture {
        let x = n as i64 * 30 / 2;
        let (subj, clip) = Util::many_windows(IntPoint::new(-x, -x), 20, 10, 30, n);
        Fixture::from_int(subj, clip, OverlayRule::Difference)
    }
}
