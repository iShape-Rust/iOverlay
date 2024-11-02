use crate::util::point::EditorPoint;
use iced::Rectangle;
use i_triangle::i_overlay::i_float::float::point::FloatPoint;
use i_triangle::i_overlay::i_float::int::rect::IntRect;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Camera {
    scale: f32,
    pos: FloatPoint<f32>
}

pub(super) struct SubjClipEditorState {
    pub(super) camera: Option<Camera>
}

#[derive(Debug, Clone)]
pub(crate) enum PolygonEditorMessage {
    CameraReady(Camera),
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
    pub(super) fn init_camera(&mut self, points: &[EditorPoint], viewport: &Rectangle) -> Option<Camera> {
        if !self.camera.is_none() {
            return self.camera;
        }

        if points.is_empty() {
            return None;
        }

        let rect = IntRect::with_iter(points.iter().map(|p|&p.pos))?;

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

        self.camera = Some(Camera { scale, pos });

        self.camera
    }

    pub(super) fn update_camera(&mut self, _viewport: &Rectangle) {

    }
}

impl Default for SubjClipEditorState {
    fn default() -> Self {
        Self {
            camera: None
        }
    }
}