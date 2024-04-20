use i_float::point::IntPoint;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct IdPoint {
    pub id: usize,
    pub point: IntPoint,
}

impl IdPoint {
    pub const ZERO: Self = Self { id: 0, point: IntPoint::ZERO };

    pub fn new(id: usize, point: IntPoint) -> Self {
        Self { id, point }
    }
}