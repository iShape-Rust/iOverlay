use std::cmp::Ordering;
use crate::geom::x_segment::XSegment;
use crate::split::shape_count::ShapeCount;

#[derive(Debug, Clone)]
pub(crate) struct CountSegment {
    pub(crate) count: ShapeCount,
    pub(crate) x_segment: XSegment
}

impl PartialEq<Self> for CountSegment {
    fn eq(&self, other: &Self) -> bool {
        self.x_segment == other.x_segment
    }
}

impl PartialOrd<Self> for CountSegment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.x_segment.order(&other.x_segment))
    }
}

impl Eq for CountSegment {}

impl Ord for CountSegment {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.x_segment.is_under_segment(other.x_segment) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}
