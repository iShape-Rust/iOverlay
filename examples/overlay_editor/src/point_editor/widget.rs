use crate::{
    app::design::Design,
    geom::camera::Camera,
    point_editor::{point::EditorPoint, state::PointsEditorState},
};
use eframe::egui::{Painter, Shape, Stroke, Vec2};

#[derive(Debug, Clone)]
pub(crate) struct PointEditUpdate {
    pub(crate) index: usize,
    pub(crate) point: EditorPoint,
}

pub(crate) struct PointsEditorWidget;

impl PointsEditorWidget {
    pub(crate) fn paint(
        painter: &Painter,
        camera: Camera,
        points: &[EditorPoint],
        state: &PointsEditorState,
    ) {
        for (index, point) in points.iter().enumerate() {
            let selected = state.selected() == Some(index);
            let color = if selected && state.is_dragging() {
                Design::accent_color()
            } else if selected {
                Design::negative_color()
            } else {
                Design::subject_color()
            };
            let radius = if selected { 5.0 } else { 4.0 };
            let p = painter.clip_rect().min + camera.int_world_to_view(point.pos);
            painter.add(Shape::convex_polygon(
                vec![
                    p + Vec2::new(-radius, 0.0),
                    p + Vec2::new(0.0, -radius),
                    p + Vec2::new(radius, 0.0),
                    p + Vec2::new(0.0, radius),
                ],
                color,
                Stroke::NONE,
            ));
        }
    }
}
