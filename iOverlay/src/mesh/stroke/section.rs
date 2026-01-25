use alloc::vec::Vec;
use core::marker::PhantomData;
use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_float::float::vector::FloatPointMath;
use crate::mesh::math::Math;
use crate::segm::offset::ShapeCountOffset;
use crate::segm::segment::Segment;

#[derive(Debug, Clone)]
pub(super) struct Section<P: FloatPointCompatible<T>, T: FloatNumber> {
    pub(super) a: P,
    pub(super) b: P,
    pub(super) a_top: P,
    pub(super) b_top: P,
    pub(super) a_bot: P,
    pub(super) b_bot: P,
    pub(super) dir: P,
    _phantom: PhantomData<T>,
}

impl<T: FloatNumber, P: FloatPointCompatible<T>> Section<P, T> {
    pub(crate) fn new(radius: T, a: &P, b: &P) -> Self {
        let dir = Math::normal(b, a);
        let t = Math::ortho_and_scale(&dir, radius);

        let a_top = FloatPointMath::add(a, &t);
        let a_bot = FloatPointMath::sub(a, &t);

        let b_top = FloatPointMath::add(b, &t);
        let b_bot = FloatPointMath::sub(b, &t);

        Section {
            a: *a,
            b: *b,
            a_top,
            b_top,
            a_bot,
            b_bot,
            dir,
            _phantom: Default::default(),
        }
    }
}

pub(crate) trait SectionToSegment<T: FloatNumber, P: FloatPointCompatible<T>> {
    fn add_section(&mut self, section: &Section<P, T>, adapter: &FloatPointAdapter<P, T>);
}

impl<T: FloatNumber, P: FloatPointCompatible<T>> SectionToSegment<T, P> for Vec<Segment<ShapeCountOffset>> {
    fn add_section(&mut self, section: &Section<P, T>, adapter: &FloatPointAdapter<P, T>) {
        let a_top = adapter.float_to_int(&section.a_top);
        let b_top = adapter.float_to_int(&section.b_top);
        let a_bot = adapter.float_to_int(&section.a_bot);
        let b_bot = adapter.float_to_int(&section.b_bot);

        if a_top != b_top {
            self.push(Segment::bold_subject_ab(b_top, a_top));
        }
        if a_bot != b_bot {
            self.push(Segment::bold_subject_ab(a_bot, b_bot));
        }
    }
}