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
    drag_state: DragState,
}

impl SheetState {
    pub(super) fn mouse_press(&mut self, camera: Camera, view_cursor: Vector<f32>) {
        self.drag_state = DragState::Drag(Drag { start_screen: view_cursor, start_world: camera.pos });
    }

    pub(super) fn mouse_release(&mut self) {
        self.drag_state = DragState::None;
    }

    pub(super) fn mouse_move(&mut self, camera: Camera, view_cursor: Vector<f32>) -> Option<Vector<f32>> {
        if let DragState::Drag(drag) = &self.drag_state {
            let translate = drag.start_screen - view_cursor;
            let world_dist = camera.view_distance_to_world(translate);
            let new_pos = Vector::new(
                drag.start_world.x + world_dist.x,
                drag.start_world.y + world_dist.y,
            );
            Some(new_pos)
        } else {
            None
        }
    }

    pub(super) fn mouse_wheel_scrolled(&mut self, camera: Camera, viewport_size: Size, delta: ScrollDelta, view_cursor: Vector<f32>) -> Option<Camera> {
        if let ScrollDelta::Pixels { x: _ , y } = delta {

            let s = 1.0 + y / viewport_size.height;
            let mut new_camera = camera;
            new_camera.set_scale(s * camera.scale);

            let world_pos = camera.view_to_world(view_cursor);
            let new_view_pos = new_camera.world_to_view(world_pos);

            let view_distance = view_cursor - new_view_pos;
            let world_distance = new_camera.view_distance_to_world(view_distance);

            new_camera.pos = new_camera.pos - world_distance;

            Some(new_camera)
        } else {
            None
        }
    }
}

impl Default for SheetState {
    fn default() -> Self {
        Self {
            drag_state: DragState::None,
        }
    }
}

