use crate::core::edge_data::OverlayEdgeData;
use crate::geom::x_segment::XSegment;
use crate::segm::winding::WindingCount;
use core::cmp::Ordering;
use i_float::int::number::int::IntNumber;
use i_float::int::point::IntPoint;

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
pub(crate) struct Segment<C, D = (), I: IntNumber = i32> {
    pub(crate) x_segment: XSegment<I>,
    pub(crate) count: C,
    pub(crate) data: D,
}

impl<C: WindingCount, I: IntNumber> Segment<C, (), I> {
    #[inline(always)]
    #[allow(dead_code)]
    pub(crate) fn create_and_validate(a: IntPoint<I>, b: IntPoint<I>, count: C) -> Self {
        Self::create_and_validate_with_data(a, b, count, ())
    }
}

impl<C: WindingCount, D: OverlayEdgeData<C>, I: IntNumber> Segment<C, D, I> {
    #[inline(always)]
    pub(crate) fn create_and_validate_with_data(a: IntPoint<I>, b: IntPoint<I>, count: C, data: D) -> Self {
        if a < b {
            Self {
                x_segment: XSegment { a, b },
                count,
                data,
            }
        } else {
            Self {
                x_segment: XSegment { a: b, b: a },
                count: count.invert(),
                data: data.reversed(),
            }
        }
    }
}

impl<C, D, I: IntNumber> PartialEq<Self> for Segment<C, D, I> {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.x_segment == other.x_segment
    }
}

impl<C, D, I: IntNumber> Eq for Segment<C, D, I> {}

impl<C, D, I: IntNumber> PartialOrd for Segment<C, D, I> {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<C, D, I: IntNumber> Ord for Segment<C, D, I> {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> Ordering {
        self.x_segment.cmp(&other.x_segment)
    }
}
