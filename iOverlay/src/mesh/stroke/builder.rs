use alloc::boxed::Box;
use alloc::vec::Vec;
use crate::mesh::stroke::builder_cap::CapBuilder;
use crate::mesh::stroke::builder_join::{
    BevelJoinBuilder, JoinBuilder, MiterJoinBuilder, RoundJoinBuilder,
};
use crate::mesh::stroke::section::{Section, SectionToSegment};
use crate::mesh::style::{LineJoin, StrokeStyle};
use crate::segm::segment::Segment;
use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use crate::segm::offset::ShapeCountOffset;

trait StrokeBuild<P: FloatPointCompatible<T>, T: FloatNumber> {
    fn build(
        &self,
        path: &[P],
        is_closed_path: bool,
        adapter: &FloatPointAdapter<P, T>,
        segments: &mut Vec<Segment<ShapeCountOffset>>,
    );

    fn capacity(&self, paths_count: usize, points_count: usize, is_closed_path: bool) -> usize;
    fn additional_offset(&self, radius: T) -> T;
}

pub(super) struct StrokeBuilder<P: FloatPointCompatible<T>, T: FloatNumber> {
    builder: Box<dyn StrokeBuild<P, T>>,
}

struct Builder<J: JoinBuilder<P, T>, P: FloatPointCompatible<T>, T: FloatNumber> {
    radius: T,
    join_builder: J,
    start_cap_builder: CapBuilder<P, T>,
    end_cap_builder: CapBuilder<P, T>,
}

impl<P: FloatPointCompatible<T> + 'static, T: FloatNumber + 'static> StrokeBuilder<P, T> {
    pub(super) fn new(style: StrokeStyle<P, T>) -> StrokeBuilder<P, T> {
        let radius = T::from_float(0.5 * style.width.to_f64().max(0.0));

        let start_cap_builder = CapBuilder::new(style.start_cap.normalize(), radius);
        let end_cap_builder = CapBuilder::new(style.end_cap.normalize(), radius);

        let builder: Box<dyn StrokeBuild<P, T>> = match style.join.normalize() {
            LineJoin::Miter(ratio) => Box::new(Builder {
                radius,
                join_builder: MiterJoinBuilder::new(ratio, radius),
                start_cap_builder,
                end_cap_builder,
            }),
            LineJoin::Round(ratio) => Box::new(Builder {
                radius,
                join_builder: RoundJoinBuilder::new(ratio, radius),
                start_cap_builder,
                end_cap_builder,
            }),
            LineJoin::Bevel => Box::new(Builder {
                radius,
                join_builder: BevelJoinBuilder {},
                start_cap_builder,
                end_cap_builder,
            }),
        };

        Self { builder }
    }

    #[inline]
    pub(super) fn build(
        &self,
        path: &[P],
        is_closed_path: bool,
        adapter: &FloatPointAdapter<P, T>,
        segments: &mut Vec<Segment<ShapeCountOffset>>,
    ) {
        self.builder.build(path, is_closed_path, adapter, segments);
    }

    #[inline]
    pub(super) fn capacity(
        &self,
        paths_count: usize,
        points_count: usize,
        is_closed_path: bool,
    ) -> usize {
        self.builder.capacity(paths_count, points_count, is_closed_path)
    }

    #[inline]
    pub(super) fn additional_offset(&self, radius: T) -> T {
        self.builder.additional_offset(radius)
    }
}

impl<J: JoinBuilder<P, T>, P: FloatPointCompatible<T>, T: FloatNumber> StrokeBuild<P, T> for Builder<J, P, T>
{
    #[inline]
    fn build(
        &self,
        path: &[P],
        is_closed_path: bool,
        adapter: &FloatPointAdapter<P, T>,
        segments: &mut Vec<Segment<ShapeCountOffset>>,
    ) {
        if is_closed_path {
            self.closed_segments(path, adapter, segments);
        } else {
            self.open_segments(path, adapter, segments);
        }
    }

    #[inline]
    fn capacity(&self, paths_count: usize, points_count: usize, is_closed_path: bool) -> usize {
        if is_closed_path {
            self.join_builder.capacity() * points_count - 2
        } else {
            self.join_builder.capacity() * (points_count.saturating_sub(1))
                + paths_count
                    * (self.end_cap_builder.capacity() + self.start_cap_builder.capacity())
        }
    }

    #[inline]
    fn additional_offset(&self, radius: T) -> T {
        let start_cap = self.start_cap_builder.additional_offset();
        let end_cap = self.end_cap_builder.additional_offset();
        let join = self.join_builder.additional_offset(radius);
        join.max(start_cap.max(end_cap))
    }
}

impl<J: JoinBuilder<P, T>, P: FloatPointCompatible<T>, T: FloatNumber> Builder<J, P, T> {
    fn open_segments(
        &self,
        path: &[P],
        adapter: &FloatPointAdapter<P, T>,
        segments: &mut Vec<Segment<ShapeCountOffset>>,
    ) {
        // build segments only from points which are not equal in int space

        let n = path.len();
        if n < 2 {
            return
        }

        let mut ip0 = adapter.float_to_int(&path[0]);
        let mut ip = adapter.float_to_int(&path[1]);
        let mut j = 1;
        while ip == ip0 {
            j += 1;
            if j >= n { return; }
            ip = adapter.float_to_int(&path[j]);
        }

        let mut s0 = Section::new(self.radius, &path[0], &path[j]);

        self.start_cap_builder.add_to_start(&s0, adapter, segments);

        segments.add_section(&s0, adapter);

        ip0 = ip;
        j += 1;
        'main_loop:
        while j < n {
            let mut p = &path[j];
            ip = adapter.float_to_int(p);
            while ip == ip0 {
                j += 1;
                if j >= n { break 'main_loop; }
                p = &path[j];
                ip = adapter.float_to_int(p);
            }
            let s1 = Section::new(self.radius, &s0.b, p);
            self.join_builder.add_join(&s0, &s1, adapter, segments);
            segments.add_section(&s1, adapter);
            s0 = s1;
            ip0 = ip;
        }

        self.end_cap_builder.add_to_end(&s0, adapter, segments);
    }

    fn closed_segments(
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