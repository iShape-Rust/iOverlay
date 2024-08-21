use std::cmp::Ordering;
use i_float::point::IntPoint;
use i_shape::int::path::IntPath;
use crate::segm::x_segment::XSegment;
use crate::vector::vector::VectorPath;

#[derive(Debug, Clone, Copy)]
pub struct IdSegment {
    pub id: usize,
    pub x_segment: XSegment,
}

impl IdSegment {
    #[inline(always)]
    pub fn new(id: usize, a: IntPoint, b: IntPoint) -> Self {
        Self {
            id,
            x_segment: XSegment { a, b },
        }
    }
}

impl PartialEq<Self> for IdSegment {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.x_segment == other.x_segment
    }
}

impl Eq for IdSegment {}

impl PartialOrd for IdSegment {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for IdSegment {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> Ordering {
        if self.x_segment.is_under_segment(&other.x_segment) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

pub trait IdSegments {
    fn append_id_segments(&self, buffer: &mut Vec<IdSegment>, id: usize, x_min: i32, x_max: i32);
}

impl IdSegments for IntPath {
    fn append_id_segments(&self, buffer: &mut Vec<IdSegment>, id: usize, x_min: i32, x_max: i32) {
        let mut b = self[self.len() - 1];
        for &a in self.iter() {
            if a.x < b.x && x_min < b.x && a.x <= x_max {
                buffer.push(IdSegment::new(id, a, b));
            }
            b = a
        }
    }
}

impl IdSegments for VectorPath {
    fn append_id_segments(&self, buffer: &mut Vec<IdSegment>, id: usize, x_min: i32, x_max: i32) {
        for vec in self.iter() {
            if vec.a.x < vec.b.x && x_min < vec.b.x && vec.a.x <= x_max {
                buffer.push(IdSegment::new(id, vec.a, vec.b));
            }
        }
    }
}