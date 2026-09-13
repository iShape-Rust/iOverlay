use crate::mesh::uniq_iter::UniqueSegment;
use i_float::int::number::int::IntNumber;
use i_float::int::point::IntPoint;

#[derive(Clone, Copy)]
pub(super) struct OffsetSection<I: IntNumber> {
    pub(super) a: IntPoint<I>,
    pub(super) b: IntPoint<I>,
    pub(super) a_top: IntPoint<I>,
    pub(super) b_top: IntPoint<I>,
}

impl<I: IntNumber> OffsetSection<I> {
    pub(super) fn new(segment: UniqueSegment<I>, offset: I) -> Self {
        let (a, b) = (segment.a, segment.b);
        if offset == I::ZERO {
            return Self {
                a,
                b,
                a_top: a,
                b_top: b,
            };
        }
        let direction = (b - a)
            .fast_normalize()
            .expect("unique segments must have nonzero length");
        let scaled = direction.scale(offset);

        let dx = scaled.y;
        let dy = -scaled.x;
        let a_top = IntPoint::new(I::from_wide(a.x.to_wide() + dx), I::from_wide(a.y.to_wide() + dy));
        let b_top = IntPoint::new(I::from_wide(b.x.to_wide() + dx), I::from_wide(b.y.to_wide() + dy));

        debug_assert!(a_top.is_in_safe_range() && b_top.is_in_safe_range());

        Self { a, b, a_top, b_top }
    }
}
