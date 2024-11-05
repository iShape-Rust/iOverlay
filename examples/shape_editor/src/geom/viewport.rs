use iced::{Rectangle, Vector};

pub(crate) trait ViewPortExt {
    fn offset(&self) -> Vector<f32>;
}

impl ViewPortExt for Rectangle {
    fn offset(&self) -> Vector {
        Vector::new(
            self.x + 0.5 * self.width,
            self.y + 0.5 * self.height,
        )
    }
}