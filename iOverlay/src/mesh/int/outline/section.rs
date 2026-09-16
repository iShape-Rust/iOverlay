use crate::mesh::int::math::{direction, point};
use crate::mesh::uniq_iter::UniqueSegment;
use i_float::int::number::int::IntNumber;
use i_float::int::point::IntPoint;
use i_float::int::unit_vector::UnitIntVector;

#[derive(Clone, Copy)]
pub(super) struct OffsetSection<I: IntNumber> {
    pub(super) offset: I,
    pub(super) direction: UnitIntVector<I>,
    pub(super) a: IntPoint<I>,
    pub(super) b: IntPoint<I>,
    pub(super) a_top: IntPoint<I>,
    pub(super) b_top: IntPoint<I>,
}

impl<I: IntNumber> OffsetSection<I> {
    pub(super) fn new(segment: UniqueSegment<I>, offset: I) -> Self {
        let (a, b) = (segment.a, segment.b);
        let direction = direction(b - a).expect("unique segment");
        if offset == I::ZERO {
            return Self {
                offset,
                direction,
                a,
                b,
                a_top: a,
                b_top: b,
            };
        }
        let scaled = direction.scale(offset);

        let dx = scaled.y;
        let dy = -scaled.x;
        let a_top = point(a, dx, dy);
        let b_top = point(b, dx, dy);

        Self {
            offset,
            direction,
            a,
            b,
            a_top,
            b_top,
        }
    }
}
