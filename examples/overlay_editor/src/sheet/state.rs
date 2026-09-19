use crate::geom::camera::Camera;
use eframe::egui::Vec2;

#[derive(Default)]
pub(crate) struct SheetState {
    drag: Option<(Vec2, Vec2)>,
}

impl SheetState {
    pub(crate) fn press(&mut self, camera: Camera, cursor: Vec2) {
        self.drag = Some((cursor, camera.pos));
    }
    pub(crate) fn release(&mut self) {
        self.drag = None;
    }
    pub(crate) fn drag(&self, camera: &mut Camera, cursor: Vec2) {
        if let Some((start, pos)) = self.drag {
            camera.pos = pos + camera.view_distance_to_world(start - cursor);
        }
    }
    pub(crate) fn zoom(camera: &mut Camera, cursor: Vec2, delta: f32) {
        let world = camera.view_to_world(cursor);
        camera.set_scale(camera.scale * (delta * 0.002).exp());
        camera.pos += world - camera.view_to_world(cursor);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use i_triangle::i_overlay::i_float::int::rect::IntRect;
    #[test]
    fn zoom_preserves_world_position_under_pointer() {
        let mut camera = Camera::new(IntRect::new(-100, 100, -100, 100), Vec2::new(800.0, 600.0));
        let cursor = Vec2::new(137.0, 251.0);
        let before = camera.view_to_world(cursor);
        SheetState::zoom(&mut camera, cursor, 120.0);
        assert!((camera.view_to_world(cursor) - before).length() < 1e-4);
        SheetState::zoom(&mut camera, cursor, -1e6);
        assert!(camera.scale > 0.0 && camera.i_scale.is_finite());
    }
}
