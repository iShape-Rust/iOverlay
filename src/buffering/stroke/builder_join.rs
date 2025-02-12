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
    fn additional_offset(&self, radius: T) -> T;
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
        Self::add_segment(&s0.b_top, &s1.a_top, adapter, segments);
    }

    #[inline]
    fn join_bot<T: FloatNumber, P: FloatPointCompatible<T>>(
        s0: &Section<P, T>,
        s1: &Section<P, T>,
        adapter: &FloatPointAdapter<P, T>,
        segments: &mut Vec<Segment<ShapeCountBoolean>>,
    ) {
        Self::add_segment(&s1.a_bot, &s0.b_bot, adapter, segments);
    }

    #[inline]
    fn add_segment<T: FloatNumber, P: FloatPointCompatible<T>>(
        a: &P,
        b: &P,
        adapter: &FloatPointAdapter<P, T>,
        segments: &mut Vec<Segment<ShapeCountBoolean>>,
    ) {
        let ia = adapter.float_to_int(a);
        let ib = adapter.float_to_int(b);
        if ia != ib {
            segments.push(Segment::subject_ab(ia, ib));
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

    #[inline]
    fn capacity(&self) -> usize {
        2
    }

    #[inline]
    fn additional_offset(&self, radius: T) -> T {
        // add extra 10% to avoid problems with floating point precision.
        T::from_float(1.1) * radius
    }
}

pub(super) struct MiterJoinBuilder<T> {
    limit_dot_product: T,
    max_offset: T,
    max_length: T,
}

impl<T: FloatNumber> MiterJoinBuilder<T> {
    pub(super) fn new(angle: T, radius: T) -> Self {
        // angle - min possible angle
        let fixed_angle = angle.to_f64().max(0.01);
        let limit_dot_product = -T::from_float(fixed_angle.cos());

        let half_angle = 0.5 * fixed_angle;
        let tan = half_angle.tan();

        let r = radius.to_f64();
        let l = r / tan;
        // add extra 10% to avoid problems with floating point precision.
        let max_offset = T::from_float(1.1 * (r * r + l * l).sqrt());
        let max_length = T::from_float(l);

        Self {
            limit_dot_product,
            max_offset,
            max_length
        }
    }
}

impl<T: FloatNumber, P: FloatPointCompatible<T>> JoinBuilder<P, T> for MiterJoinBuilder<T> {
    fn add_join(
        &self,
        s0: &Section<P, T>,
        s1: &Section<P, T>,
        adapter: &FloatPointAdapter<P, T>,
        segments: &mut Vec<Segment<ShapeCountBoolean>>,
    ) {
        let dot_product = FloatPointMath::dot_product(&s0.dir, &s1.dir);
        let cross_product = FloatPointMath::cross_product(&s0.dir, &s1.dir);
        let turn = cross_product > T::from_float(0.0);

        let is_limited = self.limit_dot_product > dot_product;

        if is_limited {
            let (pa, pb, ac, bc) = if turn {
                BevelJoinBuilder::join_top(s0, s1, adapter, segments);
                // (s1.a_bot, s0.b_bot, s1.dir, s0.dir);
                let (pa, pb, va, vb) = (s1.a_bot, s0.b_bot, s1.dir, s0.dir);

                let ax = pa.x() - self.max_length * va.x();
                let ay = pa.y() - self.max_length * va.y();
                let bx = pb.x() + self.max_length * vb.x();
                let by = pb.y() + self.max_length * vb.y();

                let ac = P::from_xy(ax, ay);
                let bc = P::from_xy(bx, by);

                (pa, pb, ac, bc)
            } else {
                BevelJoinBuilder::join_bot(s0, s1, adapter, segments);
                let (pa, pb, va, vb) = (s0.b_top, s1.a_top, s0.dir, s1.dir);

                let ax = pa.x() + self.max_length * va.x();
                let ay = pa.y() + self.max_length * va.y();
                let bx = pb.x() - self.max_length * vb.x();
                let by = pb.y() - self.max_length * vb.y();

                let ac = P::from_xy(ax, ay);
                let bc = P::from_xy(bx, by);

                (pa, pb, ac, bc)
            };

            let ia = adapter.float_to_int(&pa);
            let ib = adapter.float_to_int(&pb);
            let iac = adapter.float_to_int(&ac);
            let ibc = adapter.float_to_int(&bc);

            segments.push(Segment::subject_ab(ia, iac));
            segments.push(Segment::subject_ab(iac, ibc));
            segments.push(Segment::subject_ab(ibc, ib));
        } else {
            let (pa, pb, va, vb) = if turn {
                BevelJoinBuilder::join_top(s0, s1, adapter, segments);
                (s1.a_bot, s0.b_bot, s1.dir, s0.dir)
            } else {
                BevelJoinBuilder::join_bot(s0, s1, adapter, segments);
                (s0.b_top, s1.a_top, s0.dir, s1.dir)
            };

            let k = (pb.x() - pa.x()) / (va.x() + vb.x());
            let x = pa.x() + k * va.x();
            let y = pa.y() + k * va.y();
            let c = P::from_xy(x, y);

            let ia = adapter.float_to_int(&pa);
            let ib = adapter.float_to_int(&pb);
            let ic = adapter.float_to_int(&c);

            segments.push(Segment::subject_ab(ia, ic));
            segments.push(Segment::subject_ab(ic, ib));
        }
    }

    #[inline]
    fn capacity(&self) -> usize {
        4
    }

    #[inline]
    fn additional_offset(&self, _radius: T) -> T {
        self.max_offset
    }
}

pub(super) struct RoundJoinBuilder<T> {
    inv_ratio: f64,
    average_count: usize,
    radius: T,
    limit_dot_product: T,
}

impl<T: FloatNumber> RoundJoinBuilder<T> {
    pub(super) fn new(ratio: T, radius: T) -> Self {
        // ratio = A / R
        let fixed_ratio = ratio.to_f64().min(0.25 * PI);
        let limit_dot_product = T::from_float(fixed_ratio.cos());
        let average_count = (0.6 * PI / fixed_ratio) as usize + 2;
        Self {
            inv_ratio: 1.0 / fixed_ratio,
            average_count,
            radius,
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
        if self.limit_dot_product < dot_product {
            BevelJoinBuilder::join_top(s0, s1, adapter, segments);
            BevelJoinBuilder::join_bot(s0, s1, adapter, segments);
            return;
        }

        let angle = dot_product.to_f64().acos();
        let n = (angle * self.inv_ratio).round();
        let cnt = n as usize;
        let delta_angle = angle / n;

        let cross_product = FloatPointMath::cross_product(&s0.dir, &s1.dir);
        let (start, end, dir) = if cross_product > T::from_float(0.0) {
            BevelJoinBuilder::join_top(s0, s1, adapter, segments);
            let ortho = P::from_xy(s1.dir.y(), -s1.dir.x());
            (s1.a_bot, s0.b_bot, ortho)
        } else {
            BevelJoinBuilder::join_bot(s0, s1, adapter, segments);
            let ortho = P::from_xy(-s0.dir.y(), s0.dir.x());
            (s0.b_top, s1.a_top, ortho)
        };
        let rotator = Rotator::<T>::with_angle(-delta_angle);

        let center = s0.b;
        let mut v = dir;
        let mut a = adapter.float_to_int(&start);
        for _ in 1..cnt {
            v = rotator.rotate(&v);
            let p = FloatPointMath::add(&center, &FloatPointMath::scale(&v, self.radius));

            let b = adapter.float_to_int(&p);
            if a != b {
                segments.push(Segment::subject_ab(a, b));
                a = b;
            }
        }

        let b = adapter.float_to_int(&end);
        if a != b {
            segments.push(Segment::subject_ab(a, b));
        }
    }

    #[inline]
    fn capacity(&self) -> usize {
        self.average_count
    }

    #[inline]
    fn additional_offset(&self, radius: T) -> T {
        // add extra 10% to avoid problems with floating point precision.
        T::from_float(1.1) * radius
    }
}