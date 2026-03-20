use crate::mesh::outline::builder_join::JoinBuilder;
use crate::mesh::outline::builder_join_expand::{
    ExpandBevelJoinBuilder, ExpandMiterJoinBuilder, ExpandRoundJoinBuilder,
};
use crate::mesh::outline::builder_join_shrink::{
    BevelJoinBuilder, MiterJoinBuilder, RoundJoinBuilder,
};
use crate::mesh::outline::section::{OffsetSection, Section, SectionToSegment};
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
use i_float::int::point::IntPoint;
use crate::mesh::math::Math;
use crate::mesh::outline::uniq_iter::{UniqueSegment, UniqueSegmentsIter};

trait OutlineBuild<P: FloatPointCompatible<T>, T: FloatNumber> {
    fn build(
        &self,
        path: &[P],
        adapter: &FloatPointAdapter<P, T>,
        segments: &mut Vec<Segment<ShapeCountBoolean>>,
    );

    fn capacity(&self, points_count: usize) -> usize;
    fn additional_offset(&self, radius: T) -> T;

    fn radius(&self) -> T;
}

pub(super) struct OutlineBuilder<P: FloatPointCompatible<T>, T: FloatNumber> {
    builder: Box<dyn OutlineBuild<P, T>>,
}

struct Builder<J: JoinBuilder<P, T>, P: FloatPointCompatible<T>, T: FloatNumber> {
    radius: T,
    join_builder: J,
    _phantom: PhantomData<P>,
}

impl<P: FloatPointCompatible<T> + 'static, T: FloatNumber + 'static> OutlineBuilder<P, T> {
    pub(super) fn new(radius: T, join: &LineJoin<T>) -> OutlineBuilder<P, T> {
        let builder: Box<dyn OutlineBuild<P, T>> = if radius.is_expand() {
            match join {
                LineJoin::Miter(ratio) => Box::new(Builder {
                    radius,
                    join_builder: ExpandMiterJoinBuilder::new(*ratio, radius),
                    _phantom: Default::default(),
                }),
                LineJoin::Round(ratio) => Box::new(Builder {
                    radius,
                    join_builder: ExpandRoundJoinBuilder::new(*ratio, radius),
                    _phantom: Default::default(),
                }),
                LineJoin::Bevel => Box::new(Builder {
                    radius,
                    join_builder: ExpandBevelJoinBuilder {},
                    _phantom: Default::default(),
                }),
            }
        } else {
            match join {
                LineJoin::Miter(ratio) => Box::new(Builder {
                    radius,
                    join_builder: MiterJoinBuilder::new(*ratio, radius),
                    _phantom: Default::default(),
                }),
                LineJoin::Round(ratio) => Box::new(Builder {
                    radius,
                    join_builder: RoundJoinBuilder::new(*ratio, radius),
                    _phantom: Default::default(),
                }),
                LineJoin::Bevel => Box::new(Builder {
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
        adapter: &FloatPointAdapter<P, T>,
        segments: &mut Vec<Segment<ShapeCountBoolean>>,
    ) {
        self.builder.build(path, adapter, segments);
    }

    #[inline]
    pub(super) fn capacity(&self, points_count: usize) -> usize {
        self.builder.capacity(points_count)
    }

    #[inline]
    pub(super) fn additional_offset(&self, radius: T) -> T {
        self.builder.additional_offset(radius)
    }

    #[inline]
    pub(super) fn is_shrink(&self) -> bool {
        self.builder.radius() > T::from_float(0.0)
    }
}

impl<J: JoinBuilder<P, T>, P: FloatPointCompatible<T>, T: FloatNumber> OutlineBuild<P, T>
    for Builder<J, P, T>
{
    #[inline]
    fn build(
        &self,
        path: &[P],
        adapter: &FloatPointAdapter<P, T>,
        segments: &mut Vec<Segment<ShapeCountBoolean>>,
    ) {
        if path.len() < 2 {
            return;
        }

        self.build(path, adapter, segments);

        // if self.radius.is_expand() {
        //     self.expand_build(path, adapter, segments);
        // } else {
        //     self.shrink_build(path, adapter, segments);
        // }
    }

    #[inline]
    fn capacity(&self, points_count: usize) -> usize {
        self.join_builder.capacity() * points_count
    }

    #[inline]
    fn additional_offset(&self, radius: T) -> T {
        self.join_builder.additional_offset(radius)
    }

    fn radius(&self) -> T {
        self.radius
    }
}

impl<J: JoinBuilder<P, T>, P: FloatPointCompatible<T>, T: FloatNumber> Builder<J, P, T> {
/*
    #[inline]
    fn next_unique_point(
        start: usize,
        index: usize,
        path: &[P],
        adapter: &FloatPointAdapter<P, T>,
    ) -> (usize, IntPoint) {
        let a = adapter.float_to_int(&path[start]);
        for (j, p) in path.iter().enumerate().skip(index) {
            let b = adapter.float_to_int(p);
            if a != b {
                return (j, b);
            }
        }

        (usize::MAX, IntPoint::EMPTY)
    }

    fn shrink_build(
        &self,
        path: &[P],
        adapter: &FloatPointAdapter<P, T>,
        segments: &mut Vec<Segment<ShapeCountBoolean>>,
    ) {
        // build segments only from points which are not equal in int space
        let i0 = path.len() - 1;
        let (i1, mut p1) = Self::next_unique_point(i0, 0, path, adapter);
        if i1 == usize::MAX {
            return;
        }

        let start = Section::new(self.radius, &path[i0], &path[i1]);
        let p0 = adapter.float_to_int(&path[i0]);

        let mut s0 = start.clone();
        segments.add_shrink_section(p0, p1, &s0, adapter);

        let (mut i, mut pi) = Self::next_unique_point(i1, i1 + 1, path, adapter);
        while i != usize::MAX {
            let si = Section::new(self.radius, &s0.b, &path[i]);
            segments.add_shrink_section(p1, pi, &si, adapter);

            (i, pi) = Self::next_unique_point(i, i + 1, path, adapter);
            s0 = si;
            p1 = pi;
        }

        self.join_builder.add_join(&s0, &start, adapter, segments);
    }
*/
    fn build(
        &self,
        path: &[P],
        adapter: &FloatPointAdapter<P, T>,
        segments: &mut Vec<Segment<ShapeCountBoolean>>,
    )  {
        let iter = path.iter().map(|p| adapter.float_to_int(p));
        let mut uniq_segments = if let Some(iter) = UniqueSegmentsIter::new(iter) {
            iter
        } else {
            // TODO impl single point
            return;
        };

        let us0 = if let Some(us) = uniq_segments.next() {
            us
        } else {
            // TODO impl single point
            return;
        };

        let s0 = OffsetSection::new(self.radius, &us0, adapter);
        let mut sk = s0;

        segments.push_some(sk.main_segment());

        for usi in uniq_segments {
            let si = OffsetSection::new(self.radius, &usi, adapter);
            segments.push_some(si.main_segment());

            let vi = si.b - si.a;
            let vp = sk.b - sk.a;

            let cross = vi.cross_product(vp);
            debug_assert!(cross != 0);
            if cross > 0 {
                // no join
                segments.push_some(sk.a_segment());
                segments.push_some(sk.b_segment());
            } else {
                self.join_builder.add_join(&s0, &si, adapter, segments);
            }

            // self.join_builder.add_join(&s0, &si, adapter, segments);
            // segments.add_expand_section(&si, adapter);
            sp = si;
        }
    }
}

trait Expand {
    fn is_expand(&self) -> bool;
}

impl<T: FloatNumber> Expand for T {
    fn is_expand(&self) -> bool {
        *self <= T::from_float(0.0)
    }
}

struct UniquePointsIter<'a, P, T>
where
    P: FloatPointCompatible<T>,
    T: FloatNumber,
{
    path: &'a [P],
    adapter: &'a FloatPointAdapter<P, T>,
    start_index: usize,
    next_index: usize,
    prev: IntPoint,
}
impl<'a, P, T> UniquePointsIter<'a, P, T>
where
    P: FloatPointCompatible<T>,
    T: FloatNumber,
{
    #[inline]
    fn new(path: &'a [P], adapter: &'a FloatPointAdapter<P, T>) -> Self {
        let (start_index, prev) = if path.is_empty() {
            (0, IntPoint::EMPTY)
        } else {
            let start_index = path.len().saturating_sub(1);
            let prev = adapter.float_to_int(&path[start_index]);
            (start_index, prev)
        };

        Self {
            path,
            adapter,
            start_index,
            next_index: 0,
            prev,
        }
    }
}

impl<'a, P, T> Iterator for UniquePointsIter<'a, P, T>
where
    P: FloatPointCompatible<T>,
    T: FloatNumber,
{
    type Item = (usize, IntPoint);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.path.len() < 2 {
            return None;
        }

        for (j, p) in self.path.iter().enumerate().skip(self.next_index) {
            let b = self.adapter.float_to_int(p);
            if self.prev != b {
                self.prev = b;
                self.next_index = j + 1;
                return Some((j, b));
            }
        }

        None
    }
}

impl<P: FloatPointCompatible<T>, T: FloatNumber> OffsetSection<P, T> {
    #[inline]
    fn new(radius: T, s: &UniqueSegment, adapter: &FloatPointAdapter<P, T>) -> Self
    where P: FloatPointCompatible<T>, T: FloatNumber,
    {
        let a = adapter.int_to_float(&s.a);
        let b = adapter.int_to_float(&s.b);
        let ab = FloatPointMath::sub(&b, &a);
        let dir = FloatPointMath::normalize(&ab);
        let ft = Math::ortho_and_scale(&dir, radius);

        let t = adapter.float_to_int(&ft);

        let a_top = s.a + t;
        let b_top = s.b + t;

        Self {
            a: s.a,
            b: s.b,
            a_top,
            b_top,
            dir,
            _phantom: Default::default(),
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