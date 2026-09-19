use super::{bounds::stroke_radius, builder_join::JoinBuilder, cap::Cap, section::Section};
use crate::mesh::int::{
    join::Join,
    math::{backend::MeshMath, integer::IntegerMath},
    style::IntStrokeStyle,
};
use crate::segm::{boolean::ShapeCountBoolean, segment::Segment};
use alloc::vec::Vec;
use i_float::int::{number::int::IntNumber, point::IntPoint};

pub(super) struct StrokeBuilder<I: IntNumber, M: MeshMath<I> = IntegerMath> {
    radius: I,
    join: Join<I, M>,
    start: Cap<I, M>,
    end: Cap<I, M>,
    sections: Vec<Section<I>>,
}
impl<I: IntNumber, M: MeshMath<I>> StrokeBuilder<I, M> {
    pub(super) fn new(style: &IntStrokeStyle<I>) -> Self {
        Self {
            radius: stroke_radius(style),
            join: Join::new(style.join, style.miter_min_turn),
            start: Cap::new(&style.start_cap),
            end: Cap::new(&style.end_cap),
            sections: Vec::new(),
        }
    }
    pub(super) fn build(
        &mut self,
        path: impl IntoIterator<Item = IntPoint<I>>,
        closed: bool,
        segments: &mut Vec<Segment<ShapeCountBoolean, I>>,
    ) {
        self.sections.clear();
        if self.radius <= I::ONE {
            // Still consume points for the caller's debug range validation.
            #[cfg(debug_assertions)]
            for _ in path {}
            return;
        }
        let mut path = path.into_iter();
        let Some(first) = path.next() else {
            return;
        };
        let mut previous = first;
        for next in path {
            if previous != next {
                self.sections.push(Section::new::<M>(previous, next, self.radius));
                previous = next;
            }
        }
        if closed && previous != first {
            self.sections
                .push(Section::new::<M>(previous, first, self.radius));
        }
        if self.sections.is_empty() {
            return;
        }
        for s in &self.sections {
            s.add(segments);
        }
        for i in 1..self.sections.len() {
            self.join
                .add_join(self.sections[i - 1], self.sections[i], self.radius, segments);
        }
        let first = self.sections[0];
        let last = *self.sections.last().unwrap();
        if closed {
            self.join.add_join(last, first, self.radius, segments);
        } else {
            let backward = M::normalize(first.a - first.b).unwrap();
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mesh::int::{
        arc::ArcOptions,
        style::{IntLineCap, IntLineJoin},
    };
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
                    let mut builder = StrokeBuilder::<i32>::new(&style);
                    for closed in [false, true] {
                        for end in [IntPoint::new(10, 10), IntPoint::new(0, 0), IntPoint::new(20, 1)] {
                            let path = [
                                IntPoint::new(0, 0),
                                IntPoint::new(0, 0),
                                IntPoint::new(10, 0),
                                end,
                            ];
                            let mut segments = Vec::new();
                            builder.build(path, closed, &mut segments);
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
