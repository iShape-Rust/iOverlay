use crate::mesh::rotator::Rotator;
use crate::mesh::style::LineCap;
use crate::mesh::variable_stroke::section::Section;
use crate::segm::boolean::ShapeCountBoolean;
use crate::segm::segment::Segment;
use alloc::vec::Vec;
use core::f64::consts::PI;
use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_float::float::rect::FloatRect;
use i_float::float::vector::FloatPointMath;
use i_float::int::point::IntPoint;

#[derive(Clone)]
pub(super) struct CapBuilder<P: FloatPointCompatible> {
    cap: LineCap<P>,
}

impl<P: FloatPointCompatible> CapBuilder<P> {
    #[inline]
    pub(super) fn new(cap: LineCap<P>) -> Self {
        Self { cap }
    }

    pub(super) fn add_to_start(
        &self,
        section: &Section<P>,
        adapter: &FloatPointAdapter<P>,
        segments: &mut Vec<Segment<ShapeCountBoolean>>,
    ) {
        let mut a = adapter.float_to_int(&section.a_top);
        let dir = P::from_xy(-section.dir.x(), -section.dir.y());
        let rotator = Rotator::with_vector(&dir);
        self.add_points(section.a_radius, &rotator, &section.a, &mut a, adapter, segments);
        Self::add_segment(a, adapter.float_to_int(&section.a_bot), segments);
    }

    pub(super) fn add_to_end(
        &self,
        section: &Section<P>,
        adapter: &FloatPointAdapter<P>,
        segments: &mut Vec<Segment<ShapeCountBoolean>>,
    ) {
        let mut a = adapter.float_to_int(&section.b_bot);
        let rotator = Rotator::with_vector(&section.dir);
        self.add_points(section.b_radius, &rotator, &section.b, &mut a, adapter, segments);
        Self::add_segment(a, adapter.float_to_int(&section.b_top), segments);
    }

    fn add_points(
        &self,
        radius: P::Scalar,
        rotator: &Rotator<P::Scalar>,
        center: &P,
        a: &mut IntPoint,
        adapter: &FloatPointAdapter<P>,
        segments: &mut Vec<Segment<ShapeCountBoolean>>,
    ) {
        if radius <= P::Scalar::from_float(0.0) {
            return;
        }

        match &self.cap {
            LineCap::Butt => {}
            LineCap::Round(angle) => {
                let n = Self::round_count(*angle);
                let fixed_angle = P::Scalar::from_float(PI / n as f64);
                let round_rotator = Rotator::with_angle(fixed_angle);
                let mut v = P::from_xy(P::Scalar::from_float(0.0), -radius);
                for _ in 1..n {
                    v = round_rotator.rotate(&v);
                    self.add_local_point(&v, rotator, center, a, adapter, segments);
                }
            }
            LineCap::Square => {
                let p0 = P::from_xy(radius, -radius);
                self.add_local_point(&p0, rotator, center, a, adapter, segments);
                let p1 = P::from_xy(radius, radius);
                self.add_local_point(&p1, rotator, center, a, adapter, segments);
            }
            LineCap::Custom(points) => {
                for p in points.iter() {
                    let scaled = FloatPointMath::scale(p, radius);
                    self.add_local_point(&scaled, rotator, center, a, adapter, segments);
                }
            }
        }
    }

    #[inline]
    fn add_local_point(
        &self,
        p: &P,
        rotator: &Rotator<P::Scalar>,
        center: &P,
        a: &mut IntPoint,
        adapter: &FloatPointAdapter<P>,
        segments: &mut Vec<Segment<ShapeCountBoolean>>,
    ) {
        let r = rotator.rotate(p);
        let q = FloatPointMath::add(&r, center);
        let b = adapter.float_to_int(&q);
        Self::add_segment(*a, b, segments);
        *a = b;
    }

    #[inline]
    fn add_segment(a: IntPoint, b: IntPoint, segments: &mut Vec<Segment<ShapeCountBoolean>>) {
        if a != b {
            segments.push(Segment::subject(a, b));
        }
    }

    #[inline]
    pub(super) fn capacity(&self) -> usize {
        match &self.cap {
            LineCap::Butt => 1,
            LineCap::Round(angle) => Self::round_count(*angle),
            LineCap::Square => 3,
            LineCap::Custom(points) => 1 + points.len(),
        }
    }

    #[inline]
    pub(super) fn additional_offset(&self, radius: P::Scalar) -> P::Scalar {
        match &self.cap {
            LineCap::Butt => P::Scalar::from_float(0.0),
            LineCap::Round(_) | LineCap::Square => P::Scalar::from_float(2.0) * radius,
            LineCap::Custom(points) => {
                if let Some(rect) = FloatRect::with_iter(points.iter()) {
                    radius * (rect.width() + rect.height())
                } else {
                    P::Scalar::from_float(0.0)
                }
            }
        }
    }

    #[inline]
    fn round_count(angle: P::Scalar) -> usize {
        let angle_f64 = angle.to_f64();
        if angle_f64 > 0.0 {
            (PI / angle_f64) as usize
        } else {
            1024
        }
        .clamp(2, 1024)
    }
}
