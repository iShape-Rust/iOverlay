use crate::mesh::int::{
    arc::{ArcBuilder, ArcDirection},
    join::Join,
    math::{point, scaled_point, vector},
    style::{IntLineCap, IntStrokeStyle},
};
use crate::mesh::subject::SubjectSegments;
use crate::segm::{boolean::ShapeCountBoolean, segment::Segment};
use alloc::vec::Vec;
use i_float::int::number::{int::IntNumber, uint::UIntNumber, wide_int::WideIntNumber};
use i_float::int::{point::IntPoint, unit_vector::UnitIntVector};

#[derive(Clone, Copy)]
struct Section<I: IntNumber> {
    a: IntPoint<I>,
    b: IntPoint<I>,
    a_left: IntPoint<I>,
    a_right: IntPoint<I>,
    b_left: IntPoint<I>,
    b_right: IntPoint<I>,
    dir: UnitIntVector<I>,
}

impl<I: IntNumber> Section<I> {
    fn new(a: IntPoint<I>, b: IntPoint<I>, radius: I) -> Self {
        let dir = crate::mesh::int::math::direction(b - a).expect("unique section");
        let v = dir.scale(radius);
        Self {
            a,
            b,
            dir,
            a_left: point(a, -v.y, v.x),
            a_right: point(a, v.y, -v.x),
            b_left: point(b, -v.y, v.x),
            b_right: point(b, v.y, -v.x),
        }
    }
    fn add(&self, segments: &mut Vec<Segment<ShapeCountBoolean, I>>) {
        for (a, b) in [(self.a_right, self.b_right), (self.b_left, self.a_left)] {
            segments.push_non_degenerate(a, b);
        }
    }
}

enum Cap<I: IntNumber> {
    Butt,
    Square,
    Round(ArcBuilder<I>),
    Custom(alloc::rc::Rc<[IntPoint<I>]>),
}
impl<I: IntNumber> Cap<I> {
    fn new(cap: &IntLineCap<I>) -> Self {
        match cap {
            IntLineCap::Butt => Self::Butt,
            IntLineCap::Square => Self::Square,
            IntLineCap::Round(options) => Self::Round(ArcBuilder::new(*options)),
            IntLineCap::Custom(points) => Self::Custom(points.clone()),
        }
    }
    fn add(
        &mut self,
        center: IntPoint<I>,
        from: IntPoint<I>,
        to: IntPoint<I>,
        outward: UnitIntVector<I>,
        radius: I,
        segments: &mut Vec<Segment<ShapeCountBoolean, I>>,
    ) {
        if matches!(self, Self::Butt) {
            segments.push_non_degenerate(from, to);
            return;
        }
        let mut previous = from;
        match self {
            Self::Round(arc) => {
                let a = crate::mesh::int::math::direction(from - center).expect("positive radius");
                let b = crate::mesh::int::math::direction(to - center).expect("positive radius");
                for &dir in arc.build(a, b, ArcDirection::Counterclockwise) {
                    let next = scaled_point(center, dir, radius);
                    segments.push_non_degenerate(previous, next);
                    previous = next;
                }
            }
            Self::Square => {
                let v = outward.scale(radius);
                for next in [point(from, v.x, v.y), point(to, v.x, v.y)] {
                    segments.push_non_degenerate(previous, next);
                    previous = next;
                }
            }
            Self::Custom(points) => {
                let v = vector(outward);
                let shift = UnitIntVector::<I>::DENOMINATOR.ilog2();
                for p in points.iter() {
                    // Both products and their sum fit in Wide. Round once so
                    // the rotated cap stays within its conservative bounds.
                    let x = (v.x * p.x.to_wide() - v.y * p.y.to_wide()).shr_round(shift);
                    let y = (v.y * p.x.to_wide() + v.x * p.y.to_wide()).shr_round(shift);
                    let next = point(center, x, y);
                    segments.push_non_degenerate(previous, next);
                    previous = next;
                }
            }
            Self::Butt => {}
        }
        segments.push_non_degenerate(previous, to);
    }
}

pub(super) struct StrokeBuilder<I: IntNumber> {
    radius: I,
    join: Join<I>,
    start: Cap<I>,
    end: Cap<I>,
    sections: Vec<Section<I>>,
}
impl<I: IntNumber> StrokeBuilder<I> {
    pub(super) fn radius(style: &IntStrokeStyle<I>) -> I {
        I::from_wide((style.width.max(I::ZERO).to_wide() + I::Wide::ONE) / I::Wide::TWO)
    }
    pub(super) fn padding(style: &IntStrokeStyle<I>) -> I::Wide {
        let radius = Self::radius(style);
        let cap_padding = |cap: &IntLineCap<I>| match cap {
            IntLineCap::Square => radius.to_wide() * I::Wide::TWO,
            IntLineCap::Custom(points) => points
                .iter()
                .map(|p| {
                    let d =
                        i_float::int::vector::IntVector::<I>::new(p.x.to_wide(), p.y.to_wide()).sqr_length();
                    let root = d.isqrt();
                    I::Wide::from_uint(root)
                        + if root * root < d {
                            I::Wide::ONE
                        } else {
                            I::Wide::ZERO
                        }
                })
                .max()
                .unwrap_or(I::Wide::ZERO),
            _ => radius.to_wide(),
        };
        Join::padding(style.join, radius)
            .max(cap_padding(&style.start_cap))
            .max(cap_padding(&style.end_cap))
    }
    pub(super) fn new(style: &IntStrokeStyle<I>) -> Self {
        Self {
            radius: Self::radius(style),
            join: Join::new(style.join),
            start: Cap::new(&style.start_cap),
            end: Cap::new(&style.end_cap),
            sections: Vec::new(),
        }
    }
    pub(super) fn build(
        &mut self,
        path: &[IntPoint<I>],
        closed: bool,
        segments: &mut Vec<Segment<ShapeCountBoolean, I>>,
    ) {
        self.sections.clear();
        if self.radius <= I::ONE {
            return;
        }
        let Some(&first) = path.first() else {
            return;
        };
        let mut previous = first;
        for &next in &path[1..] {
            if previous != next {
                self.sections.push(Section::new(previous, next, self.radius));
                previous = next;
            }
        }
        if closed && previous != first {
            self.sections.push(Section::new(previous, first, self.radius));
        }
        if self.sections.is_empty() {
            return;
        }
        for s in &self.sections {
            s.add(segments);
        }
        for i in 1..self.sections.len() {
            self.add_join(self.sections[i - 1], self.sections[i], segments);
        }
        let first = self.sections[0];
        let last = *self.sections.last().unwrap();
        if closed {
            self.add_join(last, first, segments);
        } else {
            let backward = crate::mesh::int::math::direction(first.a - first.b).unwrap();
            self.start.add(
                first.a,
                first.a_left,
                first.a_right,
                backward,
                self.radius,
                segments,
            );
            self.end
                .add(last.b, last.b_right, last.b_left, last.dir, self.radius, segments);
        }
    }
    fn add_join(&mut self, a: Section<I>, b: Section<I>, segments: &mut Vec<Segment<ShapeCountBoolean, I>>) {
        let cross = (a.b - a.a).cross_product(b.b - b.a);
        let (from, to, incoming, outgoing) = if cross >= I::Wide::ZERO {
            (a.b_right, b.a_right, a.dir, b.dir)
        } else {
            (
                b.a_left,
                a.b_left,
                crate::mesh::int::math::direction(b.a - b.b).unwrap(),
                crate::mesh::int::math::direction(a.a - a.b).unwrap(),
            )
        };
        if cross >= I::Wide::ZERO {
            segments.push_non_degenerate(b.a_left, a.b_left);
        } else {
            segments.push_non_degenerate(a.b_right, b.a_right);
        }
        self.join.add(
            a.b,
            from,
            to,
            incoming,
            outgoing,
            self.radius,
            ArcDirection::Counterclockwise,
            segments,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mesh::int::{arc::ArcOptions, style::IntLineJoin};
    use i_float::int::angle::Angle;

    #[test]
    fn joins_and_caps_skip_collapsed_edges() {
        for width in [0, 1, 2, 4, 1024] {
            let radius = (width + 1) / 2;
            for cap in [
                IntLineCap::Butt,
                IntLineCap::Square,
                IntLineCap::Round(ArcOptions::default()),
                IntLineCap::Custom(
                    alloc::vec![
                        IntPoint::new(0, -radius),
                        IntPoint::new(0, -radius),
                        IntPoint::new(radius, 0),
                        IntPoint::new(0, radius)
                    ]
                    .into(),
                ),
            ] {
                for join in [
                    IntLineJoin::Bevel,
                    IntLineJoin::Miter(Angle::from_bits(1 << 26)),
                    IntLineJoin::Miter(Angle::from_bits(2_000_000_000)),
                    IntLineJoin::Round(ArcOptions::default()),
                ] {
                    let style = IntStrokeStyle::new(width)
                        .start_cap(cap.clone())
                        .end_cap(cap.clone())
                        .line_join(join);
                    let mut builder = StrokeBuilder::new(&style);
                    for closed in [false, true] {
                        for end in [IntPoint::new(10, 10), IntPoint::new(0, 0), IntPoint::new(20, 1)] {
                            let path = [
                                IntPoint::new(0, 0),
                                IntPoint::new(0, 0),
                                IntPoint::new(10, 0),
                                end,
                            ];
                            let mut segments = Vec::new();
                            builder.build(&path, closed, &mut segments);
                            assert!(segments.iter().all(|s| s.x_segment.a < s.x_segment.b));
                            if width >= 4 {
                                assert!(!segments.is_empty());
                            }
                        }
                    }
                }
            }
        }
    }
}
