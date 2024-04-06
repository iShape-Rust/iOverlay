use i_float::point::Point;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct IdPoint {
    pub id: usize,
    pub point: Point
}

impl IdPoint {

    pub fn new(id: usize, point: Point) -> Self {
        Self { id, point }
    }

}