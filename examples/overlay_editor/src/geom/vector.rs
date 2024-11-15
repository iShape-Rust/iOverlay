use i_triangle::i_overlay::i_float::int::point::IntPoint;
use iced::{Point, Vector};

pub(crate) trait VectorExt {
    fn round(&self) -> IntPoint;
    fn point(point: Point<f32>) -> Self;
}

impl VectorExt for Vector<f32> {
    fn round(&self) -> IntPoint {
        IntPoint::new(
            self.x.round() as i32,
            self.y.round() as i32,
        )
    }
    fn point(point: Point<f32>) -> Self {
        Self { x: point.x, y: point.y }
    }
}