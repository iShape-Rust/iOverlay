use crate::geom::v_segment::VSegment;
use crate::vector::edge::VectorPath;
use i_float::int::point::IntPoint;
use i_key_sort::index::{BinKey, BinLayout};
use i_shape::int::path::IntPath;

#[derive(Debug, Clone, Copy)]
pub(crate) struct IdData {
    data: usize,
}

impl IdData {

    pub(crate) const EMPTY: IdData = IdData { data: usize::MAX };

    #[inline]
    pub(crate) fn is_hole(&self) -> bool {
        self.data & 1 == 0
    }


    #[inline]
    pub(crate) fn index(&self) -> usize {
        self.data >> 1
    }

    #[inline]
    pub(crate) fn new_hole(index: usize) -> Self {
        Self { data: (index << 1) | 1}
    }

    #[inline]
    pub(crate) fn new_hull(index: usize) -> Self {
        Self { data: index << 1}
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct IdSegment {
    pub(crate) data: IdData,
    pub(crate) v_segment: VSegment,
}

impl IdSegment {

    #[inline]
    fn new(data: IdData, a: IntPoint, b: IntPoint) -> Self {
        Self {
            data,
            v_segment: VSegment { a, b },
        }
    }

    #[inline]
    pub(crate) fn with_segment(data: IdData, v_segment: VSegment) -> Self {
        Self {
            data,
            v_segment,
        }
    }
}

pub(crate) trait IdSegments {
    fn append_hull_segments(
        &self,
        buffer: &mut Vec<IdSegment>,
        index: usize,
        x_min: i32,
        x_max: i32,
    );
    fn append_hole_segments(
        &self,
        buffer: &mut Vec<IdSegment>,
        index: usize,
        x_min: i32,
        x_max: i32,
    );
    fn append_id_segments(&self, buffer: &mut Vec<IdSegment>, id_data: IdData, x_min: i32, x_max: i32);
}

impl IdSegments for IntPath {
    #[inline]
    fn append_hole_segments(
        &self,
        buffer: &mut Vec<IdSegment>,
        index: usize,
        x_min: i32,
        x_max: i32,
    ) {
        let data = IdData::new_hole(index);
        self.append_id_segments(buffer, data, x_min, x_max);
    }

    #[inline]
    fn append_hull_segments(
        &self,
        buffer: &mut Vec<IdSegment>,
        index: usize,
        x_min: i32,
        x_max: i32,
    ) {
        let data = IdData::new_hull(index);
        self.append_id_segments(buffer, data, x_min, x_max);
    }

    #[inline]
    fn append_id_segments(&self, buffer: &mut Vec<IdSegment>, id_data: IdData, x_min: i32, x_max: i32) {
        let mut b = self[self.len() - 1];
        for &a in self.iter() {
            if a.x < b.x && x_min < b.x && a.x <= x_max {
                buffer.push(IdSegment::new(id_data, a, b));
            }
            b = a
        }
    }
}

impl IdSegments for VectorPath {
    #[inline]
    fn append_hole_segments(
        &self,
        buffer: &mut Vec<IdSegment>,
        index: usize,
        x_min: i32,
        x_max: i32,
    ) {
        let data = IdData::new_hole(index);
        self.append_id_segments(buffer, data, x_min, x_max);
    }

    #[inline]
    fn append_hull_segments(
        &self,
        buffer: &mut Vec<IdSegment>,
        index: usize,
        x_min: i32,
        x_max: i32,
    ) {
        let data = IdData::new_hull(index);
        self.append_id_segments(buffer, data, x_min, x_max);
    }

    #[inline]
    fn append_id_segments(&self, buffer: &mut Vec<IdSegment>, id_data: IdData, x_min: i32, x_max: i32) {
        for vec in self.iter() {
            if vec.a.x < vec.b.x && x_min < vec.b.x && vec.a.x <= x_max {
                buffer.push(IdSegment::new(id_data, vec.a, vec.b));
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
