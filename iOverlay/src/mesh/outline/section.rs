use crate::segm::boolean::ShapeCountBoolean;
use crate::segm::segment::Segment;
use i_float::float::compatible::FloatPointCompatible;
use i_float::int::point::IntPoint;

#[derive(Debug, Clone)]
pub(super) struct OffsetSection<P: FloatPointCompatible> {
    pub(super) a: IntPoint,
    pub(super) b: IntPoint,
    pub(super) a_top: IntPoint,
    pub(super) b_top: IntPoint,
    pub(super) dir: P,
}

impl<P: FloatPointCompatible> OffsetSection<P> {
    #[inline]
    pub(super) fn top_segment(&self) -> Option<Segment<ShapeCountBoolean, i32>> {
        if self.a_top != self.b_top {
            Some(Segment::subject(self.a_top, self.b_top))
        } else {
            None
        }
    }

    #[inline]
    pub(super) fn a_segment(&self) -> Option<Segment<ShapeCountBoolean, i32>> {
        if self.a_top != self.a {
            Some(Segment::subject(self.a, self.a_top))
        } else {
            None
        }
    }

    #[inline]
    pub(super) fn b_segment(&self) -> Option<Segment<ShapeCountBoolean, i32>> {
        if self.b_top != self.b {
            Some(Segment::subject(self.b_top, self.b))
        } else {
            None
        }
    }
}
