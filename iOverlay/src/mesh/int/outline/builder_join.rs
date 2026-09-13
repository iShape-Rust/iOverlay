use super::section::OffsetSection;
use crate::segm::boolean::ShapeCountBoolean;
use crate::segm::segment::Segment;
use alloc::vec::Vec;
use i_float::int::number::int::IntNumber;

pub(super) trait JoinBuilder<I: IntNumber> {
    /// Connects the offset endpoints of an outer corner. The endpoints differ.
    fn add_join(
        &mut self,
        previous: &OffsetSection<I>,
        next: &OffsetSection<I>,
        segments: &mut Vec<Segment<ShapeCountBoolean, I>>,
    );
}

impl<I: IntNumber> JoinBuilder<I> for crate::mesh::int::join::Join<I> {
    fn add_join(
        &mut self,
        previous: &OffsetSection<I>,
        next: &OffsetSection<I>,
        segments: &mut Vec<Segment<ShapeCountBoolean, I>>,
    ) {
        let direction = if previous.offset >= I::ZERO {
            crate::mesh::int::arc::ArcDirection::Counterclockwise
        } else {
            crate::mesh::int::arc::ArcDirection::Clockwise
        };
        let radius = I::from_wide(crate::mesh::int::math::abs(previous.offset));
        self.add(
            previous.b,
            previous.b_top,
            next.a_top,
            previous.direction,
            next.direction,
            radius,
            direction,
            segments,
        );
    }
}
