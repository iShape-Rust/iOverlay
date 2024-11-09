use i_float::int::point::IntPoint;
use crate::geom::x_segment::XSegment;
use crate::segm::segment::{Segment, ToSegment};

pub type IntLine = [IntPoint; 2];

impl<C: Send> ToSegment<C> for IntLine {
    #[inline(always)]
    fn to_segment(&self, count: C) -> Segment<C> {
        let a = self[0];
        let b = self[1];
        if a < b {
            Segment { x_segment: XSegment { a, b }, count }
        } else {
            Segment { x_segment: XSegment { a: b, b: a }, count }
        }
    }
}

impl<C: Send> ToSegment<C> for [IntPoint] {
    #[inline(always)]
    fn to_segment(&self, count: C) -> Segment<C> {
        let a = self[0];
        let b = self[1];
        if a < b {
            Segment { x_segment: XSegment { a, b }, count }
        } else {
            Segment { x_segment: XSegment { a: b, b: a }, count }
        }
    }
}