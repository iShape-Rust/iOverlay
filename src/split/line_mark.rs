use i_float::point::IntPoint;
use i_key_sort::index::{BinKey, BinLayout};

#[derive(Clone)]
pub(super) struct LineMark {
    pub(super) index: usize,
    pub(super) length: i64,
    pub(super) point: IntPoint,
}

impl BinKey for LineMark {
    #[inline(always)]
    fn key(&self) -> i64 {
        self.index as i64
    }

    #[inline(always)]
    fn bin(&self, layout: &BinLayout) -> usize {
        layout.index(self.index as i64)
    }
}