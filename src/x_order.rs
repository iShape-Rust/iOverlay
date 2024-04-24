use std::cmp::Ordering;
use i_float::point::IntPoint;

pub trait XOrder {
    fn order_by_x(self, other: Self) -> Ordering;
}

impl XOrder for IntPoint {
    fn order_by_x(self, other: Self) -> Ordering {
        if self.x < other.x {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}