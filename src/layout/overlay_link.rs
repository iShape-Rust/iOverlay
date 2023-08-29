use i_float::fix_vec::FixVec;
use i_shape::index_point::IndexPoint;
use crate::fill::segment::SegmentFill;

#[derive(Debug, Clone, Copy)]
pub struct OverlayLink {
    pub(crate) a: IndexPoint,
    pub(crate) b: IndexPoint,
    pub(crate) fill: SegmentFill
}

impl OverlayLink {

    pub (super) fn new(a: IndexPoint, b: IndexPoint, fill: SegmentFill) -> OverlayLink {
        OverlayLink { a, b, fill }
    }

    pub(crate) fn other(&self, index: IndexPoint) -> IndexPoint {
        if self.a == index { self.b } else { self.a }
    }

    pub fn ab(&self) -> (FixVec, FixVec) {
        (self.a.point, self.b.point)
    }

    pub fn fill(&self) -> SegmentFill {
        self.fill
    }

}