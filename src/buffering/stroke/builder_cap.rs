use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use crate::buffering::stroke::section::Section;
use crate::segm::segment::Segment;
use crate::segm::winding_count::ShapeCountBoolean;

pub(super) trait CapBuilder<P, T> {
    fn add_start_cap(&self, section: &Section<T, P>, adapter: &FloatPointAdapter<P, T>, segments: &mut Vec<Segment<ShapeCountBoolean>>);
    fn add_end_cap(&self, section: &Section<T, P>, adapter: &FloatPointAdapter<P, T>, segments: &mut Vec<Segment<ShapeCountBoolean>>);
}

pub(super) struct ButtCapBuilder;

impl<T: FloatNumber, P: FloatPointCompatible<T>> CapBuilder<P, T> for ButtCapBuilder {
    fn add_start_cap(&self, section: &Section<T, P>, adapter: &FloatPointAdapter<P, T>, segments: &mut Vec<Segment<ShapeCountBoolean>>) {
        let a = adapter.float_to_int(section.a_top);
        let b = adapter.float_to_int(section.a_bot);
        segments.push(Segment::subject_ab(b, a));
    }

    fn add_end_cap(&self, section: &Section<T, P>, adapter: &FloatPointAdapter<P, T>, segments: &mut Vec<Segment<ShapeCountBoolean>>){
        let a = adapter.float_to_int(section.b_top);
        let b = adapter.float_to_int(section.b_bot);
        segments.push(Segment::subject_ab(a, b));
    }
}

pub(super) struct SquareCapBuilder;

impl<T: FloatNumber, P: FloatPointCompatible<T>> CapBuilder<P, T> for SquareCapBuilder {
    fn add_start_cap(&self, section: &Section<T, P>, adapter: &FloatPointAdapter<P, T>, segments: &mut Vec<Segment<ShapeCountBoolean>>) {
        let a = adapter.float_to_int(section.a_top);
        let b = adapter.float_to_int(section.a_bot);
        segments.push(Segment::subject_ab(b, a));
    }

    fn add_end_cap(&self, section: &Section<T, P>, adapter: &FloatPointAdapter<P, T>, segments: &mut Vec<Segment<ShapeCountBoolean>>){
        let a = adapter.float_to_int(section.b_top);
        let b = adapter.float_to_int(section.b_bot);
        segments.push(Segment::subject_ab(a, b));
    }
}

pub(super) struct RoundCapBuilder;

impl<T: FloatNumber, P: FloatPointCompatible<T>> CapBuilder<P, T> for RoundCapBuilder {
    fn add_start_cap(&self, section: &Section<T, P>, adapter: &FloatPointAdapter<P, T>, segments: &mut Vec<Segment<ShapeCountBoolean>>) {
        let a = adapter.float_to_int(section.a_top);
        let b = adapter.float_to_int(section.a_bot);
        segments.push(Segment::subject_ab(b, a));
    }

    fn add_end_cap(&self, section: &Section<T, P>, adapter: &FloatPointAdapter<P, T>, segments: &mut Vec<Segment<ShapeCountBoolean>>){
        let a = adapter.float_to_int(section.b_top);
        let b = adapter.float_to_int(section.b_bot);
        segments.push(Segment::subject_ab(a, b));
    }
}

/*
pub(crate) struct CapBuilder<T, P> {
    pub(super) count_for_pi: usize,
    pub(super) rotator: Rotator<T, P>,
}

impl<T: FloatNumber, P: FloatPointCompatible<T>> CapBuilder<T, P> {
    pub(super) fn new(style: StrokeStyle<T>) -> Option<Self> {
        if style.begin_cap == LineCap::Round || style.end_cap == LineCap::Round {
            return None
        }

        let count_for_pi = ((style.round_limit / style.width).to_f64().round() as usize).min(2);
        let delta_angle = PI / count_for_pi as f64;
        let (sn, cs) = delta_angle.to_f64().sin_cos();
        let sin = T::from_float(sn);
        let cos = T::from_float(cs);
        let rotator = Rotator::new(sin, cos);

        Some(Self {
            min_dot_product: cos,
            count_for_pi,
            rotator,
        })
    }
}
*/