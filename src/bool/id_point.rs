use i_float::point::Point;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(super) struct IdPoint {
    pub(super) id: usize,
    pub(super) point: Point
}

impl IdPoint {

    pub(super) fn new(id: usize, point: Point) -> Self {
        Self { id, point }
    }

}