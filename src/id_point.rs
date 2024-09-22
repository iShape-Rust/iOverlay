use i_float::point::IntPoint;
use i_key_sort::index::{BinKey, BinLayout};

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

impl BinKey for IdPoint {
    #[inline(always)]
    fn key(&self) -> i64 {
        self.point.x.into()
    }

    #[inline(always)]
    fn bin(&self, layout: &BinLayout) -> usize {
        layout.index(self.point.x.into())
    }
}