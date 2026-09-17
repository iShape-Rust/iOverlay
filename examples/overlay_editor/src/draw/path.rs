use crate::geom::camera::Camera;
use eframe::egui::{Color32, Mesh, Painter, Pos2, Shape, Stroke, Vec2};
use i_mesh::path::{round::RoundStrokeBuilder, style::StrokeStyle};
use i_triangle::i_overlay::i_shape::int::path::IntPaths;

pub(crate) struct PathWidget {
    shapes: Vec<Shape>,
}

impl PathWidget {
    pub(crate) fn with_paths(
        paths: &IntPaths<i32>,
        camera: Camera,
        color: Color32,
        width: f32,
        arrows: bool,
    ) -> Self {
        let stroke = Stroke::new(width, color);
        let mut shapes = Vec::new();
        for path in paths {
            let points: Vec<_> = path
                .iter()
                .map(|&p| camera.int_world_to_view(p).to_pos2())
                .collect();
            if arrows {
                for segment in points.windows(2) {
                    if let Some(arrow) = arrow_shape(
                        segment[0],
                        segment[1],
                        segment[0].lerp(segment[1], 0.5),
                        stroke,
                    ) {
                        shapes.push(arrow);
                    }
                }
            }
            if let Some(shape) = stroke_path(points, false, stroke) {
                shapes.push(shape);
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

pub(super) fn arrow_shape(a: Pos2, b: Pos2, tip: Pos2, stroke: Stroke) -> Option<Shape> {
    let delta = b - a;
    if delta.length_sq() < 1e-10 {
        return None;
    }
    let n = delta.normalized();
    let base = tip - n * (4.0 * stroke.width);
    let side = Vec2::new(-n.y, n.x) * (2.0 * stroke.width);
    stroke_path(vec![base + side, tip, base - side], false, stroke)
}

/// Build screen-space round joins/caps without changing editable geometry.
pub(super) fn stroke_path(mut points: Vec<Pos2>, closed: bool, stroke: Stroke) -> Option<Shape> {
    points.dedup();
    if closed && points.first() == points.last() {
        points.pop();
    }
    if points.len() < 2 || stroke.is_empty() {
        return None;
    }

    let path: Vec<[f32; 2]> = points.iter().map(|p| [p.x, p.y]).collect();
    let builder = RoundStrokeBuilder::new(StrokeStyle::with_width(stroke.width));
    let triangulation = if closed {
        builder.build_closed_path_mesh::<u32>(&path)
    } else {
        builder.build_open_path_mesh::<u32>(&path)
    };
    let mut mesh = Mesh::default();
    for [x, y] in triangulation.points {
        mesh.colored_vertex(Pos2::new(x, y), stroke.color);
    }
    mesh.indices = triangulation.indices;
    Some(Shape::mesh(mesh))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn degenerate_strokes_tessellate_to_finite_vertices() {
        let ctx = eframe::egui::Context::default();
        let a = Pos2::new(10.0, 10.0);
        let b = Pos2::new(20.0, 10.0);
        let c = Pos2::new(30.0, 10.0);
        for points in [
            vec![],
            vec![a],
            vec![a, a, a],
            vec![a, b],
            vec![a, a, b, b, b, c, a],
            vec![a, b, a, c],
        ] {
            for closed in [false, true] {
                let output = ctx.run_ui(Default::default(), |ui| {
                    if let Some(shape) =
                        stroke_path(points.clone(), closed, Stroke::new(2.0_f32, Color32::WHITE))
                    {
                        ui.painter().add(shape);
                    }
                });
                for primitive in ctx.tessellate(output.shapes, output.pixels_per_point) {
                    if let eframe::egui::epaint::Primitive::Mesh(mesh) = primitive.primitive {
                        assert!(mesh.vertices.iter().all(|v| v.pos.is_finite()));
                    }
                }
            }
        }
    }
    #[test]
    fn round_strokes_stay_within_half_width_of_path_bounds() {
        let width = 4.0;
        for closed in [false, true] {
            // Nearly reversing at the tip previously produced long miter spikes.
            let points = vec![
                Pos2::new(100.0, 100.0),
                Pos2::new(300.0, 100.0),
                Pos2::new(100.0, 100.4),
            ];
            let bounds = eframe::egui::Rect::from_points(&points).expand(width * 0.5 + 0.001);
            let Shape::Mesh(mesh) =
                stroke_path(points, closed, Stroke::new(width, Color32::WHITE)).unwrap()
            else {
                panic!("expected stroke mesh");
            };
            assert!(mesh.is_valid());
            assert!(!mesh.indices.is_empty());
            assert!(mesh.vertices.iter().all(|v| bounds.contains(v.pos)));
        }
    }

    #[test]
    fn open_stroke_has_round_caps() {
        let Shape::Mesh(mesh) = stroke_path(
            vec![Pos2::new(10.0, 10.0), Pos2::new(30.0, 10.0)],
            false,
            Stroke::new(4.0_f32, Color32::WHITE),
        )
        .unwrap() else {
            panic!("expected stroke mesh");
        };
        assert!(mesh.vertices.iter().any(|v| v.pos.x < 8.1));
        assert!(mesh.vertices.iter().any(|v| v.pos.x > 31.9));
        assert!(mesh.vertices.iter().all(|v| {
            let nearest = Pos2::new(v.pos.x.clamp(10.0, 30.0), 10.0);
            v.pos.distance(nearest) <= 2.001
        }));
    }
}
