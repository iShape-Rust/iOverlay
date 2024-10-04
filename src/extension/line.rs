use i_float::point::IntPoint;
use crate::segm::segment::{Segment, ToSegment};
use crate::segm::shape_count::ShapeCount;
use crate::segm::x_segment::XSegment;

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

pub trait LineGeometry {
    fn sqr_length(&self) -> i64;
}

impl LineGeometry for IntLine {
    fn sqr_length(&self) -> i64 {
        self[0].sqr_distance(self[1])
    }
}
