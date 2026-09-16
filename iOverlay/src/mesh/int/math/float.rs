use super::backend::MeshMath;
use super::integer::IntegerMath;
use crate::mesh::int::arc::FloatArc;
use i_float::int::number::{int::IntNumber, wide_int::WideIntNumber};
use i_float::int::{angle::Angle, point::IntPoint, unit_vector::UnitIntVector, vector::IntVector};

#[derive(Clone, Copy)]
pub(crate) struct FloatMath;

impl<I: IntNumber> MeshMath<I> for FloatMath {
    type Arc = FloatArc<I>;
    fn sin_cos(angle: Angle) -> (i32, i32) {
        angle.sin_cos_with_float()
    }

    fn angle_between(from: UnitIntVector<I>, to: UnitIntVector<I>) -> Angle {
        Angle::between_with_float(from, to)
    }

    #[inline]
    fn normalize(v: IntVector<I>) -> Option<UnitIntVector<I>> {
        UnitIntVector::normalize_with_float(v)
    }

    #[inline]
    fn scale(direction: UnitIntVector<I>, distance: I) -> IntVector<I> {
        direction.scale(distance)
    }

    #[inline]
    fn rotate(direction: UnitIntVector<I>, local: IntPoint<I>) -> IntVector<I> {
        <IntegerMath as MeshMath<I>>::rotate(direction, local)
    }

    fn guard_padding(padding: I::Wide) -> I::Wide {
        // Reserve 2^-34 relative error for normalization/rotation drift, plus
        // four grid units for final coordinate rounding. i16 only needs the
        // absolute reserve and its wide type cannot be shifted by 34.
        let relative = if I::BITS > 16 {
            padding >> 34
        } else {
            I::Wide::ZERO
        };
        padding + relative + I::Wide::FOUR
    }
}
