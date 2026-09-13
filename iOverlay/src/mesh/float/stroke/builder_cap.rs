use crate::mesh::float::rotator::Rotator;
use crate::mesh::float::stroke::section::Section;
use crate::mesh::float::style::LineCap;
use crate::segm::boolean::ShapeCountBoolean;
use crate::segm::segment::Segment;
use alloc::vec;
use alloc::vec::Vec;
use core::f64::consts::PI;
use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_float::float::vector::FloatPointMath;
use i_float::int::number::int::IntNumber;

pub(super) struct CapBuilder<P> {
    points: Option<Vec<P>>,
}

impl<P: FloatPointCompatible> CapBuilder<P> {
    pub(super) fn new(cap: LineCap<P>, radius: P::Scalar) -> Self {
        let points = match cap {
            LineCap::Butt => None,
            LineCap::Round(ratio) => Some(Self::round_points(ratio, radius)),
            LineCap::Square => Some(Self::square_points(radius)),
            LineCap::Custom(points) => Some(Self::custom_points(points.to_vec(), radius)),
        };

        Self { points }
    }

    pub(super) fn round_points(angle: P::Scalar, r: P::Scalar) -> Vec<P> {
        let angle_f64 = angle.to_f64();
        let n = if angle_f64 > 0.0 {
            let count = PI / angle_f64;
            (count as usize).clamp(2, 1024)
        } else {
            1024
        };

        let fix_angle = P::Scalar::from_float(PI / n as f64);
        let rotator = Rotator::with_angle(fix_angle);
        let mut v = P::from_xy(P::Scalar::from_float(0.0), P::Scalar::from_float(-1.0));
        let mut points = Vec::with_capacity(n);
        for _ in 1..n {
            v = rotator.rotate(&v);
            let p = FloatPointMath::scale(&v, r);
            points.push(p);
        }

        points
    }

    pub(super) fn square_points(r: P::Scalar) -> Vec<P> {
        vec![P::from_xy(r, -r), P::from_xy(r, r)]
    }

    pub(super) fn custom_points(points: Vec<P>, r: P::Scalar) -> Vec<P> {
        let mut scaled = points;
        let mut i = 0;
        while i < scaled.len() {
            let p = &scaled[i];
            scaled[i] = FloatPointMath::scale(p, r);
            i += 1
        }
        scaled
    }

    pub(super) fn add_to_start<I: IntNumber>(
        &self,
        section: &Section<P>,
        adapter: &FloatPointAdapter<P, I>,
        segments: &mut Vec<Segment<ShapeCountBoolean, I>>,
    ) {
        let mut a = adapter.float_to_int(&section.a_top);
        if let Some(points) = &self.points {
            let dir = P::from_xy(-section.dir.x(), -section.dir.y());
            let rotator = Rotator::with_vector(&dir);
            for p in points.iter() {
                let r = rotator.rotate(p);
                let q = FloatPointMath::add(&r, &section.a);
                let b = adapter.float_to_int(&q);
                // Repeated template points and rounding can collapse a cap edge.
                if a != b {
                    segments.push(Segment::subject(a, b));
                }
                a = b;
            }
        }
        let last = adapter.float_to_int(&section.a_bot);
        if a != last {
            segments.push(Segment::subject(a, last));
        }
    }

    pub(super) fn add_to_end<I: IntNumber>(
        &self,
        section: &Section<P>,
        adapter: &FloatPointAdapter<P, I>,
        segments: &mut Vec<Segment<ShapeCountBoolean, I>>,
    ) {
        let mut a = adapter.float_to_int(&section.b_bot);
        if let Some(points) = &self.points {
            let rotator = Rotator::with_vector(&section.dir);
            for p in points.iter() {
                let r = rotator.rotate(p);
                let q = FloatPointMath::add(&r, &section.b);
                let b = adapter.float_to_int(&q);
                if a != b {
                    segments.push(Segment::subject(a, b));
                }
                a = b;
            }
        }
        let last = adapter.float_to_int(&section.b_top);
        if a != last {
            segments.push(Segment::subject(a, last));
        }
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
    pub(super) fn additional_offset(&self) -> P::Scalar {
        if let Some(points) = &self.points {
            let mut max_sqr_radius = P::Scalar::from_float(0.0);
            for point in points {
                max_sqr_radius = max_sqr_radius.max(FloatPointMath::sqr_length(point));
            }
            max_sqr_radius.sqrt()
        } else {
            P::Scalar::from_float(0.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::fill_rule::FillRule;
    use crate::core::overlay::Overlay;
    use crate::core::overlay_rule::OverlayRule;
    use crate::mesh::float::stroke::section::SectionToSegment;
    use i_float::float::rect::FloatRect;

    #[test]
    fn round_caps_do_not_emit_collapsed_edges() {
        let adapter = FloatPointAdapter::<[f64; 2], i32>::with_scale(
            FloatRect {
                min_x: -3.0,
                max_x: 13.0,
                min_y: -3.0,
                max_y: 3.0,
            },
            1.0,
        );
        let section = Section::new(2.0, &[0.0, 0.0], &[10.0, 0.0]);
        let cap = CapBuilder::new(LineCap::Round(0.01 * PI), 2.0);
        let mut segments = Vec::new();
        segments.add_section(&section, &adapter);
        cap.add_to_start(&section, &adapter, &mut segments);
        cap.add_to_end(&section, &adapter, &mut segments);

        let collapsed = segments.iter().filter(|s| s.x_segment.a == s.x_segment.b).count();
        assert_eq!(
            collapsed, 0,
            "cap construction must skip coincident integer endpoints"
        );
        let output = Overlay::with_segments(segments).overlay(OverlayRule::Subject, FillRule::Positive);
        assert_eq!(output.len(), 1);
    }
}
