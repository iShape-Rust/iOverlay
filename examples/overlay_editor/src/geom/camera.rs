use eframe::egui::Vec2;
use i_triangle::i_overlay::i_float::int::point::IntPoint;
use i_triangle::i_overlay::i_float::int::rect::IntRect;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Camera {
    pub(crate) scale: f32,
    pub(crate) i_scale: f32,
    pub(crate) size: Vec2,
    pub(crate) pos: Vec2,
}

impl Camera {
    pub(crate) fn empty() -> Self {
        Self {
            scale: 0.0,
            i_scale: 0.0,
            size: Vec2::ZERO,
            pos: Vec2::new(0.0, 0.0),
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.scale < 0.000_000_000_1
    }

    pub(crate) fn set_scale(&mut self, scale: f32) {
        self.scale = scale.clamp(1e-10, 1e8);
        self.i_scale = 1.0 / self.scale;
    }

    pub(crate) fn is_not_empty(&self) -> bool {
        self.scale > 0.0
    }

    pub(crate) fn new(rect: IntRect, size: Vec2) -> Self {
        let width = (rect.max_x as f64 - rect.min_x as f64).max(1.0) as f32;
        let height = (rect.max_y as f64 - rect.min_y as f64).max(1.0) as f32;
        let scale = (0.5 * (size.x / width).min(size.y / height)).max(1e-10);
        let i_scale = 1.0 / scale;
        let x = (0.5 * (rect.min_x as f64 + rect.max_x as f64)) as f32;
        let y = (0.5 * (rect.min_y as f64 + rect.max_y as f64)) as f32;
        let pos = Vec2::new(x, y);

        Camera {
            scale,
            i_scale,
            size,
            pos,
        }
    }

    #[inline]
    pub(crate) fn int_world_to_view(&self, world: IntPoint) -> Vec2 {
        self.world_to_view(Vec2::new(world.x as f32, world.y as f32))
    }

    #[inline]
    pub(crate) fn world_to_view(&self, world: Vec2) -> Vec2 {
        let x = self.scale * (world.x - self.pos.x) + 0.5 * self.size.x;
        let y = self.scale * (self.pos.y - world.y) + 0.5 * self.size.y;
        Vec2 { x, y }
    }

    #[inline]
    pub(crate) fn view_to_world(&self, view: Vec2) -> Vec2 {
        let x = self.i_scale * (view.x - 0.5 * self.size.x) + self.pos.x;
        let y = self.i_scale * (0.5 * self.size.y - view.y) + self.pos.y;
        Vec2 { x, y }
    }

    #[inline]
    pub(crate) fn view_distance_to_world(&self, view_distance: Vec2) -> Vec2 {
        let x = view_distance.x * self.i_scale;
        let y = -view_distance.y * self.i_scale;
        Vec2 { x, y }
    }
}

#[cfg(test)]
mod tests {
    use super::Camera;
    use eframe::egui::Vec2;
    use i_triangle::i_overlay::i_float::int::rect::IntRect;

    #[test]
    fn camera_supports_degenerate_bounds() {
        let rects = [
            IntRect::new(0, 10_000, 0, 0),
            IntRect::new(0, 0, -10_000, 10_000),
            IntRect::new(42, 42, 24, 24),
        ];

        for rect in rects {
            let camera = Camera::new(rect, Vec2::new(800.0, 600.0));
            assert!(camera.scale.is_finite());
            assert!(camera.scale > 0.0);
            assert!(camera.i_scale.is_finite());
        }
    }
}
