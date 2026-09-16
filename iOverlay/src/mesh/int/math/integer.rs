use super::backend::MeshMath;
use super::{direction, vector};
use crate::mesh::int::arc::IntegerArc;
use i_float::int::number::{int::IntNumber, wide_int::WideIntNumber};
use i_float::int::{angle::Angle, point::IntPoint, unit_vector::UnitIntVector, vector::IntVector};

#[derive(Clone, Copy)]
pub(crate) struct IntegerMath;

impl<I: IntNumber> MeshMath<I> for IntegerMath {
    type Arc = IntegerArc<I>;
    #[inline]
    fn sin_cos(angle: Angle) -> (i32, i32) {
        angle.sin_cos()
    }
    #[inline]
    fn angle_between(from: UnitIntVector<I>, to: UnitIntVector<I>) -> Angle {
        Angle::between(from, to)
    }

    #[inline]
    fn normalize(v: IntVector<I>) -> Option<UnitIntVector<I>> {
        direction(v)
    }

    #[inline]
    fn scale(direction: UnitIntVector<I>, distance: I) -> IntVector<I> {
        direction.scale(distance)
    }

    #[inline]
    fn rotate(direction: UnitIntVector<I>, local: IntPoint<I>) -> IntVector<I> {
        let v = vector(direction);
        let shift = UnitIntVector::<I>::DENOMINATOR.ilog2();
        IntVector::new(
            (v.x * local.x.to_wide() - v.y * local.y.to_wide()).shr_round(shift),
            (v.y * local.x.to_wide() + v.x * local.y.to_wide()).shr_round(shift),
        )
    }

    #[inline]
    fn guard_padding(padding: I::Wide) -> I::Wide {
        padding
    }
}
