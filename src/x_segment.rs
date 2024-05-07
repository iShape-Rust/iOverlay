use std::cmp::Ordering;
use i_float::point::IntPoint;
use i_float::triangle::Triangle;
use crate::line_range::LineRange;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct XSegment {
    pub a: IntPoint,
    pub b: IntPoint,
}

impl XSegment {

    #[inline(always)]
    pub fn new(a: IntPoint, b: IntPoint) -> Self {
        Self { a, b }
    }

    #[inline]
    pub fn y_range(&self) -> LineRange {
        if self.a.y < self.b.y {
            LineRange { min: self.a.y, max: self.b.y }
        } else {
            LineRange { min: self.b.y, max: self.a.y }
        }
    }

    #[inline]
    pub fn is_vertical(&self) -> bool {
        self.a.x == self.b.x
    }

    #[inline]
    pub fn is_under_point(&self, p: IntPoint) -> bool {
        debug_assert!(self.a.x <= p.x && p.x <= self.b.x);
        debug_assert!(p != self.a && p != self.b);
        Triangle::area_two_point(self.a, p, self.b) > 0
    }

    #[inline]
    pub fn is_above_point(&self, p: IntPoint) -> bool {
        debug_assert!(self.a.x <= p.x && p.x <= self.b.x);
        debug_assert!(p != self.a && p != self.b);
        Triangle::area_two_point(self.a, p, self.b) < 0
    }

    #[inline]
    pub fn is_under_segment(&self, other: XSegment) -> bool {
        if self.a == other.a {
            Triangle::is_clockwise_point(self.a, other.b, self.b)
        } else if self.a.x < other.a.x {
            Triangle::is_clockwise_point(self.a, other.a, self.b)
        } else {
            Triangle::is_clockwise_point(other.a, other.b, self.a)
        }
    }
}

impl PartialOrd for XSegment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for XSegment {

    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        if self.a == other.a {
            if self.b == other.b {
                Ordering::Equal
            } else if self.b < other.b {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        } else if self.a < other.a {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}