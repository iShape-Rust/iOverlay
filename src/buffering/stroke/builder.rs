use crate::buffering::stroke::builder_cap::CapBuilder;
use crate::buffering::stroke::builder_join::{
    BevelJoinBuilder, JoinBuilder, MiterJoinBuilder, RoundJoinBuilder,
};
use crate::buffering::stroke::section::{Section, SectionToSegment};
use crate::buffering::stroke::style::{LineJoin, StrokeStyle};
use crate::segm::segment::Segment;
use crate::segm::winding_count::ShapeCountBoolean;
use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;

trait StrokeBuild<P: FloatPointCompatible<T>, T: FloatNumber> {
    fn build<'a>(
        &self,
        path: &[P],
        is_closed_path: bool,
        adapter: &FloatPointAdapter<P, T>,
        segments: &mut Vec<Segment<ShapeCountBoolean>>,
    );
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
        let radius = T::from_float(0.5) * style.width;

        let builder: Box<dyn StrokeBuild<P, T>> = match style.join {
            LineJoin::Miter(_) => Box::new(Builder {
                radius,
                join_builder: MiterJoinBuilder {},
                start_cap_builder: style.start_cap,
                end_cap_builder: style.end_cap,
            }),
            LineJoin::Round(_) => Box::new(Builder {
                radius,
                join_builder: RoundJoinBuilder {},
                start_cap_builder: style.start_cap,
                end_cap_builder: style.end_cap,
            }),
            LineJoin::Bevel => Box::new(Builder {
                radius,
                join_builder: BevelJoinBuilder {},
                start_cap_builder: style.start_cap,
                end_cap_builder: style.end_cap,
            }),
        };

        Self { builder }
    }

    pub(super) fn build(
        &self,
        path: &[P],
        is_closed_path: bool,
        adapter: &FloatPointAdapter<P, T>,
        segments: &mut Vec<Segment<ShapeCountBoolean>>,
    ) {
        self.builder.build(path, is_closed_path, adapter, segments);
    }
}

impl<J: JoinBuilder<P, T>, P: FloatPointCompatible<T>, T: FloatNumber> StrokeBuild<P, T>
    for Builder<J, P, T>
{
    fn build(
        &self,
        path: &[P],
        is_closed_path: bool,
        adapter: &FloatPointAdapter<P, T>,
        segments: &mut Vec<Segment<ShapeCountBoolean>>,
    ) {
        if is_closed_path {
            self.closed_segments(path, adapter, segments);
        } else {
            self.open_segments(path, adapter, segments);
        }
    }
}

impl<J: JoinBuilder<P, T>, P: FloatPointCompatible<T>, T: FloatNumber> Builder<J, P, T> {
    fn closed_segments<'a>(
        &self,
        path: &[P],
        adapter: &FloatPointAdapter<P, T>,
        segments: &mut Vec<Segment<ShapeCountBoolean>>,
    ) {
        let n = path.len();
        let start = Section::section(self.radius, &path[n - 1], &path[0]);

        let mut s0 = start.clone();
        segments.add_section(&s0, adapter);
        for b in path.iter() {
            let s1 = Section::section(self.radius, &s0.b, b);
            self.join_builder.add_join(&s0, &s1, adapter, segments);
            segments.add_section(&s1, adapter);
            s0 = s1;
        }

        self.join_builder.add_join(&s0, &start, adapter, segments);
    }

    fn open_segments<'a>(
        &self,
        path: &[P],
        adapter: &FloatPointAdapter<P, T>,
        segments: &mut Vec<Segment<ShapeCountBoolean>>,
    ) {
        let mut s0 = Section::section(self.radius, &path[0], &path[1]);

        self.start_cap_builder.add_to_start(&s0, adapter, segments);

        segments.add_section(&s0, adapter);

        for b in path.iter().skip(1) {
            let s1 = Section::section(self.radius, &s0.b, b);
            self.join_builder.add_join(&s0, &s1, adapter, segments);
            segments.add_section(&s1, adapter);
            s0 = s1;
        }

        self.end_cap_builder.add_to_end(&s0, adapter, segments);
    }
}
