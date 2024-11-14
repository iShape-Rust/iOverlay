use i_triangle::i_overlay::i_float::int::point::IntPoint;
use i_triangle::i_overlay::i_float::int::rect::IntRect;
use iced::{Size, Vector};

#[derive(Debug, Clone, Copy)]
pub(crate) struct Camera {
    pub(crate) scale: f32,
    pub(crate) i_scale: f32,
    pub(crate) size: Size,
    pub(crate) pos: Vector<f32>,
}

impl Camera {

    pub(crate) fn empty() -> Self {
        Self { scale: 0.0, i_scale: 0.0, size: Size::ZERO, pos: Vector::new(0.0 ,0.0)  }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.scale < 0.000_000_000_1
    }

    pub(crate) fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
        self.i_scale = 1.0 / scale;
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
        let i_scale = 1.0 / scale;
        let x = 0.5 * (rect.min_x + rect.max_x) as f32;
        let y = 0.5 * (rect.min_y + rect.max_y) as f32;
        let pos = Vector::new(x, y);

        Camera { scale, i_scale, size, pos }
    }

    #[inline]
    pub(crate) fn world_to_screen(&self, view_left_top: Vector<f32>, world: IntPoint) -> Vector<f32> {
        let x = self.scale * (world.x as f32 - self.pos.x) + view_left_top.x + 0.5 * self.size.width;
        let y = self.scale * (self.pos.y - world.y as f32) + view_left_top.y + 0.5 * self.size.height;
        Vector { x, y }
    }

    #[inline]
    pub(crate) fn int_world_to_view(&self, world: IntPoint) -> Vector<f32> {
        self.world_to_view(Vector::new(world.x as f32, world.y as f32))
    }

    #[inline]
    pub(crate) fn world_to_view(&self, world: Vector<f32>) -> Vector<f32> {
        let x = self.scale * (world.x - self.pos.x) + 0.5 * self.size.width;
        let y = self.scale * (self.pos.y - world.y) + 0.5 * self.size.height;
        Vector { x, y }
    }

    #[inline]
    pub(crate) fn view_to_world(&self, view: Vector<f32>) -> Vector<f32> {
        let x = self.i_scale * (view.x - 0.5 * self.size.width) + self.pos.x;
        let y = self.i_scale * (0.5 * self.size.height - view.y) + self.pos.y;
        Vector { x, y }
    }

    #[inline]
    pub(crate) fn view_distance_to_world(&self, view_distance: Vector<f32>) -> Vector<f32> {
        let x = view_distance.x * self.i_scale;
        let y = -view_distance.y * self.i_scale;
        Vector { x, y }
    }
}