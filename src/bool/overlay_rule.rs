#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OverlayRule {
    Subject,
    Clip,
    Intersect,
    Union,
    Difference,
    Xor
}