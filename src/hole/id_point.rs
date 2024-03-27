use i_float::point::Point;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) struct IdPoint {
    pub(crate) id: usize,
    pub(crate) point: Point
}

impl IdPoint {

    pub(crate) fn new(id: usize, point: Point) -> Self {
        Self { id, point }
    }

}