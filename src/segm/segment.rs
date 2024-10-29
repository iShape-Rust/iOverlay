use std::cmp::Ordering;
use i_float::int::point::IntPoint;
use i_key_sort::index::{BinKey, BinLayout};
use crate::geom::x_segment::XSegment;
use crate::segm::shape_count::ShapeCount;

pub type SegmentFill = u8;

pub const NONE: SegmentFill = 0;

pub const SUBJ_TOP: SegmentFill = 0b0001;
pub const SUBJ_BOTTOM: SegmentFill = 0b0010;
pub const CLIP_TOP: SegmentFill = 0b0100;
pub const CLIP_BOTTOM: SegmentFill = 0b1000;

pub const SUBJ_BOTH: SegmentFill = SUBJ_TOP | SUBJ_BOTTOM;
pub const CLIP_BOTH: SegmentFill = CLIP_TOP | CLIP_BOTTOM;
pub const BOTH_TOP: SegmentFill = SUBJ_TOP | CLIP_TOP;
pub const BOTH_BOTTOM: SegmentFill = SUBJ_BOTTOM | CLIP_BOTTOM;

pub const ALL: SegmentFill = SUBJ_BOTH | CLIP_BOTH;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Segment {
    pub(crate) x_segment: XSegment,
    pub(crate) count: ShapeCount,
}

impl Segment {
    #[inline(always)]
    pub(crate) fn create_and_validate(a: IntPoint, b: IntPoint, count: ShapeCount) -> Self {
        if a < b {
            Self { x_segment: XSegment { a, b }, count }
        } else {
            Self { x_segment: XSegment { a: b, b: a }, count: count.invert() }
        }
    }
}

impl PartialEq<Self> for Segment {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.x_segment == other.x_segment
    }
}

impl Eq for Segment {}

impl PartialOrd for Segment {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Segment {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> Ordering {
        self.x_segment.cmp(&other.x_segment)
    }
}

impl BinKey<i32> for Segment {
    #[inline(always)]
    fn bin_key(&self) -> i32 {
        self.x_segment.bin_key()
    }

    #[inline(always)]
    fn bin_index(&self, layout: &BinLayout<i32>) -> usize {
        self.x_segment.bin_index(layout)
    }
}

pub(crate) trait ToSegment {
    fn to_segment(&self, shape_count: ShapeCount) -> Segment;
}