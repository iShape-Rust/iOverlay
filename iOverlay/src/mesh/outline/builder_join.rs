use alloc::vec::Vec;
use crate::segm::segment::Segment;
use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use core::f64::consts::PI;
use i_float::float::vector::FloatPointMath;
use crate::mesh::miter::{Miter, SharpMiter};
use crate::mesh::outline::section::Section;
use crate::mesh::rotator::Rotator;
use crate::segm::offset::ShapeCountOffset;

pub(super) trait JoinBuilder<P: FloatPointCompatible<T>, T: FloatNumber> {
    fn add_join(
        &self,
        s0: &Section<P, T>,
        s1: &Section<P, T>,
        adapter: &FloatPointAdapter<P, T>,
        segments: &mut Vec<Segment<ShapeCountOffset>>,
    );
    fn capacity(&self) -> usize;
    fn additional_offset(&self, radius: T) -> T;
}

pub(super) struct BevelJoinBuilder;

impl BevelJoinBuilder {
    #[inline]
    fn join_weak<T: FloatNumber, P: FloatPointCompatible<T>>(
        s0: &Section<P, T>,
        s1: &Section<P, T>,
        adapter: &FloatPointAdapter<P, T>,
        segments: &mut Vec<Segment<ShapeCountOffset>>,
    ) {
        Self::add_weak_segment(&s0.b_top, &s1.a_top, adapter, segments);
    }

    #[inline]
    fn add_weak_segment<T: FloatNumber, P: FloatPointCompatible<T>>(
        a: &P,
        b: &P,
        adapter: &FloatPointAdapter<P, T>,
        segments: &mut Vec<Segment<ShapeCountOffset>>,
    ) {
        let ia = adapter.float_to_int(a);
        let ib = adapter.float_to_int(b);
        if ia != ib {
            segments.push(Segment::weak_subject_ab(ia, ib));
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
        segments: &mut Vec<Segment<ShapeCountOffset>>,
    ) {
        Self::join_weak(s0, s1, adapter, segments);
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
    expand: bool,
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

        let r = radius.to_f64().abs();
        let l = r / tan;

        // add extra 10% to avoid problems with floating point precision.
        let max_offset = T::from_float(1.1 * (r * r + l * l).sqrt());
        let max_length = T::from_float(l);

        let expand = radius >= T::from_float(0.0);
        Self {
            limit_dot_product,
            max_offset,
            max_length,
            expand,
        }
    }
}

impl<T: FloatNumber, P: FloatPointCompatible<T>> JoinBuilder<P, T> for MiterJoinBuilder<T> {
    fn add_join(
        &self,
        s0: &Section<P, T>,
        s1: &Section<P, T>,
        adapter: &FloatPointAdapter<P, T>,
        segments: &mut Vec<Segment<ShapeCountOffset>>,
    ) {
        let cross_product = FloatPointMath::cross_product(&s0.dir, &s1.dir);
        let turn = cross_product >= T::from_float(0.0);
        if turn == self.expand {
            BevelJoinBuilder::join_weak(s0, s1, adapter, segments);
            return
        }

        let pa = s0.b_top;
        let pb = s1.a_top;

        let ia = adapter.float_to_int(&pa);
        let ib = adapter.float_to_int(&pb);

        let sq_len = ia.sqr_distance(ib);
        if sq_len < 4 {
            BevelJoinBuilder::join_weak(s0, s1, adapter, segments);
            return
        }

        let dot_product = FloatPointMath::dot_product(&s0.dir, &s1.dir);
        let is_limited = self.limit_dot_product > dot_product;

        if is_limited {
            let (va, vb) = (s0.dir, s1.dir);

            let ax = pa.x() + self.max_length * va.x();
            let ay = pa.y() + self.max_length * va.y();
            let bx = pb.x() - self.max_length * vb.x();
            let by = pb.y() - self.max_length * vb.y();

            let ac = P::from_xy(ax, ay);
            let bc = P::from_xy(bx, by);

            let iac = adapter.float_to_int(&ac);
            let ibc = adapter.float_to_int(&bc);

            if ia != iac {
                segments.push(Segment::bold_subject_ab(ia, iac));
            }
            if iac != ibc {
                segments.push(Segment::bold_subject_ab(iac, ibc));
            }
            if ibc != ib {
                segments.push(Segment::bold_subject_ab(ibc, ib));
            }
        } else {
            match Miter::sharp(pa, pb, s0.dir, s1.dir, adapter) {
                SharpMiter::AB(a, b) => segments.push(Segment::bold_subject_ab(a, b)),
                SharpMiter::AcB(a, c, b) => {
                    segments.push(Segment::bold_subject_ab(a, c));
                    segments.push(Segment::bold_subject_ab(c, b));
                },
                SharpMiter::Degenerate => {}
            }
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
    inv_ratio: T,
    average_count: usize,
    radius: T,
    limit_dot_product: T,
    expand: bool,
    rot_dir: T,
}

impl<T: FloatNumber> RoundJoinBuilder<T> {
    pub(super) fn new(ratio: T, radius: T) -> Self {
        // ratio = A / R
        let fixed_ratio = ratio.min(T::from_float(0.25 * PI));
        let limit_dot_product = fixed_ratio.cos();
        let average_count = (T::from_float(0.6 * PI) / fixed_ratio).to_usize() + 2;
        let (expand, rot_dir) = if radius >= T::from_float(0.0) {
            (true, T::from_float(-1.0))
        } else {
            (false, T::from_float(1.0))
        };

        Self {
            inv_ratio: T::from_float(1.0) / fixed_ratio,
            average_count,
            radius,
            limit_dot_product,
            expand,
            rot_dir,
        }
    }
}
impl<T: FloatNumber, P: FloatPointCompatible<T>> JoinBuilder<P, T> for RoundJoinBuilder<T> {
    fn add_join(
        &self,
        s0: &Section<P, T>,
        s1: &Section<P, T>,
        adapter: &FloatPointAdapter<P, T>,
        segments: &mut Vec<Segment<ShapeCountOffset>>,
    ) {
        let cross_product = FloatPointMath::cross_product(&s0.dir, &s1.dir);
        let turn = cross_product >= T::from_float(0.0);
        if turn == self.expand {
            BevelJoinBuilder::join_weak(s0, s1, adapter, segments);
            return
        }

        let dot_product = FloatPointMath::dot_product(&s0.dir, &s1.dir);
        if self.limit_dot_product < dot_product {
            BevelJoinBuilder::join_weak(s0, s1, adapter, segments);
            return;
        }

        let angle = dot_product.acos();
        let n = (angle * self.inv_ratio).to_usize();
        let delta_angle = angle / T::from_usize(n);

        let start = s0.b_top;
        let end = s1.a_top;

        let dir = P::from_xy(-s0.dir.y(), s0.dir.x());

        let rotator = Rotator::<T>::with_angle(self.rot_dir * delta_angle);

        let center = s0.b;
        let mut v = dir;
        let mut a = adapter.float_to_int(&start);
        for _ in 1..n {
            v = rotator.rotate(&v);
            let p = FloatPointMath::add(&center, &FloatPointMath::scale(&v, self.radius));

            let b = adapter.float_to_int(&p);
            if a != b {
                segments.push(Segment::bold_subject_ab(a, b));
                a = b;
            }
        }

        let b = adapter.float_to_int(&end);
        if a != b {
            segments.push(Segment::bold_subject_ab(a, b));
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