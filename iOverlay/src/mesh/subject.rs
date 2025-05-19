use crate::geom::x_segment::XSegment;
use crate::segm::segment::Segment;
use i_float::int::point::IntPoint;
use crate::segm::offset::ShapeCountOffset;

impl Segment<ShapeCountOffset> {

    #[inline]
    pub(crate) fn bold_subject_ab(p0: IntPoint, p1: IntPoint) -> Self {
        if p0 < p1 {
            Self {
                x_segment: XSegment { a: p0, b: p1 },
                count: ShapeCountOffset { subj: 1, bold: true },
            }
        } else {
            Self {
                x_segment: XSegment { a: p1, b: p0 },
                count: ShapeCountOffset { subj: -1, bold: true },
            }
        }
    }

    #[inline]
    pub(crate) fn weak_subject_ab(p0: IntPoint, p1: IntPoint) -> Self {
        if p0 < p1 {
            Self {
                x_segment: XSegment { a: p0, b: p1 },
                count: ShapeCountOffset { subj: 1, bold: false },
            }
        } else {
            Self {
                x_segment: XSegment { a: p1, b: p0 },
                count: ShapeCountOffset { subj: -1, bold: false },
            }
        }
    }
}