use super::data::Fixture;
use super::util::Util;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::i_float::int::point::IntPoint;

pub(crate) struct NotOverlapTest;
/*
test 1
NotOverlapTest
Union:

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

impl NotOverlapTest {
    pub(crate) fn geometry(n: usize, line: bool) -> Fixture {
        if line {
            let mut f = Fixture {
                subj: vec![],
                clip: vec![],
                rule: OverlayRule::Union,
            };
            for i in 0..n as i64 {
                f.subj.push(super::data::rect(32 * i, 0, 8, 8));
                f.clip.push(super::data::rect(32 * i + 16, 12, 8, 8));
            }
            return f;
        }
        let subj = Util::many_squares(IntPoint::new(0i64, 0), 10, 30, n);
        let clip = Util::many_squares(IntPoint::new(15i64, 15), 10, 30, n - 1);
        Fixture::from_int(subj, clip, OverlayRule::Union)
    }
}
