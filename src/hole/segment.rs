use std::cmp::Ordering;
use i_float::fix_vec::FixVec;
use i_float::point::Point;
use i_shape::fix_path::FixPath;
use crate::x_segment::XSegment;
use crate::vector::vector::VectorPath;

#[derive(Debug, Clone, Copy)]
pub(crate) struct IdSegment {
    pub(crate) id: usize,
    pub(crate) x_segment: XSegment,
}

impl IdSegment {
    pub(crate) fn new(id: usize, a: FixVec, b: FixVec) -> Self {
        Self {
            id,
            x_segment: XSegment {
                a: Point::new_fix_vec(a),
                b: Point::new_fix_vec(b),
            },
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

pub(crate) trait IdSegments {
    fn id_segments(&self, id: usize, x_min: i32, x_max: i32) -> Vec<IdSegment>;
}

impl IdSegments for FixPath {
    fn id_segments(&self, id: usize, x_min: i32, x_max: i32) -> Vec<IdSegment> {
        let n = self.len();
        let mut list = Vec::with_capacity(3 * n / 4);

        let x_min64 = x_min as i64;
        let x_max64 = x_max as i64;

        let mut b = self[n - 1];
        for &a in self.iter() {
            if a.x < b.x && x_min64 < b.x && a.x <= x_max64 {
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

        let x_min64 = x_min as i64;
        let x_max64 = x_max as i64;

        for vec in self.iter() {
            if vec.a.x < vec.b.x && x_min64 < vec.b.x && vec.a.x <= x_max64 {
                list.push(IdSegment::new(id, vec.a, vec.b));
            }
        }
        list
    }
}