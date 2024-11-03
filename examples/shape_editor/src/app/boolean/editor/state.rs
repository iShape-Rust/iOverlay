use crate::util::point::EditorPoint;
use iced::{Rectangle, Transformation};
use i_triangle::i_overlay::i_float::float::point::FloatPoint;
use i_triangle::i_overlay::i_float::int::rect::IntRect;
use iced::advanced::graphics::color::Packed;
use iced::advanced::graphics::color::pack;
use iced::color;

use iced::advanced::graphics::Mesh;
use iced::advanced::graphics::mesh::{SolidVertex2D, Indexed};

#[derive(Debug, Clone, Copy)]
struct Camera {
    scale: f32,
    pos: FloatPoint<f32>,
}

pub(super) struct SubjClipEditorState {
    camera: Option<Camera>,
    viewport: Option<Rectangle>,
}

#[derive(Debug, Clone)]
pub(crate) enum PolygonEditorMessage {
    PointAdded(FloatPoint<f32>),    // Example of a point addition
    PointRemoved(usize),            // Example of point removal
}

impl Camera {
    fn transform_to_screen(&self, point: &FloatPoint<f32>) -> FloatPoint<f32> {
        let translated = *point - self.pos;
        let scaled = translated * self.scale;
        scaled
    }
}

impl SubjClipEditorState {
    pub(super) fn update_camera(&mut self, points: &[EditorPoint], viewport: &Rectangle) {
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

        let scale = sw.max(sh);
        let x = 0.5 * (rect.min_x + rect.max_x) as f32;
        let y = 0.5 * (rect.min_y + rect.max_y) as f32;
        let pos = FloatPoint::new(x, y);

        self.viewport = Some(viewport.clone());
        self.camera = Some(Camera { scale, pos });
    }

    pub(super) fn point_mesh(&self, points: &[EditorPoint]) -> Option<Mesh> {
        let camera = self.camera?;
        let viewport = self.viewport?;

        let count = points.len();
        let r = 5.0f32;
        let mut vertices = Vec::with_capacity(4 * count); //SolidVertex2D
        let mut indices = Vec::with_capacity(6 * count);
        let color = pack(color![0.0f32, 1.0, 0.0, 1.0]);
        for p in points.iter() {
            let x = (p.pos.x as f32) * camera.scale;
            let y = (p.pos.y as f32) * camera.scale;

            let i = vertices.len() as u32;

            vertices.push(SolidVertex2D { position: [x - r, y], color });
            vertices.push(SolidVertex2D { position: [x, y + r], color });
            vertices.push(SolidVertex2D { position: [x + r, y], color });
            vertices.push(SolidVertex2D { position: [x, y - r], color });

            indices.push(i);
            indices.push(i + 1);
            indices.push(i + 2);

            indices.push(i);
            indices.push(i + 2);
            indices.push(i + 3);
        }

        Some(Mesh::Solid {
            buffers: Indexed { vertices, indices },
            transformation: Transformation::IDENTITY,
            clip_bounds: viewport,
        })
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
}

impl Default for SubjClipEditorState {
    fn default() -> Self {
        Self {
            viewport: None,
            camera: None,
        }
    }
}