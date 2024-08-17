use std::cmp::Ordering;
use i_float::point::IntPoint;
use crate::segm::x_segment::XSegment;
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
pub struct Segment {
    pub(crate) x_segment: XSegment,
    pub(crate) count: ShapeCount
}

impl Segment {

    pub(crate) const ZERO: Segment = Segment {
        x_segment: XSegment { a: IntPoint::ZERO, b: IntPoint::ZERO },
        count: ShapeCount { subj: 0, clip: 0 }
    };

    #[inline(always)]
    pub fn new(a: IntPoint, b: IntPoint, count: ShapeCount) -> Self {
        if a < b {
            Self { x_segment: XSegment { a, b }, count }
        } else {
            Self { x_segment: XSegment { a: b, b: a }, count }
        }
    }

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

pub(crate) trait ShapeEdgesMerge {
    fn merge_if_needed(&mut self);
}

impl ShapeEdgesMerge for Vec<Segment> {
    fn merge_if_needed(&mut self) {
        let n = self.len();

        if n < 2 { return; }

        let mut i = 1;
        while i < n {
            if self[i - 1].x_segment.eq(&self[i].x_segment) {
                break;
            }
            i += 1;
        }

        if i == n { return; }

        let mut j = i - 1;
        let mut prev = self[j];

        while i < n {
            if prev.x_segment.eq(&self[i].x_segment) {
                prev.count = prev.count.add(self[i].count)
            } else {
                if prev.count.is_not_empty() {
                    self[j] = prev;
                    j += 1;
                }
                prev = self[i];
            }
            i += 1;
        }

        if prev.count.is_not_empty() {
            self[j] = prev;
            j += 1;
        }

        if j < n {
            self.truncate(j);
        }
    }
}