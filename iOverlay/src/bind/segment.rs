use i_float::int::point::IntPoint;
use i_key_sort::index::{BinKey, BinLayout};
use i_shape::int::path::IntPath;
use crate::geom::v_segment::VSegment;
use crate::vector::edge::VectorPath;

#[derive(Debug, Clone, Copy)]
pub(crate) struct IdSegment {
    pub(crate) id: usize,
    pub(crate) v_segment: VSegment,
}

impl IdSegment {
    #[inline(always)]
    fn new(id: usize, a: IntPoint, b: IntPoint) -> Self {
        Self {
            id,
            v_segment: VSegment { a, b },
        }
    }
}

pub(crate) trait IdSegments {
    fn append_hull_segments(&self, buffer: &mut Vec<IdSegment>, index: usize, x_min: i32, x_max: i32);
    fn append_hole_segments(&self, buffer: &mut Vec<IdSegment>, index: usize, x_min: i32, x_max: i32);
    fn append_id_segments(&self, buffer: &mut Vec<IdSegment>, id: usize, x_min: i32, x_max: i32);
}

impl IdSegments for IntPath {

    #[inline]
    fn append_hole_segments(&self, buffer: &mut Vec<IdSegment>, index: usize, x_min: i32, x_max: i32) {
        let id= (index << 1) | 1;
        self.append_id_segments(buffer, id, x_min, x_max);
    }

    #[inline]
    fn append_hull_segments(&self, buffer: &mut Vec<IdSegment>, index: usize, x_min: i32, x_max: i32) {
        let id= index << 1;
        self.append_id_segments(buffer, id, x_min, x_max);
    }

    #[inline]
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

    #[inline]
    fn append_hole_segments(&self, buffer: &mut Vec<IdSegment>, index: usize, x_min: i32, x_max: i32) {
        let id= (index << 1) + 1;
        self.append_id_segments(buffer, id, x_min, x_max);
    }

    #[inline]
    fn append_hull_segments(&self, buffer: &mut Vec<IdSegment>, index: usize, x_min: i32, x_max: i32) {
        let id= index << 1;
        self.append_id_segments(buffer, id, x_min, x_max);
    }

    #[inline]
    fn append_id_segments(&self, buffer: &mut Vec<IdSegment>, id: usize, x_min: i32, x_max: i32) {
        for vec in self.iter() {
            if vec.a.x < vec.b.x && x_min < vec.b.x && vec.a.x <= x_max {
                buffer.push(IdSegment::new(id, vec.a, vec.b));
            }
        }
    }
}

impl BinKey<i32> for IdSegment {
    #[inline(always)]
    fn bin_key(&self) -> i32 {
        self.v_segment.a.x
    }

    #[inline(always)]
    fn bin_index(&self, layout: &BinLayout<i32>) -> usize {
        layout.index(self.v_segment.a.x)
    }
}