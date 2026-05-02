use crate::core::extract::VisitState;
use crate::core::overlay_rule::OverlayRule;
use crate::geom::id_point::IdPoint;
use crate::segm::segment::SegmentFill;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy)]
pub(crate) struct OverlayLink<D = ()> {
    pub(crate) a: IdPoint,
    pub(crate) b: IdPoint,
    pub(crate) fill: SegmentFill,
    pub(crate) data: D,
}

impl<D> OverlayLink<D> {
    #[inline(always)]
    pub(crate) fn new_with_data(a: IdPoint, b: IdPoint, fill: SegmentFill, data: D) -> OverlayLink<D> {
        OverlayLink { a, b, fill, data }
    }

    #[inline(always)]
    pub(crate) fn other(&self, node_id: usize) -> IdPoint {
        if self.a.id == node_id { self.b } else { self.a }
    }

    #[inline(always)]
    pub(crate) fn is_direct(&self) -> bool {
        self.a.point < self.b.point
    }
}

pub(crate) trait OverlayLinkFilter {
    fn filter_by_overlay_into(&self, overlay_rule: OverlayRule, buffer: &mut Vec<VisitState>);
}
