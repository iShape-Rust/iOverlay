use crate::geom::x_segment::XSegment;
use crate::segm::boolean::ShapeCountBoolean;
use crate::segm::segment::Segment;
use alloc::vec::Vec;
use i_float::int::number::int::IntNumber;
use i_float::int::point::IntPoint;

pub(super) trait SubjectSegments<I: IntNumber> {
    /// Adds a subject segment only when its endpoints differ.
    fn push_non_degenerate(&mut self, a: IntPoint<I>, b: IntPoint<I>);
}

impl<I: IntNumber> SubjectSegments<I> for Vec<Segment<ShapeCountBoolean, I>> {
    #[inline]
    fn push_non_degenerate(&mut self, a: IntPoint<I>, b: IntPoint<I>) {
        if a != b {
            self.push(Segment::subject(a, b));
        }
    }
}

impl<I: IntNumber> Segment<ShapeCountBoolean, I> {
    #[inline]
    pub(crate) fn subject(p0: IntPoint<I>, p1: IntPoint<I>) -> Self {
        debug_assert!(p0 != p1, "zero-length edges must be filtered before construction");
        if p0 < p1 {
            Self {
                x_segment: XSegment { a: p0, b: p1 },
                count: ShapeCountBoolean { subj: 1, clip: 0 },
                data: (),
            }
        } else {
            Self {
                x_segment: XSegment { a: p1, b: p0 },
                count: ShapeCountBoolean { subj: -1, clip: 0 },
                data: (),
            }
        }
    }
}
