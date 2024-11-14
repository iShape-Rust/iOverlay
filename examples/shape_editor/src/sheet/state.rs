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

    pub(super) fn mouse_wheel_scrolled(&mut self, camera: Camera, viewport_size: Size, delta: ScrollDelta, view_cursor: Vector<f32>) -> Option<Camera> {
        if let ScrollDelta::Pixels { x: _ , y } = delta {

            let s = 1.0 + y / viewport_size.height;
            let mut new_camera = camera;
            new_camera.set_scale(s * camera.scale);

            let start_world = camera.view_to_world(view_cursor);
            let end_world = new_camera.view_to_world(view_cursor);

            let diff = start_world - end_world;
            new_camera.pos = new_camera.pos - diff;

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

