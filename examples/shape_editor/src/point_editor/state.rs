use crate::point_editor::point::EditorPoint;
use iced::{Color, Point, Rectangle, Transformation, Vector};
use i_triangle::i_overlay::i_float::float::point::FloatPoint;
use i_triangle::i_overlay::i_float::int::point::IntPoint;
use i_triangle::i_overlay::i_float::int::rect::IntRect;
use iced::advanced::graphics::color::Packed;
use iced::advanced::graphics::color::pack;
use iced::color;

use iced::advanced::graphics::Mesh;
use iced::advanced::graphics::mesh::{SolidVertex2D, Indexed};
use crate::app::main::AppMessage;
use crate::point_editor::widget::{PointEditUpdate, PointsEditorWidget};

#[derive(Debug, Clone, Copy)]
pub(super) struct Camera {
    pub(super) scale: f32,
    pub(super) pos: Vector<f32>,
}

#[derive(Clone)]
pub(super) struct MeshCache {
    radius: f32,
    pub(super) main: Mesh,
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
    pub(super) camera: Option<Camera>,
    pub(super) mesh_cache: Option<MeshCache>,
    pub(super) select: SelectState,
    viewport: Option<Rectangle>,
}

#[derive(Debug, Clone)]
pub(crate) enum PolygonEditorMessage {
    PointAdded(FloatPoint<f32>),    // Example of a point addition
    PointRemoved(usize),            // Example of point removal
}

impl Camera {
    pub(super) fn point_to_screen(&self, offset: Vector<f32>, point: IntPoint) -> Vector<f32> {
        let x = self.scale * (point.x as f32 - self.pos.x) + offset.x;
        let y = self.scale * (point.y as f32 - self.pos.y) + offset.y;
        Vector { x, y }
    }

    pub(super) fn point_to_world(&self, offset: Vector<f32>, point: Vector<f32>) -> IntPoint {
        let x = ((point.x - offset.x) / self.scale + self.pos.x).round() as i32;
        let y = ((point.y - offset.y) / self.scale + self.pos.y).round() as i32;
        IntPoint::new(x, y)
    }

    pub(super) fn distance_to_world(&self, distance: Vector<f32>) -> IntPoint {
        let x = (distance.x / self.scale).round() as i32;
        let y = (distance.y / self.scale).round() as i32;
        IntPoint::new(x, y)
    }
}

impl PointsEditorState {
    pub(crate) fn update_camera(&mut self, points: &[EditorPoint], viewport: &Rectangle) {
        if !self.is_need_update_camera(viewport) {
            return;
        }

        let rect = if let Some(rect) = IntRect::with_iter(points.iter().map(|p| &p.pos)) {
            rect
        } else {
            return;
        };

        let w_pow = rect.width().ilog2() as usize;
        let h_pow = rect.height().ilog2() as usize;

        let width = (1 << w_pow) as f32;
        let height = (1 << h_pow) as f32;
        let sw = viewport.width / width;
        let sh = viewport.height / height;

        let scale = 0.5 * sw.min(sh);
        let x = 0.5 * (rect.min_x + rect.max_x) as f32;
        let y = 0.5 * (rect.min_y + rect.max_y) as f32;
        let pos = Vector::new(x, y);

        self.viewport = Some(viewport.clone());
        self.camera = Some(Camera { scale, pos });
    }

    fn is_need_update_camera(&self, viewport: &Rectangle) -> bool {
        !self.is_same_viewport(viewport) || self.camera.is_none()
    }

    fn is_same_viewport(&self, viewport: &Rectangle) -> bool {
        let current = if let Some(viewport) = self.viewport {
            viewport
        } else {
            return false;
        };

        let is_width = (current.width - viewport.width).abs() < 0.1;
        let is_height = (current.height - viewport.height).abs() < 0.1;
        let is_x = (current.x - viewport.y).abs() < 0.1;
        let is_y = (current.y - viewport.y).abs() < 0.1;

        is_width && is_height && is_x && is_y
    }

    pub(crate) fn update_mesh(&mut self, r: f32, main_color: Color, hover_color: Color) {
        let radius = if let Some(cache) = &self.mesh_cache {
            cache.radius
        } else {
            0.0
        };

        if (radius - r).abs() < 0.1 {
            return;
        }

        let mut main_vertices = Vec::with_capacity(4);
        let mut hower_vertices = Vec::with_capacity(4);
        let mut indices = Vec::with_capacity(6);
        let main_pack = pack(main_color);
        let hover_pack = pack(hover_color);

        main_vertices.push(SolidVertex2D { position: [0.0, r], color: main_pack });
        main_vertices.push(SolidVertex2D { position: [r, 2.0 * r], color: main_pack });
        main_vertices.push(SolidVertex2D { position: [2.0 * r, r], color: main_pack });
        main_vertices.push(SolidVertex2D { position: [r, 0.0], color: main_pack });

        hower_vertices.push(SolidVertex2D { position: [0.0, r], color: hover_pack });
        hower_vertices.push(SolidVertex2D { position: [r, 2.0 * r], color: hover_pack });
        hower_vertices.push(SolidVertex2D { position: [2.0 * r, r], color: hover_pack });
        hower_vertices.push(SolidVertex2D { position: [r, 0.0], color: hover_pack });

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
                buffers: Indexed { vertices: hower_vertices, indices: indices },
                transformation: Transformation::IDENTITY,
                clip_bounds: Rectangle::INFINITE,
            },
        });
    }

    pub(super) fn mouse_press<'a, M>(&mut self, widget: &PointsEditorWidget<'a, M>, cursor: Vector<f32>, offset: Vector<f32>) {
        let camera = if let Some(camera) = self.camera { camera } else { return; };

        let mut min_ds = widget.radius * widget.radius;
        let mut min_index = usize::MAX;
        for (i, p) in widget.points.iter().enumerate() {
            let screen = camera.point_to_screen(offset, p.pos);
            let ds = Self::sqr_length(&cursor, &screen);
            if ds <= min_ds {
                min_ds = ds;
                min_index = i;
            }
        }

        if min_index != usize::MAX {
            self.select = SelectState::Drag(Drag {
                index: min_index,
                start_float: cursor,
                editor_point: widget.points[min_index].clone(),
            });
        }
    }

    pub(super) fn mouse_release<'a, M>(&mut self, widget: &PointsEditorWidget<M>, cursor: Vector<f32>, offset: Vector<f32>) {
        if let SelectState::Drag(_) = &self.select {
            self.select = SelectState::None;
            let camera = if let Some(camera) = self.camera { camera } else { return; };
            self.mouse_hower(&camera, widget.radius, &widget.points, cursor, offset);
        }
    }

    pub(super) fn mouse_move<'a, M>(&mut self, widget: &PointsEditorWidget<M>, cursor: Vector<f32>, offset: Vector<f32>) -> Option<PointEditUpdate> {
        let camera = if let Some(camera) = self.camera { camera } else { return None; };
        if let SelectState::Drag(drag) = &self.select {
            Self::mouse_drag(drag, &camera, &widget.points, cursor)
        } else {
            self.mouse_hower(&camera, widget.radius, &widget.points, cursor, offset);
            None
        }
    }

    fn mouse_drag(drag: &Drag, camera: &Camera, points: &Vec<EditorPoint>, cursor: Vector<f32>) -> Option<PointEditUpdate> {
        let translate = cursor - drag.start_float;
        let world_dist = camera.distance_to_world(translate);
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

    fn mouse_hower(&mut self, camera: &Camera, radius: f32, points: &Vec<EditorPoint>, cursor: Vector<f32>, offset: Vector<f32>) {
        let mut min_ds = radius * radius;
        let mut min_index = usize::MAX;
        for (i, p) in points.iter().enumerate() {
            let screen = camera.point_to_screen(offset, p.pos);
            let ds = Self::sqr_length(&cursor, &screen);
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
            viewport: None,
            mesh_cache: None,
            camera: None,
        }
    }
}