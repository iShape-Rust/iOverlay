use crate::segm::boolean::ShapeCountBoolean;
use crate::segm::segment::Segment;
use core::marker::PhantomData;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_float::int::point::IntPoint;

#[derive(Debug, Clone)]
pub(super) struct OffsetSection<P: FloatPointCompatible<T>, T: FloatNumber> {
    pub(super) a: IntPoint,
    pub(super) b: IntPoint,
    pub(super) a_top: IntPoint,
    pub(super) b_top: IntPoint,
    pub(super) dir: P,
    pub(super) _phantom: PhantomData<T>,
}

impl<P: FloatPointCompatible<T>, T: FloatNumber> OffsetSection<P, T> {
    #[inline]
    pub(super) fn top_segment(&self) -> Option<Segment<ShapeCountBoolean>> {
        if self.a_top != self.b_top {
            Some(Segment::subject(self.a_top, self.b_top))
        } else {
            None
        }
    }

    #[inline]
    pub(super) fn a_segment(&self) -> Option<Segment<ShapeCountBoolean>> {
        if self.a_top != self.a {
            Some(Segment::subject(self.a, self.a_top))
        } else {
            None
        }
    }

    #[inline]
    pub(super) fn b_segment(&self) -> Option<Segment<ShapeCountBoolean>> {
        if self.b_top != self.b {
            Some(Segment::subject(self.b_top, self.b))
        } else {
            None
        }
    }
}
