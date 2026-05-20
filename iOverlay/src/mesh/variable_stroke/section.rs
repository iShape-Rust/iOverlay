use crate::mesh::math::Math;
use crate::mesh::variable_stroke::style::StrokeVertex;
use crate::segm::boolean::ShapeCountBoolean;
use crate::segm::segment::Segment;
use alloc::vec::Vec;
use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_float::float::vector::FloatPointMath;

#[derive(Debug, Clone)]
pub(super) struct Section<P: FloatPointCompatible> {
    pub(super) a: P,
    pub(super) b: P,
    pub(super) a_radius: P::Scalar,
    pub(super) b_radius: P::Scalar,
    pub(super) a_top: P,
    pub(super) b_top: P,
    pub(super) a_bot: P,
    pub(super) b_bot: P,
    pub(super) dir: P,
}

impl<P: FloatPointCompatible> Section<P> {
    pub(super) fn new(a: &StrokeVertex<P>, b: &StrokeVertex<P>) -> Self {
        let dir = Math::normal(&b.point, &a.point);
        let ta = Math::ortho_and_scale(&dir, a.radius());
        let tb = Math::ortho_and_scale(&dir, b.radius());

        let a_top = FloatPointMath::add(&a.point, &ta);
        let a_bot = FloatPointMath::sub(&a.point, &ta);

        let b_top = FloatPointMath::add(&b.point, &tb);
        let b_bot = FloatPointMath::sub(&b.point, &tb);

        Section {
            a: a.point,
            b: b.point,
            a_radius: a.radius(),
            b_radius: b.radius(),
            a_top,
            b_top,
            a_bot,
            b_bot,
            dir,
        }
    }

    #[inline]
    pub(super) fn join_radius(&self, next: &Self) -> P::Scalar {
        self.b_radius.max(next.a_radius)
    }
}

pub(super) trait SectionToSegment<P: FloatPointCompatible> {
    fn add_section(&mut self, section: &Section<P>, adapter: &FloatPointAdapter<P>);
}

impl<P: FloatPointCompatible> SectionToSegment<P> for Vec<Segment<ShapeCountBoolean>> {
    fn add_section(&mut self, section: &Section<P>, adapter: &FloatPointAdapter<P>) {
        let a_top = adapter.float_to_int(&section.a_top);
        let b_top = adapter.float_to_int(&section.b_top);
        let a_bot = adapter.float_to_int(&section.a_bot);
        let b_bot = adapter.float_to_int(&section.b_bot);

        if a_top != b_top {
            self.push(Segment::subject(b_top, a_top));
        }
        if a_bot != b_bot {
            self.push(Segment::subject(a_bot, b_bot));
        }
    }
}
