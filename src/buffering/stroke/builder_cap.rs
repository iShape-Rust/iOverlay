use std::marker::PhantomData;
use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use crate::buffering::stroke::section::Section;
use crate::buffering::stroke::style::LineCap;
use crate::segm::segment::Segment;
use crate::segm::winding_count::ShapeCountBoolean;

#[derive(Debug, Clone)]
pub(super) struct CapBuilder<P, T> {
    points: Option<Vec<P>>,
    _phantom: PhantomData<T>,
}

impl<T: FloatNumber, P: FloatPointCompatible<T>> CapBuilder<P, T> {

    pub(super) fn butt() -> Self {
        Self { points: None, _phantom: Default::default() }
    }

    pub(super) fn new(cap: LineCap<P, T>) -> Self {
        let points = match cap {
            LineCap::Butt => None,
            LineCap::Round(ratio) => Some(Self::round_points(ratio)),
            LineCap::Square => Some(Self::square_points()),
            LineCap::Custom(points) => Some(points)
        };

        Self { points, _phantom: Default::default() }
    }

    pub(super) fn round_points(ratio: T) -> Vec<P> {
        Vec::new()
    }

    pub(super) fn square_points() -> Vec<P> {
        let r = T::from_float(1.0);
        vec![P::from_xy(r, r), P::from_xy(r, -r)]
    }

    pub(super) fn add_to_start(&self, section: &Section<P, T>, adapter: &FloatPointAdapter<P, T>, segments: &mut Vec<Segment<ShapeCountBoolean>>) {
        if let Some(points) = &self.points {

        } else {
            let a = adapter.float_to_int(&section.a_top);
            let b = adapter.float_to_int(&section.a_bot);
            segments.push(Segment::subject_ab(b, a));
        }
    }

    pub(super) fn add_to_end(&self, section: &Section<P, T>, adapter: &FloatPointAdapter<P, T>, segments: &mut Vec<Segment<ShapeCountBoolean>>) {
        if let Some(points) = &self.points {

        } else {
            let a = adapter.float_to_int(&section.b_top);
            let b = adapter.float_to_int(&section.b_bot);
            segments.push(Segment::subject_ab(a, b));
        }
    }

    pub(super) fn capacity(&self) -> usize {
        if let Some(points) = &self.points {
            1 + points.len()
        } else {
            1
        }
    }
}
/*

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