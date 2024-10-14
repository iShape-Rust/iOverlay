use std::cmp::Ordering;
use crate::geom::x_segment::XSegment;
use crate::geom::shape_count::ShapeCount;

#[derive(Debug, Clone)]
pub(super) struct CountSegment {
    pub(super) count: ShapeCount,
    pub(super) x_segment: XSegment
}

impl PartialEq<Self> for CountSegment {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.x_segment == other.x_segment
    }
}

impl Eq for CountSegment {}

impl PartialOrd for CountSegment {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CountSegment {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> Ordering {
        if self.x_segment.is_under_segment(&other.x_segment) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}
