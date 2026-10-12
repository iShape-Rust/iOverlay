use super::data::{rect, Fixture};
use i_overlay::core::overlay_rule::OverlayRule;
pub(crate) struct CollinearTest;
impl CollinearTest {
    pub(crate) fn geometry(size: usize) -> Fixture {
        let n = size as i64;
        let mut f = Fixture {
            subj: vec![],
            clip: vec![],
            rule: OverlayRule::Union,
        };
        for i in 0..n {
            let x = 32 * i;
            f.subj.push(rect(x, 0, 24, 12));
            f.clip.push(rect(x + 8, 0, 24, 12)); // Partial collinear overlap.
            f.clip.push(rect(x, 12, 24, 12)); // Shared boundary.
        }
        f.rule = OverlayRule::Xor;
        f
    }
}
