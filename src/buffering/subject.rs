use crate::geom::x_segment::XSegment;
use crate::segm::segment::Segment;
use crate::segm::winding_count::ShapeCountBoolean;
use i_float::int::point::IntPoint;

impl Segment<ShapeCountBoolean> {
    #[inline]
    pub(crate) fn subject_ab(p0: IntPoint, p1: IntPoint) -> Self {
        if p0 < p1 {
            Self {
                x_segment: XSegment { a: p0, b: p1 },
                count: ShapeCountBoolean::SUBJ_DIRECT,
            }
        } else {
            Self {
                x_segment: XSegment { a: p1, b: p0 },
                count: ShapeCountBoolean::SUBJ_INVERT,
            }
        }
    }
}
