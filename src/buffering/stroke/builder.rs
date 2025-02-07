use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use crate::buffering::stroke::builder_round::RoundBuilder;
use crate::buffering::stroke::section::Section;
use crate::buffering::stroke::style::{LineJoin, StrokeStyle};
use crate::segm::segment::Segment;
use crate::segm::winding_count::ShapeCountBoolean;

pub(super) struct StrokeBuilder<T, P, C, J> {
    cap_builder: C,
    join_builder: J
}

impl<T: FloatNumber, P: FloatPointCompatible<T>> StrokeBuilder<T, P, C, J> {

    fn new(style: StrokeStyle<T>) -> StrokeBuilder<T, P, C, J> {
        match style.join {
            LineJoin::Miter => {

            }
            LineJoin::Round => {

            }
            LineJoin::Bevel => {

            }
        }
    }

}



pub(super) trait SectionBuilder<T, P> {
    fn add_section(&mut self, section: &Section<T, P>, adapter: &FloatPointAdapter<P, T>);
    fn add_bevel_join(&mut self, s0: &Section<T, P>, s1: &Section<T, P>, adapter: &FloatPointAdapter<P, T>);
    fn add_round_join(&mut self, s0: &Section<T, P>, s1: &Section<T, P>, adapter: &FloatPointAdapter<P, T>, round_builder: &RoundBuilder<T, P>);
    fn add_start_bevel_cap(&mut self, section: &Section<T, P>, adapter: &FloatPointAdapter<P, T>);
    fn add_end_bevel_cap(&mut self, section: &Section<T, P>, adapter: &FloatPointAdapter<P, T>);
    fn add_start_round_cap(&mut self, section: &Section<T, P>, adapter: &FloatPointAdapter<P, T>, round_builder: &RoundBuilder<T, P>);
    fn add_end_round_cap(&mut self, section: &Section<T, P>, adapter: &FloatPointAdapter<P, T>, round_builder: &RoundBuilder<T, P>);
}

/*
impl<T: FloatNumber, P: FloatPointCompatible<T>> SectionBuilder<T, P> for Vec<Segment<ShapeCountBoolean>> {
    fn add_section(&mut self, section: &Section<T, P>, adapter: &FloatPointAdapter<P, T>) {
        let a_top = adapter.float_to_int(section.a_top);
        let b_top = adapter.float_to_int(section.b_top);
        let a_bot = adapter.float_to_int(section.a_bot);
        let b_bot = adapter.float_to_int(section.b_bot);

        self.push(Segment::subject_ab(a_top, b_top));
        self.push(Segment::subject_ab(b_bot, a_bot));
    }

    fn add_bevel_join(&mut self, s0: &Section<T, P>, s1: &Section<T, P>, adapter: &FloatPointAdapter<P, T>) {
        if s0.b_top != s1.a_top {
            let a = adapter.float_to_int(s0.b_top);
            let b = adapter.float_to_int(s1.a_top);
            self.push(Segment::subject_ab(a, b));
        }

        if s0.b_bot != s1.a_bot {
            let a = adapter.float_to_int(s0.b_bot);
            let b = adapter.float_to_int(s1.a_bot);
            self.push(Segment::subject_ab(b, a));
        }
    }

    fn add_round_join(&mut self, s0: &Section<T, P>, s1: &Section<T, P>, adapter: &FloatPointAdapter<P, T>, round_builder: &RoundBuilder<T, P>) {

    }

    fn add_start_bevel_cap(&mut self, section: &Section<T, P>, adapter: &FloatPointAdapter<P, T>) {
        let a = adapter.float_to_int(section.a_top);
        let b = adapter.float_to_int(section.a_bot);
        self.push(Segment::subject_ab(b, a));
    }

    fn add_end_bevel_cap(&mut self, section: &Section<T, P>, adapter: &FloatPointAdapter<P, T>) {
        let a = adapter.float_to_int(section.b_top);
        let b = adapter.float_to_int(section.b_bot);
        self.push(Segment::subject_ab(a, b));
    }

    fn add_start_round_cap(&mut self, section: &Section<T, P>, adapter: &FloatPointAdapter<P, T>, round_builder: &RoundBuilder<T, P>) {

    }

    fn add_end_round_cap(&mut self, section: &Section<T, P>, adapter: &FloatPointAdapter<P, T>, round_builder: &RoundBuilder<T, P>) {

    }
}
*/