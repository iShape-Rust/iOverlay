use std::cmp::Ordering;
use i_float::point::Point;
use i_float::triangle::Triangle;
use crate::x_order::XOrder;
use crate::line_range::LineRange;

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
        debug_assert!(self.a.x <= p.x && p.x <= self.b.x);
        debug_assert!(p != self.a && p != self.b);
        Triangle::is_clockwise_point(self.a, p, self.b)
    }

    pub(crate) fn is_under_segment(&self, other: XSegment) -> bool {
        if self.a == other.a {
            Triangle::is_clockwise_point(self.a, other.b, self.b)
        } else if self.a.x < other.a.x {
            Triangle::is_clockwise_point(self.a, other.a, self.b)
        } else {
            Triangle::is_clockwise_point(other.a, other.b, self.a)
        }
    }

    pub(crate) fn order(&self, other: &Self) -> Ordering {
        if self.is_less(other) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }

    pub(crate) fn is_less(&self, other: &Self) -> bool {
        if self.a == other.a {
            self.b.order_by_line_compare(other.b)
        } else {
            self.a.order_by_line_compare(other.a)
        }
    }
}