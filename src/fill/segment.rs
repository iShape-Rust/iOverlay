use i_float::fix_vec::FixVec;
use crate::split::shape_count::ShapeCount;
use crate::split::shape_edge::ShapeEdge;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SegmentFill(pub(crate) u8);

impl SegmentFill {
    pub const NONE: Self = Self(0b0000);
    pub const SUBJECT_TOP: Self = Self(0b0001);
    pub const SUBJECT_BOTTOM: Self = Self(0b0010);
    pub const CLIP_TOP: Self = Self(0b0100);
    pub const CLIP_BOTTOM: Self = Self(0b1000);
    pub const SUBJECT_BOTH: Self = Self(Self::SUBJECT_TOP.0 | Self::SUBJECT_BOTTOM.0);
    pub const CLIP_BOTH: Self = Self(Self::CLIP_TOP.0 | Self::CLIP_BOTTOM.0);
    pub const BOTH_TOP: Self = Self(Self::SUBJECT_TOP.0 | Self::CLIP_TOP.0);
    pub const BOTH_BOTTOM: Self = Self(Self::SUBJECT_BOTTOM.0 | Self::CLIP_BOTTOM.0);
    pub const ALL: Self = Self(Self::SUBJECT_BOTH.0 | Self::CLIP_BOTH.0);
    pub fn value(&self) -> u8 { self.0 }
}

impl std::ops::BitOr for SegmentFill {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for SegmentFill {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Segment {
    // start < end
    pub(crate) a: FixVec,
    // start
    pub(crate) b: FixVec,
    // end
    pub(crate) count: ShapeCount,
    pub(crate) fill: SegmentFill,
}

impl Segment {
    pub(crate) fn new(edge: &ShapeEdge) -> Self {
        Self {
            a: edge.a,
            b: edge.b,
            count: edge.count,
            fill: SegmentFill::NONE,
        }
    }
}