use crate::mesh::math::Math;
use crate::mesh::outline::builder_join::JoinBuilder;
use crate::mesh::outline::builder_join::{BevelJoinBuilder, MiterJoinBuilder, RoundJoinBuilder};
use crate::mesh::outline::section::OffsetSection;
use crate::mesh::outline::uniq_iter::{UniqueSegment, UniqueSegmentsIter};
use crate::mesh::style::LineJoin;
use crate::segm::boolean::ShapeCountBoolean;
use crate::segm::segment::Segment;
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::marker::PhantomData;
use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_float::float::vector::FloatPointMath;

trait OutlineBuild<P: FloatPointCompatible> {
    fn build(
        &self,
        path: &[P],
        adapter: &FloatPointAdapter<P, i32>,
        segments: &mut Vec<Segment<ShapeCountBoolean>>,
    );

    fn capacity(&self, points_count: usize) -> usize;
    fn additional_offset(&self, radius: P::Scalar) -> P::Scalar;
}

pub(super) struct OutlineBuilder<P: FloatPointCompatible> {
    builder: Box<dyn OutlineBuild<P>>,
}

struct Builder<J: JoinBuilder<P>, P: FloatPointCompatible> {
    extend: bool,
    radius: P::Scalar,
    join_builder: J,
    _phantom: PhantomData<P>,
}

impl<P: FloatPointCompatible + 'static> OutlineBuilder<P> {
    pub(super) fn new(radius: P::Scalar, join: &LineJoin<P::Scalar>) -> OutlineBuilder<P> {
        let extend = radius > P::Scalar::from_float(0.0);
        let builder: Box<dyn OutlineBuild<P>> = {
            match join {
                LineJoin::Miter(ratio) => Box::new(Builder {
                    extend,
                    radius,
                    join_builder: MiterJoinBuilder::new(*ratio, radius),
                    _phantom: Default::default(),
                }),
                LineJoin::Round(ratio) => Box::new(Builder {
                    extend,
                    radius,
                    join_builder: RoundJoinBuilder::new(*ratio, radius),
                    _phantom: Default::default(),
                }),
                LineJoin::Bevel => Box::new(Builder {
                    extend,
                    radius,
                    join_builder: BevelJoinBuilder {},
                    _phantom: Default::default(),
                }),
            }
        };

        Self { builder }
    }

    #[inline]
    pub(super) fn build(
        &self,
        path: &[P],
        adapter: &FloatPointAdapter<P, i32>,
        segments: &mut Vec<Segment<ShapeCountBoolean>>,
    ) {
        self.builder.build(path, adapter, segments);
    }

    #[inline]
    pub(super) fn capacity(&self, points_count: usize) -> usize {
        self.builder.capacity(points_count)
    }

    #[inline]
    pub(super) fn additional_offset(&self, radius: P::Scalar) -> P::Scalar {
        self.builder.additional_offset(radius)
    }
}

impl<J: JoinBuilder<P>, P: FloatPointCompatible> OutlineBuild<P> for Builder<J, P> {
    #[inline]
    fn build(
        &self,
        path: &[P],
        adapter: &FloatPointAdapter<P, i32>,
        segments: &mut Vec<Segment<ShapeCountBoolean>>,
    ) {
        if path.len() < 2 {
            return;
        }

        self.build(path, adapter, segments);
    }

    #[inline]
    fn capacity(&self, points_count: usize) -> usize {
        self.join_builder.capacity() * points_count
    }

    #[inline]
    fn additional_offset(&self, radius: P::Scalar) -> P::Scalar {
        self.join_builder.additional_offset(radius)
    }
}

impl<J: JoinBuilder<P>, P: FloatPointCompatible> Builder<J, P> {
    fn build(
        &self,
        path: &[P],
        adapter: &FloatPointAdapter<P, i32>,
        segments: &mut Vec<Segment<ShapeCountBoolean>>,
    ) {
        let iter = path.iter().map(|p| adapter.float_to_int(p));
        let mut uniq_segments = if let Some(iter) = UniqueSegmentsIter::new(iter) {
            iter
        } else {
            return;
        };

        let us0 = if let Some(us) = uniq_segments.next() {
            us
        } else {
            return;
        };

        let s0 = OffsetSection::new(self.radius, &us0, adapter);
        let mut sk = s0.clone();

        segments.push_some(sk.top_segment());

        for usi in uniq_segments {
            let si = OffsetSection::new(self.radius, &usi, adapter);
            segments.push_some(si.top_segment());
            self.feed_join(&sk, &si, adapter, segments);
            sk = si;
        }
        self.feed_join(&sk, &s0, adapter, segments);
    }

    #[inline]
    fn feed_join(
        &self,
        s0: &OffsetSection<P>,
        s1: &OffsetSection<P>,
        adapter: &FloatPointAdapter<P, i32>,
        segments: &mut Vec<Segment<ShapeCountBoolean>>,
    ) {
        let vi = s1.b - s1.a;
        let vp = s0.b - s0.a;

        let cross = vi.cross_product(vp);
        debug_assert!(cross != 0, "not possible! UniqueSegmentsIter guarantee it");
        let outer_corner = (cross > 0) == self.extend;
        if outer_corner {
            if s0.b_top != s1.a_top {
                self.join_builder.add_join(s0, s1, adapter, segments);
            }
        } else {
            // no join
            segments.push_some(s0.b_segment());
            segments.push_some(s1.a_segment());
        }
    }
}

impl<P: FloatPointCompatible> OffsetSection<P> {
    #[inline]
    fn new(radius: P::Scalar, s: &UniqueSegment, adapter: &FloatPointAdapter<P, i32>) -> Self {
        let a = adapter.int_to_float(&s.a);
        let b = adapter.int_to_float(&s.b);
        let ab = FloatPointMath::sub(&b, &a);
        let dir = FloatPointMath::normalize(&ab);
        let t = Math::ortho_and_scale(&dir, radius);

        let at = FloatPointMath::add(&a, &t);
        let bt = FloatPointMath::add(&b, &t);
        let a_top = adapter.float_to_int(&at);
        let b_top = adapter.float_to_int(&bt);

        Self {
            a: s.a,
            b: s.b,
            a_top,
            b_top,
            dir,
        }
    }
}

trait VecPushSome<T> {
    fn push_some(&mut self, value: Option<T>);
}

impl<T> VecPushSome<T> for Vec<T> {
    #[inline]
    fn push_some(&mut self, value: Option<T>) {
        if let Some(v) = value {
            self.push(v);
        }
    }
}
