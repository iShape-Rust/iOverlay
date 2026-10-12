use super::data::Fixture;
use i_overlay::core::overlay_rule::OverlayRule;
pub(crate) struct DiagonalTest;
impl DiagonalTest {
    pub(crate) fn geometry(size: usize) -> Fixture {
        let n = size as i64;
        let mut f = Fixture {
            subj: vec![],
            clip: vec![],
            rule: OverlayRule::Union,
        };
        let l = 8 * n + 64;
        for i in 0..n {
            let y = 16 * i;
            let dy = l / 2 + i % 3;
            f.subj
                .push(vec![(-l, y), (l, y + dy), (l, y + dy + 4), (-l, y + 4)]);
            f.clip.push(vec![
                (-l, y + 2),
                (l, y + dy + 2),
                (l, y + dy + 6),
                (-l, y + 6),
            ]);
        }
        f.rule = OverlayRule::Intersect;
        f
    }
}
