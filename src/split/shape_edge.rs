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

    pub fn new(a: IntPoint, b: IntPoint, count: ShapeCount) -> Self {
        if a < b {
            Self { x_segment: XSegment { a, b }, count }
        } else {
            Self { x_segment: XSegment { a: b, b: a }, count }
        }
    }
}

impl PartialEq<Self> for ShapeEdge {
    fn eq(&self, other: &Self) -> bool {
        self.x_segment == other.x_segment
    }
}

impl Eq for ShapeEdge {}

impl PartialOrd for ShapeEdge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ShapeEdge {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.x_segment.is_less(&other.x_segment) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}
