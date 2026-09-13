use super::section::OffsetSection;
use crate::mesh::subject::SubjectSegments;
use crate::segm::boolean::ShapeCountBoolean;
use crate::segm::segment::Segment;
use alloc::vec::Vec;
use i_float::int::number::int::IntNumber;
use i_float::int::number::wide_int::WideIntNumber;

pub(super) trait JoinBuilder<I: IntNumber> {
    /// Connects the offset endpoints of an outer corner. The endpoints differ.
    fn add_join(
        &self,
        previous: &OffsetSection<I>,
        next: &OffsetSection<I>,
        segments: &mut Vec<Segment<ShapeCountBoolean, I>>,
    );

    /// Nonnegative conservative padding, including temporary join points.
    fn additional_offset(&self, offset: I) -> I::Wide;
}

pub(super) struct BevelJoinBuilder;

impl<I: IntNumber> JoinBuilder<I> for BevelJoinBuilder {
    #[inline]
    fn add_join(
        &self,
        previous: &OffsetSection<I>,
        next: &OffsetSection<I>,
        segments: &mut Vec<Segment<ShapeCountBoolean, I>>,
    ) {
        segments.push_non_degenerate(previous.b_top, next.a_top);
    }

    #[inline]
    fn additional_offset(&self, offset: I) -> I::Wide {
        let offset = offset.to_wide();
        if offset < I::Wide::ZERO { -offset } else { offset }
    }
}
