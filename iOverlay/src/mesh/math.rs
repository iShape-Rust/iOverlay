use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_float::float::vector::FloatPointMath;

pub(crate) struct Math<T, P> {
    _phantom: core::marker::PhantomData<(T, P)>,
}
impl<T: FloatNumber, P: FloatPointCompatible<T>> Math<T, P> {
    #[inline(always)]
    pub(crate) fn normal(a: &P, b: &P) -> P {
        let c = FloatPointMath::sub(a, b);
        FloatPointMath::normalize(&c)
    }

    #[inline(always)]
    pub(crate) fn ortho_and_scale(p: &P, s: T) -> P {
        let t = P::from_xy(-p.y(), p.x());
        FloatPointMath::scale(&t, s)
    }
}