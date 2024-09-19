use i_float::point::IntPoint;
use i_key_sort::index::{BinKey, BinLayout};

#[derive(Clone)]
pub(crate) struct End {
    pub(crate) index: usize,
    pub(crate) point: IntPoint,
}

impl BinKey for End {
    #[inline(always)]
    fn key(&self) -> i64 {
        self.point.x.into()
    }

    #[inline(always)]
    fn bin(&self, layout: &BinLayout) -> usize {
        layout.index(self.point.x.into())
    }
}