use i_float::fix_vec::FixVec;
use i_float::point::IntPoint;

pub(crate) struct NearestClockWiseVector {
    c: IntPoint,        // center
    va: FixVec,         // our target vector
    vb: FixVec,         // nearest vector to Va by clock wise rotation
    ab_more_180: bool,  // is angle between Va and Vb more 180 degrees
}

impl NearestClockWiseVector {
    #[inline(always)]
    pub(crate) fn new(c: IntPoint, a: IntPoint, b: IntPoint) -> Self {
        let va = a.subtract(c);
        let vb = b.subtract(c);
        let ab_more_180 = va.cross_product(vb) <= 0;
        Self { c, va, vb, ab_more_180 }
    }

    #[inline(always)]
    pub(crate) fn add(&mut self, p: IntPoint) -> bool {
        let vp = p.subtract(self.c);
        let ap_more_180 = self.va.cross_product(vp) <= 0;

        if self.ab_more_180 == ap_more_180 {
            // both more 180 or both less 180
            let is_clock_wise = vp.cross_product(self.vb) > 0;
            if is_clock_wise {
                self.vb = vp;
                return true;
            }
        } else if self.ab_more_180 {
            // angle between Va and Vp less 180
            self.ab_more_180 = false;
            self.vb = vp;

            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use i_float::point::IntPoint;

    #[test]
    fn test_nearest_ccw_vector_creation() {
        let c = IntPoint::new(0, 0);
        let a = IntPoint::new(1, 0);
        let b = IntPoint::new(0, 1);

        let nearest_ccw = NearestClockWiseVector::new(c, a, b);

        assert_eq!(nearest_ccw.va, FixVec::new(1, 0));
        assert_eq!(nearest_ccw.vb, FixVec::new(0, 1));
        assert!(!nearest_ccw.ab_more_180);
    }

    #[test]
    fn test_nearest_ccw_vector_add_less_than_180() {
        let c = IntPoint::new(0, 0);
        let a = IntPoint::new(1, 0);
        let b = IntPoint::new(0, 1);

        let mut nearest_ccw = NearestClockWiseVector::new(c, a, b);
        let p = IntPoint::new(-1, 0);

        let updated = nearest_ccw.add(p);

        assert!(!updated);
        assert_eq!(nearest_ccw.vb, FixVec::new(0, 1));
        assert!(!nearest_ccw.ab_more_180);
    }

    #[test]
    fn test_nearest_ccw_vector_add_more_than_180() {
        let c = IntPoint::new(0, 0);
        let a = IntPoint::new(1, 0);
        let b = IntPoint::new(-1, 0);

        let mut nearest_ccw = NearestClockWiseVector::new(c, a, b);
        let p = IntPoint::new(0, 1);

        let updated = nearest_ccw.add(p);

        assert!(updated);
        assert_eq!(nearest_ccw.vb, FixVec::new(0, 1));
        assert!(!nearest_ccw.ab_more_180);
    }

    #[test]
    fn test_nearest_ccw_vector_no_update() {
        let c = IntPoint::new(0, 0);
        let a = IntPoint::new(1, 0);
        let b = IntPoint::new(0, 1);

        let mut nearest_ccw = NearestClockWiseVector::new(c, a, b);
        let p = IntPoint::new(1, 1);

        let updated = nearest_ccw.add(p);

        assert!(updated);
        assert_eq!(nearest_ccw.vb, FixVec::new(1, 1));
    }
}
