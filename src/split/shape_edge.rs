use std::cmp::Ordering;
use i_float::point::IntPoint;
use crate::x_segment::XSegment;
use crate::split::shape_count::ShapeCount;

#[derive(Debug, Clone, Copy)]
pub struct ShapeEdge {
    pub(crate) x_segment: XSegment,
    pub(crate) count: ShapeCount,
}

impl ShapeEdge {
    pub(crate) const ZERO: ShapeEdge = ShapeEdge {
        x_segment: XSegment { a: IntPoint::ZERO, b: IntPoint::ZERO },
        count: ShapeCount { subj: 0, clip: 0 },
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
    pub(super) fn create_and_validate(a: IntPoint, b: IntPoint, count: ShapeCount) -> Self {
        if a < b {
            Self { x_segment: XSegment { a, b }, count }
        } else {
            Self { x_segment: XSegment { a: b, b: a }, count: count.invert() }
        }
    }
}

impl PartialEq<Self> for ShapeEdge {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.x_segment == other.x_segment
    }
}

impl Eq for ShapeEdge {}

impl PartialOrd for ShapeEdge {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ShapeEdge {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> Ordering {
        self.x_segment.cmp(&other.x_segment)
    }
}

pub(crate) trait ShapeEdgesMerge {
    fn merge_if_needed(&mut self);
}

impl ShapeEdgesMerge for Vec<ShapeEdge> {
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