pub(crate) mod backend;
pub(crate) mod float;
pub(crate) mod integer;

use i_float::int::number::{
    int::IntNumber, product_uint::UIntProduct, uint::UIntNumber, wide_int::WideIntNumber,
};
use i_float::int::{point::IntPoint, unit_vector::UnitIntVector, vector::IntVector};

pub(super) fn mul_div<I: IntNumber>(a: I::Wide, b: I::Wide, divisor: I::Wide) -> I::Wide {
    let negative = (a < I::Wide::ZERO) ^ (b < I::Wide::ZERO) ^ (divisor < I::Wide::ZERO);
    let product = <I::WideUInt as UIntNumber>::Product::multiply(a.unsigned_abs(), b.unsigned_abs());
    let value = I::Wide::from_uint(product.divide_with_rounding(divisor.unsigned_abs()));
    if negative { -value } else { value }
}

pub(super) fn vector<I: IntNumber>(direction: UnitIntVector<I>) -> IntVector<I> {
    IntVector::new(direction.x().to_wide(), direction.y().to_wide())
}

pub(super) fn point<I: IntNumber>(center: IntPoint<I>, dx: I::Wide, dy: I::Wide) -> IntPoint<I> {
    let x = center.x.to_wide() + dx;
    let y = center.y.to_wide() + dy;
    debug_assert!(x > -UnitIntVector::<I>::DENOMINATOR && x < UnitIntVector::<I>::DENOMINATOR);
    debug_assert!(y > -UnitIntVector::<I>::DENOMINATOR && y < UnitIntVector::<I>::DENOMINATOR);
    IntPoint::new(I::from_wide(x), I::from_wide(y))
}

pub(super) fn scaled_point<I: IntNumber>(
    center: IntPoint<I>,
    direction: UnitIntVector<I>,
    radius: I,
) -> IntPoint<I> {
    let offset = direction.scale(radius);
    point(center, offset.x, offset.y)
}

pub(super) fn abs<I: IntNumber>(value: I) -> I::Wide {
    let wide = value.to_wide();
    if wide < I::Wide::ZERO { -wide } else { wide }
}

pub(super) fn direction<I: IntNumber>(v: IntVector<I>) -> Option<UnitIntVector<I>> {
    if v.x == I::Wide::ZERO {
        IntVector::<I>::new(I::Wide::ZERO, v.y.signum()).fast_normalize()
    } else if v.y == I::Wide::ZERO {
        IntVector::<I>::new(v.x.signum(), I::Wide::ZERO).fast_normalize()
    } else {
        v.fast_normalize()
    }
}
