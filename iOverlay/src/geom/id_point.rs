use i_float::int::point::IntPoint;
use i_key_sort::bin_key::index::{BinKey, BinLayout};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) struct IdPoint {
    pub(crate) id: usize,
    pub(crate) point: IntPoint,
}

impl IdPoint {
    pub(crate) fn new(id: usize, point: IntPoint) -> Self {
        Self { id, point }
    }
}

impl BinKey<i32> for IdPoint {
    #[inline(always)]
    fn bin_key(&self) -> i32 {
        self.point.x
    }

    #[inline(always)]
    fn bin_index(&self, layout: &BinLayout<i32>) -> usize {
        layout.index(self.point.x)
    }
}