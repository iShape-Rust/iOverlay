use i_float::point::IntPoint;
use i_key_sort::index::{BinKey, BinLayout};

#[derive(Clone, PartialEq)]
pub(super) struct LineMark {
    pub(super) index: usize,
    pub(super) length: i64,
    pub(super) point: IntPoint,
}

impl BinKey<usize> for LineMark {
    #[inline(always)]
    fn bin_key(&self) -> usize {
        self.index
    }

    #[inline(always)]
    fn bin_index(&self, layout: &BinLayout<usize>) -> usize {
        layout.index(self.index)
    }
}