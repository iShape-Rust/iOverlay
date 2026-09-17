use crate::draw::path::{arrow_shape, stroke_path};
use crate::geom::camera::Camera;
use eframe::egui::{Color32, Painter, Shape, Stroke, Vec2};
use i_triangle::i_overlay::vector::edge::{
    DataVectorEdge, CLIP_LEFT, CLIP_RIGHT, SUBJ_LEFT, SUBJ_RIGHT,
};

pub(crate) struct VectorsWidget {
    shapes: Vec<Shape>,
}

impl VectorsWidget {
    pub(crate) fn with_vectors(
        vectors: &[DataVectorEdge<i32>],
        camera: Camera,
        subj: Color32,
        clip: Color32,
        both: Color32,
        width: f32,
    ) -> Self {
        let mut shapes = Vec::new();
        for vector in vectors {
            let a = camera.int_world_to_view(vector.a).to_pos2();
            let b = camera.int_world_to_view(vector.b).to_pos2();
            if (b - a).length_sq() < 1e-10 {
                continue;
            }
            let fill = vector.fill;
            let color = match (
                fill & (SUBJ_LEFT | SUBJ_RIGHT) != 0,
                fill & (CLIP_LEFT | CLIP_RIGHT) != 0,
            ) {
                (true, true) => both,
                (true, false) => subj,
                (false, true) => clip,
                _ => Color32::GRAY,
            };
            let stroke = Stroke::new(width, color);
            if let Some(line) = stroke_path(vec![a, b], false, stroke) {
                shapes.push(line);
            }
            if let Some(arrow) = arrow_shape(a, b, b, stroke) {
                shapes.push(arrow);
            }
            let n = (b - a).normalized() * (4.0 * width);
            let side = Vec2::new(-n.y, n.x);
            let mid = a.lerp(b, 0.5);
            for (pos, mask, color) in [
                (mid + side + n, SUBJ_RIGHT, subj),
                (mid - side + n, SUBJ_LEFT, subj),
                (mid + side - n, CLIP_RIGHT, clip),
                (mid - side - n, CLIP_LEFT, clip),
            ] {
                shapes.push(Shape::circle_filled(
                    pos,
                    2.0 * width,
                    if fill & mask != 0 {
                        color
                    } else {
                        color.gamma_multiply(0.05)
                    },
                ));
            }
        }
        Self { shapes }
    }

    pub(crate) fn paint(self, painter: &Painter) {
        for mut shape in self.shapes {
            shape.translate(painter.clip_rect().min.to_vec2());
            painter.add(shape);
        }
    }
}
