use alloc::boxed::Box;
use alloc::vec::Vec;
use core::marker::PhantomData;
use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use crate::mesh::outline::builder_join::{JoinBuilder, BevelJoinBuilder, MiterJoinBuilder, RoundJoinBuilder};
use crate::mesh::outline::section::{Section, SectionToSegment};
use crate::mesh::style::LineJoin;
use crate::segm::offset::ShapeCountOffset;
use crate::segm::segment::Segment;

trait OutlineBuild<P: FloatPointCompatible<T>, T: FloatNumber> {
    fn build(
        &self,
        path: &[P],
        adapter: &FloatPointAdapter<P, T>,
        segments: &mut Vec<Segment<ShapeCountOffset>>,
    );

    fn capacity(&self, points_count: usize) -> usize;
    fn additional_offset(&self, radius: T) -> T;
}

pub(super) struct OutlineBuilder<P: FloatPointCompatible<T>, T: FloatNumber> {
    builder: Box<dyn OutlineBuild<P, T>>
}

struct Builder<J: JoinBuilder<P, T>, P: FloatPointCompatible<T>, T: FloatNumber> {
    radius: T,
    join_builder: J,
    _phantom: PhantomData<P>,
}

impl<P: FloatPointCompatible<T> + 'static, T: FloatNumber + 'static> OutlineBuilder<P, T> {
    pub(super) fn new(radius: T, join: &LineJoin<T>) -> OutlineBuilder<P, T> {

        let builder: Box<dyn OutlineBuild<P, T>> = match join {
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
        };

        Self { builder }
    }

    #[inline]
    pub(super) fn build(
        &self,
        path: &[P],
        adapter: &FloatPointAdapter<P, T>,
        segments: &mut Vec<Segment<ShapeCountOffset>>,
    ) {
        self.builder.build(path, adapter, segments);
    }

    #[inline]
    pub(super) fn capacity(
        &self,
        points_count: usize,
    ) -> usize {
        self.builder.capacity(points_count)
    }

    #[inline]
    pub(super) fn additional_offset(&self, radius: T) -> T {
        self.builder.additional_offset(radius)
    }
}

impl<J: JoinBuilder<P, T>, P: FloatPointCompatible<T>, T: FloatNumber> OutlineBuild<P, T> for Builder<J, P, T>
{
    #[inline]
    fn build(
        &self,
        path: &[P],
        adapter: &FloatPointAdapter<P, T>,
        segments: &mut Vec<Segment<ShapeCountOffset>>,
    ) {
        if path.len() < 2 { return; }

        // build segments only from points which are not equal in int space
        let i0 = path.len() - 1;
        let i1 = Self::next_unique_point(i0, 0, path, adapter);
        if i1 == usize::MAX { return }

        let start = Section::new(self.radius, &path[i0], &path[i1]);
        let mut s0 = start.clone();
        segments.add_section(&s0, adapter);

        let mut i = i1;
        i = Self::next_unique_point(i, i + 1, path, adapter);
        while i != usize::MAX {
            let si = Section::new(self.radius, &s0.b, &path[i]);
            self.join_builder.add_join(&s0, &si, adapter, segments);
            segments.add_section(&si, adapter);

            i = Self::next_unique_point(i, i + 1, path, adapter);
            s0 = si;
        }

        self.join_builder.add_join(&s0, &start, adapter, segments);
    }

    #[inline]
    fn capacity(&self, points_count: usize) -> usize {
        self.join_builder.capacity() * points_count
    }

    #[inline]
    fn additional_offset(&self, radius: T) -> T {
        self.join_builder.additional_offset(radius)
    }
}

impl<J: JoinBuilder<P, T>, P: FloatPointCompatible<T>, T: FloatNumber> Builder<J, P, T> {
    #[inline]
    fn next_unique_point(start: usize, index: usize, path: &[P], adapter: &FloatPointAdapter<P, T>) -> usize {
        let a = adapter.float_to_int(&path[start]);
        for (j, p) in path.iter().enumerate().skip(index) {
            let b = adapter.float_to_int(p);
            if a != b {
                return j;
            }
        }

        usize::MAX
    }
}