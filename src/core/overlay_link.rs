use i_float::point::IntPoint;
use crate::segm::segment::SegmentFill;
use crate::id_point::IdPoint;

#[derive(Debug, Clone, Copy)]
pub struct OverlayLink {
    pub(crate) a: IdPoint,
    pub(crate) b: IdPoint,
    pub(crate) fill: SegmentFill,
}

impl OverlayLink {
    #[inline(always)]
    pub(super) fn new(a: IdPoint, b: IdPoint, fill: SegmentFill) -> OverlayLink {
        OverlayLink { a, b, fill }
    }

    #[inline(always)]
    pub(crate) fn other(&self, index: IdPoint) -> IdPoint {
        if self.a == index { self.b } else { self.a }
    }

    #[inline(always)]
    pub fn ab(&self) -> (IntPoint, IntPoint) {
        (self.a.point, self.b.point)
    }

    #[inline(always)]
    pub fn fill(&self) -> SegmentFill {
        self.fill
    }
}