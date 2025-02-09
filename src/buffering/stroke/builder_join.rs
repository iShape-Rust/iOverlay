use crate::buffering::stroke::section::Section;
use crate::segm::segment::Segment;
use crate::segm::winding_count::ShapeCountBoolean;
use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use std::f64::consts::PI;
use i_float::float::vector::FloatPointMath;
use crate::buffering::rotator::Rotator;

pub(super) trait JoinBuilder<P: FloatPointCompatible<T>, T: FloatNumber> {
    fn add_join(
        &self,
        s0: &Section<P, T>,
        s1: &Section<P, T>,
        adapter: &FloatPointAdapter<P, T>,
        segments: &mut Vec<Segment<ShapeCountBoolean>>,
    );
    fn capacity(&self) -> usize;
}

pub(super) struct BevelJoinBuilder;

impl BevelJoinBuilder {
    #[inline]
    fn join_top<T: FloatNumber, P: FloatPointCompatible<T>>(
        s0: &Section<P, T>,
        s1: &Section<P, T>,
        adapter: &FloatPointAdapter<P, T>,
        segments: &mut Vec<Segment<ShapeCountBoolean>>,
    ) {
        let a0 = adapter.float_to_int(&s0.b_top);
        let b0 = adapter.float_to_int(&s1.a_top);

        if a0 != b0 {
            segments.push(Segment::subject_ab(a0, b0));
        }
    }

    #[inline]
    fn join_bot<T: FloatNumber, P: FloatPointCompatible<T>>(
        s0: &Section<P, T>,
        s1: &Section<P, T>,
        adapter: &FloatPointAdapter<P, T>,
        segments: &mut Vec<Segment<ShapeCountBoolean>>,
    ) {
        let a1 = adapter.float_to_int(&s0.b_bot);
        let b1 = adapter.float_to_int(&s1.a_bot);

        if a1 != b1 {
            segments.push(Segment::subject_ab(b1, a1));
        }
    }
}

impl<T: FloatNumber, P: FloatPointCompatible<T>> JoinBuilder<P, T> for BevelJoinBuilder {
    #[inline]
    fn add_join(
        &self,
        s0: &Section<P, T>,
        s1: &Section<P, T>,
        adapter: &FloatPointAdapter<P, T>,
        segments: &mut Vec<Segment<ShapeCountBoolean>>,
    ) {
        Self::join_top(s0, s1, adapter, segments);
        Self::join_bot(s0, s1, adapter, segments);
    }

    fn capacity(&self) -> usize {
        2
    }
}

pub(super) struct MiterJoinBuilder;

impl<T: FloatNumber, P: FloatPointCompatible<T>> JoinBuilder<P, T> for MiterJoinBuilder {
    fn add_join(
        &self,
        s0: &Section<P, T>,
        s1: &Section<P, T>,
        adapter: &FloatPointAdapter<P, T>,
        segments: &mut Vec<Segment<ShapeCountBoolean>>,
    ) {
        let a0 = adapter.float_to_int(&s0.b_top);
        let b0 = adapter.float_to_int(&s1.a_top);

        if a0 != b0 {
            segments.push(Segment::subject_ab(a0, b0));
        }

        let a1 = adapter.float_to_int(&s0.b_bot);
        let b1 = adapter.float_to_int(&s1.a_bot);

        if a1 != b1 {
            segments.push(Segment::subject_ab(b1, a1));
        }
    }

    fn capacity(&self) -> usize {
        4
    }
}

pub(super) struct RoundJoinBuilder<T> {
    inv_ratio: f64,
    limit_dot_product: T,
}

impl<T: FloatNumber> RoundJoinBuilder<T> {
    pub(super) fn new(ratio: T) -> Self {
        /// ratio = A / R
        let fixed_ratio = ratio.to_f64().max(0.45 * PI);
        let limit_angle = PI - fixed_ratio;
        let limit_dot_product = T::from_float(limit_angle.cos());

        Self {
            inv_ratio: 1.0 / fixed_ratio,
            limit_dot_product,
        }
    }
}
impl<T: FloatNumber, P: FloatPointCompatible<T>> JoinBuilder<P, T> for RoundJoinBuilder<T> {
    fn add_join(
        &self,
        s0: &Section<P, T>,
        s1: &Section<P, T>,
        adapter: &FloatPointAdapter<P, T>,
        segments: &mut Vec<Segment<ShapeCountBoolean>>,
    ) {
        let dot_product = FloatPointMath::dot_product(&s0.dir, &s1.dir);
        if dot_product < self.limit_dot_product {
            BevelJoinBuilder::join_top(s0, s1, adapter, segments);
            BevelJoinBuilder::join_bot(s0, s1, adapter, segments);
            return;
        }

        let angle = dot_product.to_f64().acos();
        let n = (angle * self.inv_ratio).round();
        let cnt = n as usize;
        let delta_angle = 1.0 / n;

        let cross_product = FloatPointMath::cross_product(&s0.dir, &s1.dir);
        let (rotator) = if cross_product > T::from_float(0.0) {
            BevelJoinBuilder::join_bot(s0, s1, adapter, segments);
            Rotator::<T>::with_angle(-delta_angle)
        } else {
            BevelJoinBuilder::join_top(s0, s1, adapter, segments);
            Rotator::<T>::with_angle(delta_angle)
        };

        // let center = s0.b;
        // let v =
        // for _ in 0..cnt {
        //
        //
        // }

    }

    fn capacity(&self) -> usize {
        4
    }
}
