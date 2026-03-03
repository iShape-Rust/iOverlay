use crate::geom::x_segment::XSegment;
use crate::segm::segment::Segment;
use i_float::int::point::IntPoint;
use crate::segm::boolean::ShapeCountBoolean;

impl Segment<ShapeCountBoolean> {
    #[inline]
    pub(crate) fn subject_ab(p0: IntPoint, p1: IntPoint) -> Self {
        if p0 < p1 {
            Self {
                x_segment: XSegment { a: p0, b: p1 },
                count: ShapeCountBoolean { subj: 1, clip: 0 },
            }
        } else {
            Self {
                x_segment: XSegment { a: p1, b: p0 },
                count: ShapeCountBoolean { subj: -1, clip: 0 },
            }
        }
    }
}
