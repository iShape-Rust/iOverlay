use crate::{draw::shape::ShapeWidget, geom::camera::Camera};
use eframe::egui::{Color32, Painter};
use i_triangle::i_overlay::i_shape::int::shape::IntShapes;

pub(crate) struct VaricoloredWidget {
    shapes: Vec<ShapeWidget>,
}

impl VaricoloredWidget {
    const COLORS: [[u8; 3]; 12] = [
        [255, 149, 0],
        [88, 86, 214],
        [255, 45, 85],
        [90, 200, 250],
        [76, 217, 100],
        [255, 204, 0],
        [142, 142, 147],
        [255, 59, 48],
        [52, 199, 89],
        [0, 122, 255],
        [175, 82, 222],
        [255, 214, 10],
    ];
    pub(crate) fn with_shapes(shapes: &IntShapes<i32>, camera: Camera, width: f32) -> Self {
        Self {
            shapes: shapes
                .iter()
                .enumerate()
                .map(|(index, paths)| {
                    let [r, g, b] = Self::COLORS[index % Self::COLORS.len()];
                    let color = Color32::from_rgb(r, g, b);
                    ShapeWidget::with_paths(
                        paths,
                        camera,
                        None,
                        Some(color.gamma_multiply(0.2)),
                        Some(color),
                        width,
                    )
                })
                .collect(),
        }
    }
    pub(crate) fn paint(self, painter: &Painter) {
        for shape in self.shapes {
            shape.paint(painter);
        }
    }
}
