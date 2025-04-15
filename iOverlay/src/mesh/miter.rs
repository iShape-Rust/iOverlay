use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_float::int::point::IntPoint;

pub(super) struct Miter;

pub(super) enum SharpMiter {
    Degenerate,
    AB(IntPoint, IntPoint),
    AcB(IntPoint, IntPoint, IntPoint),
}

impl Miter {
    #[inline]
    pub(super) fn sharp<T: FloatNumber, P: FloatPointCompatible<T>>(
        pa: P,
        pb: P,
        va: P,
        vb: P,
        adapter: &FloatPointAdapter<P, T>,
    ) -> SharpMiter {
        let ia = adapter.float_to_int(&pa);
        let ib = adapter.float_to_int(&pb);

        if ia == ib {
            return SharpMiter::Degenerate;
        }

        let pax = pa.x();
        let pay = pa.y();
        let pbx = pb.x();
        let pby = pb.y();
        let vax = va.x();
        let vay = va.y();
        let vbx = vb.x();
        let vby = vb.y();

        let xx = vax + vbx;
        let yy = vay + vby;

        let k = if xx.abs() > yy.abs() {
            (pbx - pax) / xx
        } else {
            (pby - pay) / yy
        };

        let x = pax + k * vax;
        let y = pay + k * vay;
        let c = P::from_xy(x, y);

        let ic = adapter.float_to_int(&c);

        if ia == ic || ib == ic {
            SharpMiter::AB(ia, ib)
        } else {
            SharpMiter::AcB(ia, ic, ib)
        }
    }
}
