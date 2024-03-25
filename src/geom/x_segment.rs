use std::cmp::Ordering;
use i_float::fix_vec::FixVec;
use i_float::point::Point;
use i_float::triangle::Triangle;
use crate::geom::x_order::XOrder;
use crate::space::line_range::LineRange;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct XSegment {
    pub(crate) a: Point,
    pub(crate) b: Point,
}

impl XSegment {
    pub fn new(a: Point, b: Point) -> Self {
        Self { a, b }
    }

    pub(crate) fn y_range(&self) -> LineRange {
        if self.a.y < self.b.y {
            LineRange { min: self.a.y, max: self.b.y }
        } else {
            LineRange { min: self.b.y, max: self.a.y }
        }
    }

    pub(crate) fn is_vertical(&self) -> bool {
        self.a.x == self.b.x
    }

    pub(crate) fn is_under_point(&self, p: Point) -> bool {
        assert!(self.a.x <= p.x && p.x <= self.b.x);
        assert!(p != self.a && p != self.b);
        Self::is_clockwise_points(self.a, p, self.b)
    }

    pub(crate) fn is_above_point(&self, p: Point) -> bool {
        assert!(self.a.x <= p.x && p.x <= self.b.x);
        Self::is_clockwise_points(self.a, self.b, p)
    }

    pub(crate) fn is_under_segment(&self, other: XSegment) -> bool {
        if self.a == other.a {
            Self::is_clockwise_points(self.a, other.b, self.b)
        } else if self.a.x < other.a.x {
            Self::is_clockwise_points(self.a, other.a, self.b)
        } else {
            Self::is_clockwise_points(other.a, other.b, self.a)
        }
    }

    pub(crate) fn is_clockwise_points(p0: Point, p1: Point, p2: Point) -> bool {
        Triangle::is_clockwise(FixVec::new_point(p0), FixVec::new_point(p1), FixVec::new_point(p2))
    }

    pub (crate) fn order(&self, other: &Self) -> Ordering {
        if self.a == other.a {
            self.b.order_by_line(other.b)
        } else {
            self.a.order_by_line(other.a)
        }
    }

    pub (crate) fn is_less(&self, other: &Self) -> bool {
        if self.a == other.a {
            self.b.order_by_line_compare(other.b)
        } else {
            self.a.order_by_line_compare(other.a)
        }
    }
}