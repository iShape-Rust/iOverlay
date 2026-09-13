use super::builder_join::JoinBuilder;
use super::section::OffsetSection;
use crate::mesh::subject::SubjectSegments;
use crate::mesh::uniq_iter::UniqueSegmentsIter;
use crate::segm::boolean::ShapeCountBoolean;
use crate::segm::segment::Segment;
use alloc::vec::Vec;
use i_float::int::number::int::IntNumber;
use i_float::int::number::wide_int::WideIntNumber;
use i_float::int::point::IntPoint;

pub(super) struct OutlineBuilder<I: IntNumber, J: JoinBuilder<I>> {
    offset: I,
    join_builder: J,
}

impl<I: IntNumber, J: JoinBuilder<I>> OutlineBuilder<I, J> {
    pub(super) fn new(offset: I, join_builder: J) -> Self {
        Self { offset, join_builder }
    }

    pub(super) fn build(&self, path: &[IntPoint<I>], segments: &mut Vec<Segment<ShapeCountBoolean, I>>) {
        let Some(mut iter) = UniqueSegmentsIter::new(path.iter().copied()) else {
            return;
        };
        let Some(first) = iter.next() else {
            return;
        };
        let first = OffsetSection::new(first, self.offset);
        segments.push_non_degenerate(first.a_top, first.b_top);
        let mut previous = first;
        for segment in iter {
            let next = OffsetSection::new(segment, self.offset);
            segments.push_non_degenerate(next.a_top, next.b_top);
            self.feed_join(&previous, &next, segments);
            previous = next;
        }
        self.feed_join(&previous, &first, segments);
    }

    #[inline]
    fn feed_join(
        &self,
        previous: &OffsetSection<I>,
        next: &OffsetSection<I>,
        segments: &mut Vec<Segment<ShapeCountBoolean, I>>,
    ) {
        let vi = next.b - next.a;
        let vp = previous.b - previous.a;
        let cross = vi.cross_product(vp);
        let outer_corner = if cross != I::Wide::ZERO {
            (cross > I::Wide::ZERO) == (self.offset < I::ZERO)
        } else {
            vi.dot_product(vp) < I::Wide::ZERO
        };
        if outer_corner {
            if previous.b_top != next.a_top {
                self.join_builder.add_join(previous, next, segments);
            }
        } else {
            segments.push_non_degenerate(previous.b_top, previous.b);
            segments.push_non_degenerate(next.a, next.a_top);
        }
    }
}
