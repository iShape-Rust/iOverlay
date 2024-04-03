use std::cmp::Ordering;
use i_float::point::Point;

pub trait XOrder {
    fn order_by_x(self, other: Self) -> Ordering;
    fn order_by_line(self, b: Self) -> Ordering;
    fn order_by_line_compare(self, other: Self) -> bool;
}

impl XOrder for Point {
    fn order_by_x(self, other: Self) -> Ordering {
        if self.x < other.x {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }

    fn order_by_line(self, other: Self) -> Ordering {
        if self.order_by_line_compare(other) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }

    fn order_by_line_compare(self, other: Self) -> bool {
        self.x < other.x || self.x == other.x && self.y < other.y
    }
}