use i_float::int::point::IntPoint;

#[derive(Clone, Copy)]
pub(crate) struct End {
    pub(crate) index: usize,
    pub(crate) point: IntPoint,
}

impl Default for End {
    #[inline(always)]
    fn default() -> Self {
        Self {
            index: 0,
            point: IntPoint::ZERO,
        }
    }
}
