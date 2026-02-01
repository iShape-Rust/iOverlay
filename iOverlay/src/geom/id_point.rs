use i_float::int::point::IntPoint;

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
