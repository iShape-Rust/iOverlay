use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use crate::buffering::stroke::section::Section;
use crate::segm::segment::Segment;
use crate::segm::winding_count::ShapeCountBoolean;

pub(super) trait JoinBuilder<P: FloatPointCompatible<T>, T: FloatNumber> {
    fn add_join(&self, s0: &Section<P, T>, s1: &Section<P, T>, adapter: &FloatPointAdapter<P, T>, segments: &mut Vec<Segment<ShapeCountBoolean>>);
    fn capacity(&self) -> usize;
}

pub(super) struct BevelJoinBuilder;

impl<T: FloatNumber, P: FloatPointCompatible<T>> JoinBuilder<P, T> for BevelJoinBuilder {
    fn add_join(&self, s0: &Section<P, T>, s1: &Section<P, T>, adapter: &FloatPointAdapter<P, T>, segments: &mut Vec<Segment<ShapeCountBoolean>>) {
        let a0 = adapter.float_to_int(&s0.b_top);
        let b0 = adapter.float_to_int(&s1.a_top);

        if a0 != b0 {
            segments.push(Segment::subject_ab(a0, b0));
        }

        let a1 = adapter.float_to_int(&s0.b_bot);
        let b1 = adapter.float_to_int(&s1.a_top);

        if a1 != b1 {
            segments.push(Segment::subject_ab(b1, a1));
        }
    }

    fn capacity(&self) -> usize {
        4
    }
}

pub(super) struct MiterJoinBuilder;

impl<T: FloatNumber, P: FloatPointCompatible<T>> JoinBuilder<P, T> for MiterJoinBuilder {
    fn add_join(&self, s0: &Section<P, T>, s1: &Section<P, T>, adapter: &FloatPointAdapter<P, T>, segments: &mut Vec<Segment<ShapeCountBoolean>>) {
        let a0 = adapter.float_to_int(&s0.b_top);
        let b0 = adapter.float_to_int(&s1.a_top);

        if a0 != b0 {
            segments.push(Segment::subject_ab(a0, b0));
        }

        let a1 = adapter.float_to_int(&s0.b_bot);
        let b1 = adapter.float_to_int(&s1.a_top);

        if a1 != b1 {
            segments.push(Segment::subject_ab(b1, a1));
        }
    }

    fn capacity(&self) -> usize {
        4
    }
}

pub(super) struct RoundJoinBuilder;

impl<T: FloatNumber, P: FloatPointCompatible<T>> JoinBuilder<P, T> for RoundJoinBuilder {
    fn add_join(&self, s0: &Section<P, T>, s1: &Section<P, T>, adapter: &FloatPointAdapter<P, T>, segments: &mut Vec<Segment<ShapeCountBoolean>>) {
        let a0 = adapter.float_to_int(&s0.b_top);
        let b0 = adapter.float_to_int(&s1.a_top);

        if a0 != b0 {
            segments.push(Segment::subject_ab(a0, b0));
        }

        let a1 = adapter.float_to_int(&s0.b_bot);
        let b1 = adapter.float_to_int(&s1.a_top);

        if a1 != b1 {
            segments.push(Segment::subject_ab(b1, a1));
        }
    }

    fn capacity(&self) -> usize {
        4
    }
}