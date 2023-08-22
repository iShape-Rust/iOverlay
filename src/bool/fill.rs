#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FillRule {
    Subject,
    Clip,
    Intersect,
    Union,
    Difference,
    Xor,
}

pub struct ShapeType(pub(crate) u8);

impl ShapeType {
    pub const SUBJECT: Self = Self(0b0001);
    pub const CLIP: Self = Self(0b0010);
    pub const COMMON: Self = Self(Self::SUBJECT.0 | Self::CLIP.0);
}

pub struct SegmentFill(pub(crate) u8);

impl SegmentFill {
    pub const SUBJECT_TOP: Self = Self(0b0001);
    pub const SUBJECT_BOTTOM: Self = Self(0b0010);
    pub const CLIP_TOP: Self = Self(0b0100);
    pub const CLIP_BOTTOM: Self = Self(0b1000);
    pub const SUBJECT_BOTH: Self = Self(Self::SUBJECT_TOP.0 | Self::SUBJECT_BOTTOM.0);
    pub const CLIP_BOTH: Self = Self(Self::CLIP_TOP.0 | Self::CLIP_BOTTOM.0);
    pub const BOTH_TOP: Self = Self(Self::SUBJECT_TOP.0 | Self::CLIP_TOP.0);
    pub const BOTH_BOTTOM: Self = Self(Self::SUBJECT_BOTTOM.0 | Self::CLIP_BOTTOM.0);
}
