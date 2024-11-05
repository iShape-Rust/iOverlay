use i_triangle::i_overlay::i_float::int::point::IntPoint;
use i_triangle::i_overlay::i_float::int::rect::IntRect;
use iced::{Size, Vector};

#[derive(Debug, Clone, Copy)]
pub(crate) struct Camera {
    pub(crate) scale: f32,
    pub(crate) pos: Vector<f32>,
}

impl Camera {

    pub(crate) fn empty() -> Self {
        Self { scale: 0.0, pos: Vector::new(0.0 ,0.0)  }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.scale < 0.000_000_0001
    }

    pub(crate) fn is_not_empty(&self) -> bool {
        self.scale > 0.0
    }

    pub(crate) fn new(rect: IntRect, size: Size) -> Self {
        let w_pow = rect.width().ilog2() as usize;
        let h_pow = rect.height().ilog2() as usize;

        let width = (1 << w_pow) as f32;
        let height = (1 << h_pow) as f32;
        let sw = size.width / width;
        let sh = size.height / height;

        let scale = 0.25 * sw.min(sh);
        let x = 0.5 * (rect.min_x + rect.max_x) as f32;
        let y = 0.5 * (rect.min_y + rect.max_y) as f32;
        let pos = Vector::new(x, y);

        Camera { scale, pos }
    }

    pub(crate) fn point_to_screen_offset(&self, offset: Vector<f32>, point: IntPoint) -> Vector<f32> {
        let x = self.scale * (point.x as f32 - self.pos.x) + offset.x;
        let y = self.scale * (point.y as f32 - self.pos.y) + offset.y;
        Vector { x, y }
    }

    pub(crate) fn point_to_screen(&self, point: IntPoint) -> Vector<f32> {
        let x = self.scale * (point.x as f32 - self.pos.x);
        let y = self.scale * (point.y as f32 - self.pos.y);
        Vector { x, y }
    }

    // pub(super) fn point_to_world(&self, offset: Vector<f32>, point: Vector<f32>) -> IntPoint {
    //     let x = ((point.x - offset.x) / self.scale + self.pos.x).round() as i32;
    //     let y = ((point.y - offset.y) / self.scale + self.pos.y).round() as i32;
    //     IntPoint::new(x, y)
    // }

    pub(crate) fn distance_to_world(&self, distance: Vector<f32>) -> Vector<f32> {
        let x = distance.x / self.scale;
        let y = distance.y / self.scale;
        Vector { x, y }
    }

    // pub(crate) fn is_equal(&self, camera: Camera) -> bool {
    //     let same_scale = (camera.scale - self.scale).abs() < 0.000_0001;
    //     let same_pos = (camera.pos.x - self.pos.x).abs() < 0.000_0001 && (camera.pos.y - self.pos.y).abs() < 0.000_0001;
    //     same_scale && same_pos
    // }
}