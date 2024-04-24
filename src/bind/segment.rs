use std::cmp::Ordering;
use i_float::point::IntPoint;
use i_shape::int::path::IntPath;
use crate::x_segment::XSegment;
use crate::vector::vector::VectorPath;

#[derive(Debug, Clone, Copy)]
pub struct IdSegment {
    pub id: usize,
    pub x_segment: XSegment,
}

impl IdSegment {
    pub fn new(id: usize, a: IntPoint, b: IntPoint) -> Self {
        Self {
            id,
            x_segment: XSegment { a, b },
        }
    }
}

impl PartialEq<Self> for IdSegment {
    fn eq(&self, other: &Self) -> bool {
        self.x_segment == other.x_segment
    }
}

impl Eq for IdSegment {}

impl PartialOrd for IdSegment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for IdSegment {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.x_segment.is_under_segment(other.x_segment) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

pub trait IdSegments {
    fn id_segments(&self, id: usize, x_min: i32, x_max: i32) -> Vec<IdSegment>;
}

impl IdSegments for IntPath {
    fn id_segments(&self, id: usize, x_min: i32, x_max: i32) -> Vec<IdSegment> {
        let n = self.len();
        let mut list = Vec::with_capacity(3 * n / 4);

        let mut b = self[n - 1];
        for &a in self.iter() {
            if a.x < b.x && x_min < b.x && a.x <= x_max {
                list.push(IdSegment::new(id, a, b));
            }
            b = a
        }
        list
    }
}

impl IdSegments for VectorPath {
    fn id_segments(&self, id: usize, x_min: i32, x_max: i32) -> Vec<IdSegment> {
        let n = self.len();
        let mut list = Vec::with_capacity(3 * n / 4);

        for vec in self.iter() {
            if vec.a.x < vec.b.x && x_min < vec.b.x && vec.a.x <= x_max {
                list.push(IdSegment::new(id, vec.a, vec.b));
            }
        }
        list
    }
}