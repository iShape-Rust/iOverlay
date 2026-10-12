use super::data::{rect, rotate, Fixture};
use i_overlay::core::overlay_rule::OverlayRule;
pub(crate) struct OrientationTest;
impl OrientationTest {
    pub(crate) fn geometry(size: usize, vertical: bool) -> Fixture {
        let n = size as i64;
        let mut f = Fixture {
            subj: vec![],
            clip: vec![],
            rule: OverlayRule::Union,
        };
        for i in 0..n {
            f.subj.push(rect(-8 * n, 16 * i, 16 * n, 6));
            f.clip.push(rect(-8 * n + 4, 16 * i + 3, 16 * n, 6));
        }
        f.rule = OverlayRule::Xor;
        if vertical {
            rotate(&mut f.subj);
            rotate(&mut f.clip);
        }
        f
    }
}
