use i_float::int::point::IntPoint;
use i_key_sort::index::{BinKey, BinLayout};

#[derive(Clone, Copy)]
pub(crate) struct End {
    pub(crate) index: usize,
    pub(crate) point: IntPoint,
}

impl BinKey<i32> for End {
    #[inline(always)]
    fn bin_key(&self) -> i32 {
        self.point.x
    }

    #[inline(always)]
    fn bin_index(&self, layout: &BinLayout<i32>) -> usize {
        layout.index(self.point.x)
    }
}