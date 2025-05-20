use core::cmp::Ordering;
use core::mem;
use i_tree::ExpiredKey;
use i_float::int::point::IntPoint;
use i_float::triangle::Triangle;
use crate::geom::x_segment::XSegment;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct VSegment {
    pub(crate) a: IntPoint,
    pub(crate) b: IntPoint,
}

impl VSegment {
    #[inline(always)]
    fn is_under_segment_order(&self, other: &VSegment) -> Ordering {
        match self.a.cmp(&other.a) {
            Ordering::Less => Triangle::clock_order_point(self.a, other.a, self.b),
            Ordering::Equal => Triangle::clock_order_point(self.a, other.b, self.b),
            Ordering::Greater => Triangle::clock_order_point(other.a, other.b, self.a),
        }
    }

    #[inline(always)]
    pub(crate) fn is_under_point_order(&self, p: IntPoint) -> Ordering {
        debug_assert!(self.a.x <= p.x && p.x <= self.b.x);
        debug_assert!(p != self.a && p != self.b);

        Triangle::clock_order_point(self.a, p, self.b)
    }

    #[inline(always)]
    pub(crate) fn is_under_segment(&self, other: &VSegment) -> bool {
        match self.a.cmp(&other.a) {
            Ordering::Less => {
                Triangle::is_clockwise_point(self.a, other.a, self.b)
            }
            Ordering::Equal => {
                Triangle::is_clockwise_point(self.a, other.b, self.b)
            }
            Ordering::Greater => {
                Triangle::is_clockwise_point(other.a, other.b, self.a)
            }
        }
    }

    #[inline(always)]
    pub(crate) fn cmp_by_angle(&self, other: &Self) -> Ordering {
        // sort angles counterclockwise
        debug_assert!(self.a == other.a);
        let v0 = self.b.subtract(self.a);
        let v1 = other.b.subtract(other.a);
        let cross = v0.cross_product(v1);
        0.cmp(&cross)
    }
}

impl From<VSegment> for XSegment {
    #[inline(always)]
    fn from(seg: VSegment) -> Self {
        unsafe { mem::transmute(seg) }
    }
}

impl From<XSegment> for VSegment {
    #[inline(always)]
    fn from(seg: XSegment) -> Self {
        unsafe { mem::transmute(seg) }
    }
}

impl PartialOrd<Self> for VSegment {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for VSegment {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> Ordering {
        self.is_under_segment_order(other)
    }
}

impl ExpiredKey<i32> for VSegment {
    #[inline]
    fn expiration(&self) -> i32 {
        self.b.x
    }
}

#[cfg(test)]
mod tests {
    use core::cmp::Ordering;
    use i_float::int::point::IntPoint;
    use crate::geom::v_segment::VSegment;

    #[test]
    fn test_00() {
        let p = IntPoint::new(-10, 10);
        let s = VSegment { a: IntPoint::new(-10, -10), b: IntPoint::new(10, -10) };
        let order = s.is_under_point_order(p);
        assert_eq!(order, Ordering::Less);
    }
}