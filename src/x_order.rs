use std::cmp::Ordering;
use i_float::point::Point;

pub trait XOrder {
    fn order_by_x(self, other: Self) -> Ordering;
}

impl XOrder for Point {
    fn order_by_x(self, other: Self) -> Ordering {
        if self.x < other.x {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}