use crate::mesh::int::{
    arc::{ArcDirection, ArcMath},
    math::{backend::MeshMath, point},
    style::IntLineCap,
};
use crate::mesh::subject::SubjectSegments;
use crate::segm::{boolean::ShapeCountBoolean, segment::Segment};
use alloc::vec::Vec;
use i_float::int::{number::int::IntNumber, point::IntPoint, unit_vector::UnitIntVector};

pub(super) enum Cap<I: IntNumber, M: MeshMath<I>> {
    Butt,
    Square,
    Round(M::Arc),
    Custom(alloc::rc::Rc<[IntPoint<I>]>),
}
impl<I: IntNumber, M: MeshMath<I>> Cap<I, M> {
    pub(super) fn new(cap: &IntLineCap<I>) -> Self {
        match cap {
            IntLineCap::Butt => Self::Butt,
            IntLineCap::Square => Self::Square,
            IntLineCap::Round(options) => Self::Round(M::Arc::new(*options)),
            IntLineCap::Custom(points) => Self::Custom(points.clone()),
        }
    }
    pub(super) fn add(
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
                let a = M::normalize(from - center).expect("positive radius");
                let b = M::normalize(to - center).expect("positive radius");
                for &dir in arc.build(a, b, ArcDirection::Counterclockwise) {
                    let offset = M::scale(dir, radius);
                    let next = point(center, offset.x, offset.y);
                    segments.push_non_degenerate(previous, next);
                    previous = next;
                }
            }
            Self::Square => {
                let v = M::scale(outward, radius);
                for next in [point(from, v.x, v.y), point(to, v.x, v.y)] {
                    segments.push_non_degenerate(previous, next);
                    previous = next;
                }
            }
            Self::Custom(points) => {
                for p in points.iter() {
                    let v = M::rotate(outward, *p);
                    let next = point(center, v.x, v.y);
                    segments.push_non_degenerate(previous, next);
                    previous = next;
                }
            }
            Self::Butt => {}
        }
        segments.push_non_degenerate(previous, to);
    }
}
