use i_float::point::Point;
use crate::geom::x_order::XOrder;
use crate::geom::x_segment::XSegment;
use crate::split::shape_count::ShapeCount;

#[derive(Debug, Clone, Copy)]
pub struct ShapeEdge {
    pub(crate) x_segment: XSegment,
    pub(crate) count: ShapeCount,
}

impl ShapeEdge {
    pub(crate) const ZERO: ShapeEdge = ShapeEdge {
        x_segment: XSegment { a: Point::ZERO, b: Point::ZERO },
        count: ShapeCount { subj: 0, clip: 0 },
    };

    pub fn new(a: Point, b: Point, count: ShapeCount) -> Self {
        if a.order_by_line_compare(b) {
            Self { x_segment: XSegment { a, b }, count }
        } else {
            Self { x_segment: XSegment { a: b, b: a }, count }
        }
    }
}