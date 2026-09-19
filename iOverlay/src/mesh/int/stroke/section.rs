use crate::mesh::int::math::{backend::MeshMath, point};
use crate::mesh::subject::SubjectSegments;
use crate::segm::{boolean::ShapeCountBoolean, segment::Segment};
use alloc::vec::Vec;
use i_float::int::{number::int::IntNumber, point::IntPoint, unit_vector::UnitIntVector};

#[derive(Clone, Copy)]
pub(super) struct Section<I: IntNumber> {
    pub(super) a: IntPoint<I>,
    pub(super) b: IntPoint<I>,
    pub(super) a_left: IntPoint<I>,
    pub(super) a_right: IntPoint<I>,
    pub(super) b_left: IntPoint<I>,
    pub(super) b_right: IntPoint<I>,
    pub(super) dir: UnitIntVector<I>,
}

impl<I: IntNumber> Section<I> {
    pub(super) fn new<M: MeshMath<I>>(a: IntPoint<I>, b: IntPoint<I>, radius: I) -> Self {
        let dir = M::normalize(b - a).expect("unique section");
        let v = M::scale(dir, radius);
        Self {
            a,
            b,
            dir,
            a_left: point(a, -v.y, v.x),
            a_right: point(a, v.y, -v.x),
            b_left: point(b, -v.y, v.x),
            b_right: point(b, v.y, -v.x),
        }
    }
    pub(super) fn add(&self, segments: &mut Vec<Segment<ShapeCountBoolean, I>>) {
        for (a, b) in [(self.a_right, self.b_right), (self.b_left, self.a_left)] {
            segments.push_non_degenerate(a, b);
        }
    }
}
