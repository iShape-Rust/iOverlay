use i_float::point::IntPoint;
use crate::geom::x_segment::XSegment;
use crate::segm::segment::{Segment, ToSegment};
use crate::segm::shape_count::ShapeCount;

pub type IntLine = [IntPoint; 2];

impl ToSegment for IntLine {
    #[inline(always)]
    fn to_segment(&self, count: ShapeCount) -> Segment {
        let a = self[0];
        let b = self[1];
        if a < b {
            Segment { x_segment: XSegment { a, b }, count }
        } else {
            Segment { x_segment: XSegment { a: b, b: a }, count }
        }
    }
}

impl ToSegment for [IntPoint] {
    #[inline(always)]
    fn to_segment(&self, count: ShapeCount) -> Segment {
        let a = self[0];
        let b = self[1];
        if a < b {
            Segment { x_segment: XSegment { a, b }, count }
        } else {
            Segment { x_segment: XSegment { a: b, b: a }, count }
        }
    }
}