use iced::Rectangle;
use i_triangle::i_overlay::i_float::float::point::FloatPoint;
use i_triangle::i_overlay::i_float::float::rect::FloatRect;

#[derive(Debug, Clone)]
pub(in crate::app) struct MultiIndex {
    first: usize,   // point index
    second: usize,  // path index
    third: usize,   // shape index
    forth: usize,   // subj or clip index
}

#[derive(Debug, Clone)]
pub(in crate::app) struct EditorPoint {
    pos: FloatPoint<f32>,
    index: MultiIndex
}

#[derive(Debug, Clone)]
pub(in crate::app) struct Camera {
    scale: f32,
    pos: FloatPoint<f32>
}

pub(in crate::app) struct PolygonEditorWidgetState {
    pub(in crate::app) points: Vec<EditorPoint>,
    pub(in crate::app) camera: Option<Camera>
}

#[derive(Debug, Clone)]
pub(in crate::app) enum PolygonEditorMessage {
    CameraReady(Camera),
    PointAdded(FloatPoint<f32>), // Example of a point addition
    PointRemoved(usize),          // Example of point removal
}

impl Camera {

    fn transform_to_screen(&self, point: &FloatPoint<f32>) -> FloatPoint<f32> {
        let translated = *point - self.pos;
        let scaled = translated * self.scale;
        scaled
    }

}

impl PolygonEditorWidgetState {
    pub(in crate::app) fn init_camera(&mut self, viewport: &Rectangle) -> Option<Camera> {
        if !self.camera.is_none() || self.points.is_empty() {
            return None;
        }

        let rect = FloatRect::with_iter(self.points.iter().map(|p|&p.pos))?;

        let w_pow = (rect.width().log2() + 0.5).round() as usize;
        let h_pow = (rect.height().log2() + 0.5).round() as usize;

        let width = (1 << w_pow) as f32;
        let height = (1 << h_pow) as f32;
        let sw = viewport.width / width;
        let sh = viewport.height / height;

        let scale = sw.max(sh);
        let x = 0.5 * (rect.min_x + rect.max_x) as f32;
        let y = 0.5 * (rect.min_y + rect.max_y) as f32;
        let pos = FloatPoint::new(x, y);

        Some(Camera { scale, pos })
    }

    pub(in crate::app) fn update_camera(&mut self, viewport: &Rectangle) {

    }
}

impl Default for PolygonEditorWidgetState {
    fn default() -> Self {
        Self {
            camera: None,
            points: vec![],
        }
    }
}