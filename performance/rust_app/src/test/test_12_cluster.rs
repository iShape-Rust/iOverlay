use super::data::{rect, Fixture, Prng};
use i_overlay::core::overlay_rule::OverlayRule;
pub(crate) struct ClusterTest;
impl ClusterTest {
    pub(crate) fn geometry(size: usize, seed: u64, outlier: bool) -> Fixture {
        let n = size as i64;
        let mut f = Fixture {
            subj: vec![],
            clip: vec![],
            rule: OverlayRule::Union,
        };
        let mut rng = Prng(seed);
        for _ in 0..n {
            let x = (rng.next() % 512) as i64 - 256;
            let y = (rng.next() % 512) as i64 - 256;
            f.subj.push(rect(x, y, 64, 64));
            f.clip.push(rect(x + 16, y + 16, 64, 64));
        }
        // Both paired cases have exactly the same edge count.
        let x = if outlier { 12000 } else { 0 };
        f.subj.push(rect(x, x, 8, 8));
        f
    }
}
