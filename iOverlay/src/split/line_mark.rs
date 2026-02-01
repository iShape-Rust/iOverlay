use alloc::vec::Vec;
use i_float::int::point::IntPoint;
use i_key_sort::sort::one_key_cmp::OneKeyAndCmpSort;

#[derive(Clone, Copy, PartialEq)]
pub(super) struct LineMark {
    pub(super) index: usize,
    pub(super) point: IntPoint,
}

pub(super) trait SortMarkByIndexAndPoint {
    fn sort_by_index_and_point(&mut self, parallel: bool, reusable_buffer: &mut Vec<LineMark>);
}

impl SortMarkByIndexAndPoint for [LineMark] {
    #[inline]
    fn sort_by_index_and_point(&mut self, parallel: bool, reusable_buffer: &mut Vec<LineMark>) {
        self.sort_by_one_key_then_by_and_buffer(
            parallel,
            reusable_buffer,
            |m| m.index,
            |m0, m1| m0.point.cmp(&m1.point),
        );
    }
}
