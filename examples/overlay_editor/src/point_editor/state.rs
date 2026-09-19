use crate::{
    geom::{camera::Camera, vector::round_to_int},
    point_editor::{point::EditorPoint, widget::PointEditUpdate},
};
use eframe::egui::Vec2;

pub(crate) struct Drag {
    index: usize,
    point: EditorPoint,
    start: Vec2,
}

#[derive(Default)]
pub(crate) struct PointsEditorState {
    pub(crate) hover: Option<usize>,
    drag: Option<Drag>,
}

impl PointsEditorState {
    pub(crate) fn is_dragging(&self) -> bool {
        self.drag.is_some()
    }
    pub(crate) fn selected(&self) -> Option<usize> {
        self.drag.as_ref().map(|d| d.index).or(self.hover)
    }
    pub(crate) fn hover(&mut self, camera: Camera, points: &[EditorPoint], cursor: Vec2) {
        let mut distance = 12.0_f32.powi(2);
        self.hover = None;
        for (index, point) in points.iter().enumerate() {
            let d = (camera.int_world_to_view(point.pos) - cursor).length_sq();
            if d <= distance {
                distance = d;
                self.hover = Some(index);
            }
        }
    }
    pub(crate) fn press(&mut self, camera: Camera, points: &[EditorPoint], cursor: Vec2) -> bool {
        self.hover(camera, points, cursor);
        self.drag = self.hover.map(|index| Drag {
            index,
            point: points[index].clone(),
            start: cursor,
        });
        self.is_dragging()
    }
    pub(crate) fn release(&mut self) {
        self.drag = None;
    }
    pub(crate) fn drag(
        &self,
        camera: Camera,
        points: &[EditorPoint],
        cursor: Vec2,
    ) -> Option<PointEditUpdate> {
        let drag = self.drag.as_ref()?;
        let delta = round_to_int(camera.view_distance_to_world(cursor - drag.start));
        let pos = i_triangle::i_overlay::i_float::int::point::IntPoint::new(
            drag.point.pos.x.saturating_add(delta.x),
            drag.point.pos.y.saturating_add(delta.y),
        );
        if points.get(drag.index)?.pos == pos {
            return None;
        }
        Some(PointEditUpdate {
            index: drag.index,
            point: EditorPoint {
                pos,
                index: drag.point.index.clone(),
            },
        })
    }
}
