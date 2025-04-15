use i_float::fix_vec::FixVec;
use i_float::int::point::IntPoint;

pub(crate) struct NearestVector {
    c: IntPoint,            // center
    va: FixVec,             // our target vector
    vb: FixVec,             // nearest vector to Va by specified rotation
    ab_more_180: bool,      // is angle between Va and Vb more than 180 degrees
    pub(crate) best_id: usize,
    rotation_factor: i64,   // +1 for clockwise, -1 for counterclockwise
}

impl NearestVector {
    #[inline]
    pub(crate) fn new(c: IntPoint, a: IntPoint, b: IntPoint, best_id: usize, clockwise: bool) -> Self {
        let va = a.subtract(c);
        let vb = b.subtract(c);
        let (ab_more_180, rotation_factor) = if clockwise {
            (va.cross_product(vb) >= 0, 1)
        } else {
            (va.cross_product(vb) <= 0, -1)
        };
        Self { c, va, vb, ab_more_180, best_id, rotation_factor }
    }

    #[inline]
    pub(crate) fn add(&mut self, p: IntPoint, id: usize) {
        let vp = p.subtract(self.c);
        let ap_more_180 = self.va.cross_product(vp) * self.rotation_factor >= 0;

        if self.ab_more_180 == ap_more_180 {
            if vp.cross_product(self.vb) * self.rotation_factor < 0 {
                self.vb = vp;
                self.best_id = id;
            }
        } else if self.ab_more_180 {
            self.ab_more_180 = false;
            self.vb = vp;
            self.best_id = id;
        }
    }
}

#[cfg(test)]
mod tests {
    use i_float::fix_vec::FixVec;
    use i_float::int::point::IntPoint;
    use crate::core::nearest_vector::NearestVector;

    #[test]
    fn test_nearest_ccw_vector_creation() {
        let c = IntPoint::new(0, 0);
        let a = IntPoint::new(1, 0);
        let b = IntPoint::new(0, 1);

        let nearest_ccw = NearestVector::new(c, a, b, 0, false);

        assert_eq!(nearest_ccw.va, FixVec::new(1, 0));
        assert_eq!(nearest_ccw.vb, FixVec::new(0, 1));
        assert!(!nearest_ccw.ab_more_180);
    }

    #[test]
    fn test_nearest_ccw_vector_add_less_than_180() {
        let c = IntPoint::new(0, 0);
        let a = IntPoint::new(1, 0);
        let b = IntPoint::new(0, 1);

        let mut nearest_ccw = NearestVector::new(c, a, b, 0, false);
        let p = IntPoint::new(-1, 0);

        nearest_ccw.add(p, 1);
        assert_eq!(nearest_ccw.vb, FixVec::new(0, 1));
        assert!(!nearest_ccw.ab_more_180);
    }

    #[test]
    fn test_nearest_ccw_vector_add_more_than_180() {
        let c = IntPoint::new(0, 0);
        let a = IntPoint::new(1, 0);
        let b = IntPoint::new(-1, 0);

        let mut nearest_ccw = NearestVector::new(c, a, b, 0, false);
        let p = IntPoint::new(0, 1);
        nearest_ccw.add(p, 1);
        assert_eq!(nearest_ccw.vb, FixVec::new(0, 1));
        assert!(!nearest_ccw.ab_more_180);
    }

    #[test]
    fn test_nearest_ccw_vector_no_update() {
        let c = IntPoint::new(0, 0);
        let a = IntPoint::new(1, 0);
        let b = IntPoint::new(0, 1);

        let mut nearest_ccw = NearestVector::new(c, a, b, 0, false);
        let p = IntPoint::new(1, 1);

        nearest_ccw.add(p, 1);

        assert_eq!(nearest_ccw.vb, FixVec::new(1, 1));
    }

    #[test]
    fn test_ccw_0() {
        let c = IntPoint::new(-1, -1);
        let a = IntPoint::new( 0, -1);
        let b = IntPoint::new(-2, -1);

        let mut nearest_ccw = NearestVector::new(c, a, b, 1, false);
        let p = IntPoint::new(-1, -2);

        nearest_ccw.add(p, 3);

        assert_eq!(nearest_ccw.best_id, 1);
    }

    #[test]
    fn test_nearest_cw_vector_creation() {
        let c = IntPoint::new(0, 0);
        let a = IntPoint::new(1, 0);
        let b = IntPoint::new(0, -1);

        let nearest_cw = NearestVector::new(c, a, b, 0, true);

        assert_eq!(nearest_cw.va, FixVec::new(1, 0));
        assert_eq!(nearest_cw.vb, FixVec::new(0, -1));
        assert!(!nearest_cw.ab_more_180);
    }

    #[test]
    fn test_nearest_cw_vector_add_less_than_180() {
        let c = IntPoint::new(0, 0);
        let a = IntPoint::new(1, 0);
        let b = IntPoint::new(0, -1);

        let mut nearest_cw = NearestVector::new(c, a, b, 0, true);
        let p = IntPoint::new(-1, 0);

        nearest_cw.add(p, 1);
        assert_eq!(nearest_cw.vb, FixVec::new(0, -1));
        assert!(!nearest_cw.ab_more_180);
    }

    #[test]
    fn test_nearest_cw_vector_add_more_than_180() {
        let c = IntPoint::new(0, 0);
        let a = IntPoint::new(1, 0);
        let b = IntPoint::new(-1, 0);

        let mut nearest_cw = NearestVector::new(c, a, b, 0, true);
        let p = IntPoint::new(0, -1);
        nearest_cw.add(p, 1);
        assert_eq!(nearest_cw.vb, FixVec::new(0, -1));
        assert!(!nearest_cw.ab_more_180);
    }

    #[test]
    fn test_nearest_cw_vector_no_update() {
        let c = IntPoint::new(0, 0);
        let a = IntPoint::new(1, 0);
        let b = IntPoint::new(0, -1);

        let mut nearest_cw = NearestVector::new(c, a, b, 0, true);
        let p = IntPoint::new(1, 1);

        nearest_cw.add(p, 1);

        assert_eq!(nearest_cw.vb, FixVec::new(0, -1));
    }

    #[test]
    fn test_cw_0() {
        let c = IntPoint::new(-1, -1);
        let a = IntPoint::new(0, -1);
        let b = IntPoint::new(-2, -1);

        let mut nearest_cw = NearestVector::new(c, a, b, 1, true);
        let p = IntPoint::new(-1, -2);

        nearest_cw.add(p, 3);

        assert_eq!(nearest_cw.best_id, 3);
    }
}
