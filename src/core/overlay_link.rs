use crate::segm::segment::SegmentFill;
use crate::id_point::IdPoint;

#[derive(Debug, Clone, Copy)]
pub(crate) struct OverlayLink {
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
    pub(crate) fn other(&self, index: &IdPoint) -> IdPoint {
        if self.a.id == index.id { self.b } else { self.a }
    }
}