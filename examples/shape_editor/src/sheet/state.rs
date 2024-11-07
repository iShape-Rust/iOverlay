use iced::{Size, Vector};
use iced::mouse::ScrollDelta;
use crate::geom::camera::Camera;

struct Drag {
    start_screen: Vector<f32>,
    start_world: Vector<f32>,
}

enum DragState {
    Drag(Drag),
    None,
}

pub(super) struct SheetState {
    size: Size,
    drag_state: DragState,
}

impl SheetState {
    pub(super) fn is_size_changed(&mut self, size: Size) -> bool {
        let w = (size.width - self.size.width).abs();
        let h = (size.height - self.size.height).abs();
        let is_changed = w > 0.01 || h > 0.01;
        if is_changed {
            self.size = size
        }
        is_changed
    }

    pub(super) fn mouse_press(&mut self, camera: Camera, cursor: Vector<f32>) {
        self.drag_state = DragState::Drag(Drag { start_screen: cursor, start_world: camera.pos });
    }

    pub(super) fn mouse_release(&mut self) {
        self.drag_state = DragState::None;
    }

    pub(super) fn mouse_move(&mut self, camera: Camera, cursor: Vector<f32>) -> Option<Vector<f32>> {
        if let DragState::Drag(drag) = &self.drag_state {
            let translate = drag.start_screen - cursor;
            let world_dist = camera.distance_to_world(translate);
            let new_pos = Vector::new(
                drag.start_world.x + world_dist.x,
                drag.start_world.y + world_dist.y,
            );
            Some(new_pos)
        } else {
            None
        }
    }

    pub(super) fn mouse_wheel_scrolled(&mut self, camera: Camera, viewport_size: Size, delta: ScrollDelta) -> Option<f32> {
        if let ScrollDelta::Pixels { x: _ , y } = delta {
            let s = 1.0 + y / viewport_size.height;
            Some(s * camera.scale)
        } else {
            None
        }
    }
}

impl Default for SheetState {
    fn default() -> Self {
        Self {
            size: Size::ZERO,
            drag_state: DragState::None,
        }
    }
}

