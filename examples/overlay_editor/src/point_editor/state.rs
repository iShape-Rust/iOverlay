use crate::point_editor::point::EditorPoint;
use iced::{Color, Rectangle, Transformation, Vector};
use iced::advanced::graphics::color::pack;

use iced::advanced::graphics::Mesh;
use iced::advanced::graphics::mesh::{SolidVertex2D, Indexed};
use crate::point_editor::widget::{PointEditUpdate, PointsEditorWidget};
use crate::geom::camera::Camera;
use crate::geom::vector::VectorExt;

#[derive(Clone)]
pub(super) struct MeshCache {
    radius: f32,
    pub(super) main: Mesh,
    pub(super) drag: Mesh,
    pub(super) hover: Mesh,
}

pub(super) enum SelectState {
    Hover(usize),
    Drag(Drag),
    None,
}

pub(super) struct Drag {
    pub(super) index: usize,
    editor_point: EditorPoint,
    start_float: Vector<f32>,
}

pub(crate) struct PointsEditorState {
    pub(super) mesh_cache: Option<MeshCache>,
    pub(super) select: SelectState,
}

impl PointsEditorState {
    pub(crate) fn update_mesh(&mut self, r: f32, main_color: Color, hover_color: Color, drag_color: Color) {
        let radius = if let Some(cache) = &self.mesh_cache {
            cache.radius
        } else {
            0.0
        };

        if (radius - r).abs() < 0.1 {
            return;
        }

        let sr = 1.2 * r;

        let mut main_vertices = Vec::with_capacity(4);
        let mut hover_vertices = Vec::with_capacity(4);
        let mut drag_vertices = Vec::with_capacity(4);
        let mut indices = Vec::with_capacity(6);
        let main_pack = pack(main_color);
        let hover_pack = pack(hover_color);
        let drag_pack = pack(drag_color);

        main_vertices.push(SolidVertex2D { position: [0.0, r], color: main_pack });
        main_vertices.push(SolidVertex2D { position: [r, 2.0 * r], color: main_pack });
        main_vertices.push(SolidVertex2D { position: [2.0 * r, r], color: main_pack });
        main_vertices.push(SolidVertex2D { position: [r, 0.0], color: main_pack });

        hover_vertices.push(SolidVertex2D { position: [0.0, sr], color: hover_pack });
        hover_vertices.push(SolidVertex2D { position: [r, 2.0 * sr], color: hover_pack });
        hover_vertices.push(SolidVertex2D { position: [2.0 * sr, sr], color: hover_pack });
        hover_vertices.push(SolidVertex2D { position: [sr, 0.0], color: hover_pack });

        drag_vertices.push(SolidVertex2D { position: [0.0, r], color: drag_pack });
        drag_vertices.push(SolidVertex2D { position: [r, 2.0 * r], color: drag_pack });
        drag_vertices.push(SolidVertex2D { position: [2.0 * r, r], color: drag_pack });
        drag_vertices.push(SolidVertex2D { position: [r, 0.0], color: drag_pack });

        indices.push(0);
        indices.push(1);
        indices.push(2);

        indices.push(0);
        indices.push(2);
        indices.push(3);

        self.mesh_cache = Some(MeshCache {
            radius: r,
            main: Mesh::Solid {
                buffers: Indexed { vertices: main_vertices, indices: indices.clone() },
                transformation: Transformation::IDENTITY,
                clip_bounds: Rectangle::INFINITE,
            },
            hover: Mesh::Solid {
                buffers: Indexed { vertices: hover_vertices, indices: indices.clone() },
                transformation: Transformation::translate(r - sr, r - sr),
                clip_bounds: Rectangle::INFINITE,
            },
            drag: Mesh::Solid {
                buffers: Indexed { vertices: drag_vertices, indices },
                transformation: Transformation::IDENTITY,
                clip_bounds: Rectangle::INFINITE,
            },
        });
    }

    pub(super) fn mouse_press<M>(&mut self, widget: &PointsEditorWidget<M>, cursor: Vector<f32>) -> bool {
        let mut min_ds = widget.hover_radius * widget.hover_radius;
        let mut min_index = usize::MAX;
        // println!("cursor: {:?}", &cursor);
        for (i, p) in widget.points.iter().enumerate() {
            let view_pos = widget.camera.int_world_to_view(p.pos);
            // println!("screen_pos: {:?}", &screen_pos);
            let ds = Self::sqr_length(&cursor, &view_pos);
            if ds <= min_ds {
                min_ds = ds;
                min_index = i;
            }
        }

        let is_catch = min_index != usize::MAX;

        if is_catch {
            self.select = SelectState::Drag(Drag {
                index: min_index,
                start_float: cursor,
                editor_point: widget.points[min_index].clone(),
            });
        }

        is_catch
    }

    pub(super) fn mouse_release<M>(&mut self, widget: &PointsEditorWidget<M>, cursor: Vector<f32>) -> bool {
        if let SelectState::Drag(_) = &self.select {
            self.select = SelectState::None;
            self.mouse_hover(widget.camera, widget.hover_radius, widget.points, cursor);
            true
        } else {
            false
        }
    }

    pub(super) fn mouse_move<M>(&mut self, widget: &PointsEditorWidget<M>, cursor: Vector<f32>) -> Option<PointEditUpdate> {
        if let SelectState::Drag(drag) = &self.select {
            Self::mouse_drag(drag, widget.camera, widget.points, cursor)
        } else {
            self.mouse_hover(widget.camera, widget.hover_radius, widget.points, cursor);
            None
        }
    }

    fn mouse_drag(drag: &Drag, camera: Camera, points: &[EditorPoint], cursor: Vector<f32>) -> Option<PointEditUpdate> {
        let translate = cursor - drag.start_float;
        let world_dist = camera.view_distance_to_world(translate).round();
        let world_point = world_dist + drag.editor_point.pos;
        let real_point = &points[drag.index];
        if world_point != real_point.pos {
            return Some(PointEditUpdate {
                index: drag.index,
                point: EditorPoint { pos: world_point, index: drag.editor_point.index.clone() },
            });
        }

        None
    }

    fn mouse_hover(&mut self, camera: Camera, radius: f32, points: &[EditorPoint], cursor: Vector<f32>) {
        let mut min_ds = radius * radius;
        let mut min_index = usize::MAX;
        for (i, p) in points.iter().enumerate() {
            let view_pos = camera.int_world_to_view(p.pos);
            let ds = Self::sqr_length(&cursor, &view_pos);
            if ds <= min_ds {
                min_ds = ds;
                min_index = i;
            }
        }

        if min_index == usize::MAX {
            self.select = SelectState::None;
        } else {
            self.select = SelectState::Hover(min_index);
        }
    }

    fn sqr_length(a: &Vector, b: &Vector) -> f32 {
        let dx = a.x - b.x;
        let dy = a.y - b.y;
        dx * dx + dy * dy
    }
}

impl Default for PointsEditorState {
    fn default() -> Self {
        Self {
            select: SelectState::None,
            mesh_cache: None,
        }
    }
}