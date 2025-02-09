use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;

pub(crate) struct Rotator<T: FloatNumber> {
    a_x: T,
    a_y: T,
    b_x: T,
    b_y: T,
}

impl<T: FloatNumber> Rotator<T> {

    #[inline]
    pub(crate) fn new(cs: T, sn: T) -> Self {
        let a_x = cs;
        let a_y = sn;
        let b_x = -a_y;
        let b_y = a_x;

        Self {
            a_x,
            a_y,
            b_x,
            b_y,
        }
    }

    #[inline]
    pub(crate) fn with_angle(angle: f64) -> Self {
        let (sn, cs) = angle.sin_cos();
        let sin = T::from_float(sn);
        let cos = T::from_float(cs);
        Self::new(sin, cos)
    }

    #[inline]
    pub(crate) fn rotate<P: FloatPointCompatible<T>>(&self, v: P) -> P {
        let v_x = v.x();
        let v_y = v.y();
        let x = self.a_x * v_x + self.b_x * v_y;
        let y = self.a_y * v_x + self.b_y * v_y;
        P::from_xy(x, y)
    }
}