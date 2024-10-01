use i_float::point::IntPoint;
use crate::core::overlay::ShapeType;
use crate::segm::segment::{Segment, ToSegment};
use crate::segm::shape_count::ShapeCount;

pub type IntLine = [IntPoint; 2];

impl ToSegment for IntLine {
    #[inline(always)]
    fn to_segment(&self, shape_type: ShapeType) -> Segment {
        Segment::create_and_validate(self[0], self[1], ShapeCount::with_shape_type(shape_type))
    }
}