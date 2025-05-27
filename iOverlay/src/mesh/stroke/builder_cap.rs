use alloc::vec;
use alloc::vec::Vec;
use core::f64::consts::PI;
use core::marker::PhantomData;
use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_float::float::rect::FloatRect;
use i_float::float::vector::FloatPointMath;
use crate::mesh::stroke::section::Section;
use crate::mesh::style::LineCap;
use crate::mesh::rotator::Rotator;
use crate::segm::offset::ShapeCountOffset;
use crate::segm::segment::Segment;

#[derive(Debug, Clone)]
pub(super) struct CapBuilder<P, T> {
    points: Option<Vec<P>>,
    _phantom: PhantomData<T>,
}

impl<T: FloatNumber, P: FloatPointCompatible<T>> CapBuilder<P, T> {

    pub(super) fn new(cap: LineCap<P, T>, radius: T) -> Self {
        let points = match cap {
            LineCap::Butt => None,
            LineCap::Round(ratio) => Some(Self::round_points(ratio, radius)),
            LineCap::Square => Some(Self::square_points(radius)),
            LineCap::Custom(points) => Some(Self::custom_points(points, radius))
        };

        Self { points, _phantom: Default::default() }
    }

    pub(super) fn round_points(angle: T, r: T) -> Vec<P> {
        let angle_f64 = angle.to_f64();
        let n = if angle_f64 > 0.0 {
            let count = PI / angle_f64;
            (count as usize).clamp(2, 1024)
        } else {
            1024
        };

        let fix_angle = T::from_float(PI / n as f64);
        let rotator = Rotator::with_angle(fix_angle);
        let mut v = P::from_xy(T::from_float(0.0), T::from_float(-1.0));
        let mut points = Vec::with_capacity(n);
        for _ in 1..n {
            v = rotator.rotate(&v);
            let p = FloatPointMath::scale(&v, r);
            points.push(p);
        }

        points
    }

    pub(super) fn square_points(r: T) -> Vec<P> {
        vec![P::from_xy(r, -r), P::from_xy(r, r)]
    }

    pub(super) fn custom_points(points: Vec<P>, r: T) -> Vec<P> {
        let mut scaled = points;
        let mut i = 0;
        while i < scaled.len() {
            let p = &scaled[i];
            scaled[i] = FloatPointMath::scale(p, r);
            i += 1
        }
        scaled
    }

    pub(super) fn add_to_start(&self, section: &Section<P, T>, adapter: &FloatPointAdapter<P, T>, segments: &mut Vec<Segment<ShapeCountOffset>>) {
        let mut a = adapter.float_to_int(&section.a_top);
        if let Some(points) = &self.points {
            let dir = P::from_xy(-section.dir.x(), -section.dir.y());
            let rotator = Rotator::with_vector(&dir);
            for p in points.iter() {
                let r = rotator.rotate(p);
                let q = FloatPointMath::add(&r, &section.a);
                let b = adapter.float_to_int(&q);
                segments.push(Segment::bold_subject_ab(a, b));
                a = b;
            }
        }
        let last = adapter.float_to_int(&section.a_bot);
        segments.push(Segment::bold_subject_ab(a, last));
    }

    pub(super) fn add_to_end(&self, section: &Section<P, T>, adapter: &FloatPointAdapter<P, T>, segments: &mut Vec<Segment<ShapeCountOffset>>) {
        let mut a = adapter.float_to_int(&section.b_bot);
        if let Some(points) = &self.points {
            let rotator = Rotator::with_vector(&section.dir);
            for p in points.iter() {
                let r = rotator.rotate(p);
                let q = FloatPointMath::add(&r, &section.b);
                let b = adapter.float_to_int(&q);
                segments.push(Segment::bold_subject_ab(a, b));
                a = b;
            }
        }
        let last = adapter.float_to_int(&section.b_top);
        segments.push(Segment::bold_subject_ab(a, last));
    }

    #[inline]
    pub(super) fn capacity(&self) -> usize {
        if let Some(points) = &self.points {
            1 + points.len()
        } else {
            1
        }
    }

    #[inline]
    pub(super) fn additional_offset(&self) -> T {
        if let Some(points) = &self.points {
            if let Some(rect) = FloatRect::with_iter(points.iter()) {
                rect.width() + rect.height()
            } else {
                T::from_float(0.0)
            }
        } else {
            T::from_float(0.0)
        }
    }
}