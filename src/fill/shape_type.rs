#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ShapeType(pub(crate) u8);

impl ShapeType {
    pub const NONE: Self = Self(0b00);
    pub const SUBJECT: Self = Self(0b01);
    pub const CLIP: Self = Self(0b10);
    pub const COMMON: Self = Self(Self::SUBJECT.0 | Self::CLIP.0);
}

impl std::ops::BitOr for ShapeType {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for ShapeType {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}