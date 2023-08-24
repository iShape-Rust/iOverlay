#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FillRule {
    Subject,
    Clip,
    Intersect,
    Union,
    Difference,
    Xor
}