use crate::mesh::style::LineJoin;
use crate::mesh::variable_stroke::builder_cap::CapBuilder;
use crate::mesh::variable_stroke::builder_join::{
    BevelJoinBuilder, JoinBuilder, MiterJoinBuilder, RoundJoinBuilder,
};
use crate::mesh::variable_stroke::section::{Section, SectionToSegment};
use crate::mesh::variable_stroke::style::{StrokeVertex, VariableStrokeStyle};
use crate::segm::boolean::ShapeCountBoolean;
use crate::segm::segment::Segment;
use alloc::boxed::Box;
use alloc::vec::Vec;
use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;

trait VariableStrokeBuild<P: FloatPointCompatible> {
    fn build(
        &self,
        path: &[StrokeVertex<P>],
        is_closed_path: bool,
        adapter: &FloatPointAdapter<P>,
        segments: &mut Vec<Segment<ShapeCountBoolean>>,
    );

    fn capacity(&self, paths_count: usize, points_count: usize, is_closed_path: bool) -> usize;

    fn additional_offset(&self, max_radius: P::Scalar) -> P::Scalar;
}

pub(super) struct VariableStrokeBuilder<P: FloatPointCompatible> {
    builder: Box<dyn VariableStrokeBuild<P>>,
}

struct Builder<J: JoinBuilder<P>, P: FloatPointCompatible> {
    join_builder: J,
    start_cap_builder: CapBuilder<P>,
    end_cap_builder: CapBuilder<P>,
}

impl<P: FloatPointCompatible + 'static> VariableStrokeBuilder<P> {
    pub(super) fn new(style: VariableStrokeStyle<P>) -> VariableStrokeBuilder<P> {
        let start_cap_builder = CapBuilder::new(style.start_cap.normalize());
        let end_cap_builder = CapBuilder::new(style.end_cap.normalize());

        let builder: Box<dyn VariableStrokeBuild<P>> = match style.join.normalize() {
            LineJoin::Miter(ratio) => Box::new(Builder {
                join_builder: MiterJoinBuilder::new(ratio),
                start_cap_builder,
                end_cap_builder,
            }),
            LineJoin::Round(ratio) => Box::new(Builder {
                join_builder: RoundJoinBuilder::new(ratio),
                start_cap_builder,
                end_cap_builder,
            }),
            LineJoin::Bevel => Box::new(Builder {
                join_builder: BevelJoinBuilder,
                start_cap_builder,
                end_cap_builder,
            }),
        };

        Self { builder }
    }

    #[inline]
    pub(super) fn build(
        &self,
        path: &[StrokeVertex<P>],
        is_closed_path: bool,
        adapter: &FloatPointAdapter<P>,
        segments: &mut Vec<Segment<ShapeCountBoolean>>,
    ) {
        self.builder.build(path, is_closed_path, adapter, segments);
    }

    #[inline]
    pub(super) fn capacity(&self, paths_count: usize, points_count: usize, is_closed_path: bool) -> usize {
        self.builder.capacity(paths_count, points_count, is_closed_path)
    }

    #[inline]
    pub(super) fn additional_offset(&self, max_radius: P::Scalar) -> P::Scalar {
        self.builder.additional_offset(max_radius)
    }
}

impl<J: JoinBuilder<P>, P: FloatPointCompatible> VariableStrokeBuild<P> for Builder<J, P> {
    fn build(
        &self,
        path: &[StrokeVertex<P>],
        is_closed_path: bool,
        adapter: &FloatPointAdapter<P>,
        segments: &mut Vec<Segment<ShapeCountBoolean>>,
    ) {
        let path = Self::unique_path(path, is_closed_path, adapter);
        if is_closed_path {
            self.closed_segments(path.as_slice(), adapter, segments);
        } else {
            self.open_segments(path.as_slice(), adapter, segments);
        }
    }

    #[inline]
    fn capacity(&self, paths_count: usize, points_count: usize, is_closed_path: bool) -> usize {
        if is_closed_path {
            self.join_builder.capacity() * points_count.saturating_sub(1)
        } else {
            self.join_builder.capacity() * points_count.saturating_sub(1)
                + paths_count * (self.end_cap_builder.capacity() + self.start_cap_builder.capacity())
        }
    }

    #[inline]
    fn additional_offset(&self, max_radius: P::Scalar) -> P::Scalar {
        let start_cap = self.start_cap_builder.additional_offset(max_radius);
        let end_cap = self.end_cap_builder.additional_offset(max_radius);
        let join = self.join_builder.additional_offset(max_radius);
        join.max(start_cap.max(end_cap))
    }
}

impl<J: JoinBuilder<P>, P: FloatPointCompatible> Builder<J, P> {
    fn open_segments(
        &self,
        path: &[StrokeVertex<P>],
        adapter: &FloatPointAdapter<P>,
        segments: &mut Vec<Segment<ShapeCountBoolean>>,
    ) {
        let n = path.len();
        if n < 2 {
            return;
        }

        let mut s0 = Section::new(&path[0], &path[1]);
        self.start_cap_builder.add_to_start(&s0, adapter, segments);
        segments.add_section(&s0, adapter);

        for i in 2..n {
            let s1 = Section::new(&path[i - 1], &path[i]);
            self.join_builder.add_join(&s0, &s1, adapter, segments);
            segments.add_section(&s1, adapter);
            s0 = s1;
        }

        self.end_cap_builder.add_to_end(&s0, adapter, segments);
    }

    fn closed_segments(
        &self,
        path: &[StrokeVertex<P>],
        adapter: &FloatPointAdapter<P>,
        segments: &mut Vec<Segment<ShapeCountBoolean>>,
    ) {
        let n = path.len();
        if n < 2 {
            return;
        }

        let start = Section::new(&path[n - 1], &path[0]);
        let mut s0 = start.clone();
        segments.add_section(&s0, adapter);

        for i in 1..n {
            let s1 = Section::new(&path[i - 1], &path[i]);
            self.join_builder.add_join(&s0, &s1, adapter, segments);
            segments.add_section(&s1, adapter);
            s0 = s1;
        }

        self.join_builder.add_join(&s0, &start, adapter, segments);
    }

    fn unique_path(
        path: &[StrokeVertex<P>],
        is_closed_path: bool,
        adapter: &FloatPointAdapter<P>,
    ) -> Vec<StrokeVertex<P>> {
        let mut unique: Vec<StrokeVertex<P>> = Vec::with_capacity(path.len());

        for vertex in path {
            let ip = adapter.float_to_int(&vertex.point);
            if let Some(last) = unique.last_mut() {
                let last_ip = adapter.float_to_int(&last.point);
                if last_ip == ip {
                    *last = *vertex;
                    continue;
                }
            }
            unique.push(*vertex);
        }

        if is_closed_path && unique.len() > 1 {
            let first_ip = adapter.float_to_int(&unique[0].point);
            let last_index = unique.len() - 1;
            let last_ip = adapter.float_to_int(&unique[last_index].point);
            if first_ip == last_ip {
                unique[0] = unique[last_index];
                unique.pop();
            }
        }

        unique
    }
}
