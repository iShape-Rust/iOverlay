use i_float::fix_vec::FixVec;
use i_float::point::Point;
use i_shape::fix_path::FixPath;
use crate::geom::x_segment::XSegment;
use crate::vector::vector::VectorPath;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Floor {
    pub(crate) id: usize,
    pub(crate) seg: XSegment,
}

impl Floor {
    pub(crate) fn new(id: usize, a: FixVec, b: FixVec) -> Self {
        Self {
            id,
            seg: XSegment {
                a: Point::new_fix_vec(a),
                b: Point::new_fix_vec(b),
            },
        }
    }
}

pub(crate) trait Floors {
    fn floors(&self, id: usize, x_min: i32, x_max: i32, y_min: &mut i32, y_max: &mut i32) -> Vec<Floor>;
}

impl Floors for FixPath {
    fn floors(&self, id: usize, x_min: i32, x_max: i32, y_min: &mut i32, y_max:&mut i32) -> Vec<Floor> {
        let n = self.len();
        let mut list = Vec::with_capacity(3 * n / 4);

        let x_min64 = x_min as i64;
        let x_max64 = x_max as i64;

        let mut b = self[n - 1];
        for &a in self.iter() {
            if a.x < b.x && x_min64 < b.x && a.x < x_max64 {
                list.push(Floor::new(id, a, b));
                if a.y < b.y {
                    *y_min = (*y_min).min(a.y as i32);
                    *y_max = (*y_max).max(b.y as i32);
                } else {
                    *y_min = (*y_min).min(b.y as i32);
                    *y_max = (*y_max).max(a.y as i32);
                }
            }
            b = a
        }
        list
    }
}

impl Floors for VectorPath {
    fn floors(&self, id: usize, x_min: i32, x_max: i32, y_min: &mut i32, y_max:&mut i32) -> Vec<Floor> {
        let n = self.len();
        let mut list = Vec::with_capacity(3 * n / 4);

        let x_min64 = x_min as i64;
        let x_max64 = x_max as i64;

        for vec in self.iter() {
            if vec.a.x < vec.b.x && x_min64 < vec.b.x && vec.a.x < x_max64 {
                list.push(Floor::new(id, vec.a, vec.b));
                if vec.a.y < vec.b.y {
                    *y_min = (*y_min).min(vec.a.y as i32);
                    *y_max = (*y_max).max(vec.b.y as i32);
                } else {
                    *y_min = (*y_min).min(vec.b.y as i32);
                    *y_max = (*y_max).max(vec.a.y as i32);
                }
            }
        }
        list
    }
}