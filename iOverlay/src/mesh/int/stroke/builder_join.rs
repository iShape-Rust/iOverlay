use super::section::Section;
use crate::mesh::int::{arc::ArcDirection, join::Join, math::backend::MeshMath};
use crate::mesh::subject::SubjectSegments;
use crate::segm::{boolean::ShapeCountBoolean, segment::Segment};
use alloc::vec::Vec;
use i_float::int::number::{int::IntNumber, wide_int::WideIntNumber};

pub(super) trait JoinBuilder<I: IntNumber> {
    fn add_join(
        &mut self,
        a: Section<I>,
        b: Section<I>,
        radius: I,
        segments: &mut Vec<Segment<ShapeCountBoolean, I>>,
    );
}

impl<I: IntNumber, M: MeshMath<I>> JoinBuilder<I> for Join<I, M> {
    fn add_join(
        &mut self,
        a: Section<I>,
        b: Section<I>,
        radius: I,
        segments: &mut Vec<Segment<ShapeCountBoolean, I>>,
    ) {
        let cross = (a.b - a.a).cross_product(b.b - b.a);
        let (from, to, incoming, outgoing) = if cross >= I::Wide::ZERO {
            (a.b_right, b.a_right, a.dir, b.dir)
        } else {
            (
                b.a_left,
                a.b_left,
                M::normalize(b.a - b.b).unwrap(),
                M::normalize(a.a - a.b).unwrap(),
            )
        };
        if cross >= I::Wide::ZERO {
            segments.push_non_degenerate(b.a_left, a.b_left);
        } else {
            segments.push_non_degenerate(a.b_right, b.a_right);
        }
        self.add(
            a.b,
            from,
            to,
            incoming,
            outgoing,
            radius,
            ArcDirection::Counterclockwise,
            segments,
        );
    }
}
