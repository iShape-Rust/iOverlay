use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_float::float::vector::FloatPointMath;
use crate::buffering::math::Math;

#[derive(Debug, Clone)]
pub(super) struct Section<T, P> {
    pub(super) a: P,
    pub(super) b: P,
    pub(super) a_top: P,
    pub(super) b_top: P,
    pub(super) a_bot: P,
    pub(super) b_bot: P,
    pub(super) dir: P
}

impl<T: FloatNumber, P: FloatPointCompatible<T>> Section<T, P> {
    pub(crate) fn section(radius: T, a: &P, b: &P) -> Self {
        let dir = Math::normal(a, b);
        let t = Math::ortho_and_scale(&dir, radius);

        let a_top = FloatPointMath::add(a, t);
        let a_bot = FloatPointMath::sub(a, t);

        let b_top = FloatPointMath::add(b, t);
        let b_bot = FloatPointMath::sub(b, t);

        Section {
            a,
            b,
            a_top,
            b_top,
            a_bot,
            b_bot,
            dir,
        }
    }
}