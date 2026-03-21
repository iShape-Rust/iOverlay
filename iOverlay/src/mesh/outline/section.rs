use crate::mesh::math::Math;
use crate::segm::segment::Segment;
use alloc::vec::Vec;
use core::marker::PhantomData;
use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_float::float::vector::FloatPointMath;
use i_float::int::point::IntPoint;
use crate::segm::boolean::ShapeCountBoolean;

#[derive(Debug, Clone)]
pub(super) struct Section<P: FloatPointCompatible<T>, T: FloatNumber> {
    pub(super) b: P,
    pub(super) a_top: P,
    pub(super) b_top: P,
    pub(super) dir: P,
    _phantom: PhantomData<T>,
}

impl<T: FloatNumber, P: FloatPointCompatible<T>> Section<P, T> {
    pub(super) fn new(radius: T, a: &P, b: &P) -> Self {
        let dir = Math::normal(b, a);
        let t = Math::ortho_and_scale(&dir, radius);

        let a_top = FloatPointMath::add(a, &t);
        let b_top = FloatPointMath::add(b, &t);

        Section {
            b: *b,
            a_top,
            b_top,
            dir,
            _phantom: Default::default(),
        }
    }

    pub(super) fn is_positive_turn(&self, other: &Self) -> bool {
        let cross_product = FloatPointMath::cross_product(&self.dir, &other.dir);
        cross_product >= T::from_float(0.0)
    }
}

pub(super) trait SectionToSegment<T: FloatNumber, P: FloatPointCompatible<T>> {
    fn add_expand_section(&mut self, section: &Section<P, T>, adapter: &FloatPointAdapter<P, T>);
    fn add_shrink_section(&mut self, a: IntPoint, b: IntPoint, section: &Section<P, T>, adapter: &FloatPointAdapter<P, T>);
}

impl<T: FloatNumber, P: FloatPointCompatible<T>> SectionToSegment<T, P> for Vec<Segment<ShapeCountBoolean>> {
    fn add_expand_section(&mut self, section: &Section<P, T>, adapter: &FloatPointAdapter<P, T>) {
        let a_top = adapter.float_to_int(&section.a_top);
        let b_top = adapter.float_to_int(&section.b_top);
        if a_top != b_top {
            self.push(Segment::subject(a_top, b_top));
        }
    }

    fn add_shrink_section(&mut self, a: IntPoint, b: IntPoint, section: &Section<P, T>, adapter: &FloatPointAdapter<P, T>) {
        let a_top = adapter.float_to_int(&section.a_top);
        let b_top = adapter.float_to_int(&section.b_top);
        debug_assert!(a != b);

        // ToDO Subject+clip
        self.push(Segment::subject(b, a));

        if a != a_top {
            self.push(Segment::subject(a, a_top));
        }
        if b != b_top {
            self.push(Segment::subject(b_top, b));
        }
        if a_top != b_top {
            self.push(Segment::subject(a_top, b_top));
        }
    }
}


#[derive(Debug, Clone)]
pub(super) struct OffsetSection<P: FloatPointCompatible<T>, T: FloatNumber> {
    pub(super) a: IntPoint,
    pub(super) b: IntPoint,
    pub(super) a_top: IntPoint,
    pub(super) b_top: IntPoint,
    pub(super) dir: P,
    pub(super) _phantom: PhantomData<T>,
}

impl<P: FloatPointCompatible<T>, T: FloatNumber> OffsetSection<P, T> {
    #[inline]
    pub(super) fn top_segment(&self) -> Option<Segment<ShapeCountBoolean>> {
        if self.a_top != self.b_top {
            Some(Segment::subject(self.a_top, self.b_top))
        } else {
            None
        }
    }

    #[inline]
    pub(super) fn a_segment(&self) -> Option<Segment<ShapeCountBoolean>> {
        if self.a_top != self.a {
            Some(Segment::subject(self.a, self.a_top))
        } else {
            None
        }
    }

    #[inline]
    pub(super) fn b_segment(&self) -> Option<Segment<ShapeCountBoolean>> {
        if self.b_top != self.b {
            Some(Segment::subject(self.b_top, self.b))
        } else {
            None
        }
    }
}