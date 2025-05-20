use alloc::vec::Vec;
use crate::geom::v_segment::VSegment;
use crate::vector::edge::{VectorEdge, VectorPath};
use i_float::int::point::IntPoint;
use i_key_sort::bin_key::index::{BinKey, BinLayout};
use i_shape::int::path::IntPath;

#[derive(Debug, Clone, Copy)]
pub(crate) struct ContourIndex {
    data: usize,
}

impl ContourIndex {
    pub(crate) const EMPTY: ContourIndex = ContourIndex { data: usize::MAX };

    #[inline]
    pub(crate) fn is_hole(&self) -> bool {
        self.data & 1 == 1
    }

    #[inline]
    pub(crate) fn index(&self) -> usize {
        self.data >> 1
    }

    #[inline]
    pub(crate) fn new_hole(index: usize) -> Self {
        Self {
            data: (index << 1) | 1,
        }
    }

    #[inline]
    pub(crate) fn new_shape(index: usize) -> Self {
        Self { data: index << 1 }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct IdSegment {
    pub(crate) contour_index: ContourIndex,
    pub(crate) v_segment: VSegment,
}

impl IdSegment {
    #[inline]
    fn new(data: ContourIndex, a: IntPoint, b: IntPoint) -> Self {
        Self {
            contour_index: data,
            v_segment: VSegment { a, b },
        }
    }

    #[inline]
    pub(crate) fn with_segment(data: ContourIndex, v_segment: VSegment) -> Self {
        Self {
            contour_index: data,
            v_segment,
        }
    }
}

pub(crate) trait IdSegments {
    fn append_id_segments(
        &self,
        buffer: &mut Vec<IdSegment>,
        id_data: ContourIndex,
        x_min: i32,
        x_max: i32,
        clockwise: bool,
    );
}

impl IdSegments for IntPath {
    #[inline]
    fn append_id_segments(
        &self,
        buffer: &mut Vec<IdSegment>,
        id_data: ContourIndex,
        x_min: i32,
        x_max: i32,
        clockwise: bool,
    ) {
        fn inner<I: Iterator<Item = IntPoint>>(
            mut iter: I,
            buffer: &mut Vec<IdSegment>,
            id_data: ContourIndex,
            x_min: i32,
            x_max: i32,
        ) {
            let first = iter.next().unwrap();
            let mut b = first;
            for a in iter {
                if a.x < b.x && x_min < b.x && a.x <= x_max {
                    buffer.push(IdSegment::new(id_data, a, b));
                }
                b = a;
            }
            let a = first;
            if a.x < b.x && x_min < b.x && a.x <= x_max {
                buffer.push(IdSegment::new(id_data, a, b));
            }
        }

        if clockwise {
            inner(self.iter().copied(), buffer, id_data, x_min, x_max);
        } else {
            inner(self.iter().rev().copied(), buffer, id_data, x_min, x_max);
        }
    }
}

impl IdSegments for VectorPath {
    #[inline]
    fn append_id_segments(
        &self,
        buffer: &mut Vec<IdSegment>,
        id_data: ContourIndex,
        x_min: i32,
        x_max: i32,
        clockwise: bool,
    ) {
        fn inner<I: Iterator<Item = VectorEdge>>(
            iter: I,
            buffer: &mut Vec<IdSegment>,
            id_data: ContourIndex,
            x_min: i32,
            x_max: i32,
        ) {
            for vec in iter {
                if vec.a.x < vec.b.x && x_min < vec.b.x && vec.a.x <= x_max {
                    buffer.push(IdSegment::new(id_data, vec.a, vec.b));
                }
            }
        }

        if clockwise {
            inner(self.iter().copied(), buffer, id_data, x_min, x_max);
        } else {
            inner(self.iter().rev().copied(), buffer, id_data, x_min, x_max);
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
