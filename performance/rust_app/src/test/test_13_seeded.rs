use super::data::{rect, Fixture, Prng};
use i_overlay::core::overlay_rule::OverlayRule;
pub(crate) struct SeededTest;
impl SeededTest {
    pub(crate) fn geometry(size: usize, seed: u64) -> Fixture {
        let mut f = Fixture {
            subj: vec![],
            clip: vec![],
            rule: OverlayRule::Union,
        };
        let mut rng = Prng(seed);
        // Exactly four draws per rectangle, alternating subject and clip.
        // Modulo mapping is intentional (no distribution/version dependency).
        for i in 0..2 * size {
            let x = (rng.next() % 4096) as i64 - 2048;
            let y = (rng.next() % 4096) as i64 - 2048;
            let w = 8 + (rng.next() % 256) as i64;
            let h = 8 + (rng.next() % 256) as i64;
            if i % 2 == 0 {
                f.subj.push(rect(x, y, w, h));
            } else {
                f.clip.push(rect(x, y, w, h));
            }
        }
        f.rule = OverlayRule::Xor;
        f
    }
}
