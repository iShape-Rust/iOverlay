use i_float::int::point::IntPoint;
use i_key_sort::sort::key_sort::KeySort;

#[derive(Clone, Copy, PartialEq)]
pub(super) struct LineMark {
    pub(super) index: usize,
    pub(super) point: IntPoint,
}

pub(super) trait SortMarkByIndexAndPoint {
    fn sort_by_index_and_point(&mut self, parallel: bool);
}

impl SortMarkByIndexAndPoint for [LineMark] {
    #[inline]
    fn sort_by_index_and_point(&mut self, parallel: bool) {
        self.sort_by_two_keys_then_by(
            parallel,
            |m| m.index,
            |m| m.point.x,
            |m0, m1| m0.point.y.cmp(&m1.point.y),
        );
    }
}
