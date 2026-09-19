use crate::geom::camera::Camera;
use eframe::egui::{Color32, Mesh, Painter, Shape, Stroke};
use i_triangle::i_overlay::core::fill_rule::FillRule;
use i_triangle::i_overlay::i_shape::int::{count::PointsCount, path::IntPaths, shape::IntShapes};
use i_triangle::int::{
    triangulation::IntTriangulation, triangulator::IntTriangulator, validation::Validation,
};

pub(crate) struct ShapeWidget {
    fill: Option<Mesh>,
    strokes: Vec<Shape>,
}

impl ShapeWidget {
    pub(crate) fn with_shapes(
        shapes: &IntShapes<i32>,
        camera: Camera,
        fill_rule: Option<FillRule>,
        fill_color: Option<Color32>,
        stroke_color: Option<Color32>,
        stroke_width: f32,
    ) -> Self {
        let fill = fill_color.filter(|_| !shapes.is_empty()).map(|color| {
            let triangulation = IntTriangulator::new(
                shapes.points_count(),
                Validation::with_fill_rule(fill_rule.unwrap_or_default()),
                Default::default(),
            )
            .triangulate_shapes(shapes);
            Self::fill_mesh(triangulation, camera, color)
        });
        let strokes = shapes
            .iter()
            .flat_map(|paths| Self::strokes(paths, camera, stroke_color, stroke_width))
            .collect();
        Self { fill, strokes }
    }

    pub(crate) fn with_paths(
        paths: &IntPaths<i32>,
        camera: Camera,
        fill_rule: Option<FillRule>,
        fill_color: Option<Color32>,
        stroke_color: Option<Color32>,
        stroke_width: f32,
    ) -> Self {
        let fill = fill_color.filter(|_| !paths.is_empty()).map(|color| {
            let triangulation = IntTriangulator::new(
                paths.points_count(),
                Validation::with_fill_rule(fill_rule.unwrap_or_default()),
                Default::default(),
            )
            .triangulate_shape(paths);
            Self::fill_mesh(triangulation, camera, color)
        });
        Self {
            fill,
            strokes: Self::strokes(paths, camera, stroke_color, stroke_width),
        }
    }

    fn fill_mesh(
        triangulation: IntTriangulation<i32, usize>,
        camera: Camera,
        color: Color32,
    ) -> Mesh {
        let mut mesh = Mesh::default();
        for p in triangulation.points {
            mesh.colored_vertex(camera.int_world_to_view(p).to_pos2(), color);
        }
        mesh.indices = triangulation
            .indices
            .into_iter()
            .map(|i| i as u32)
            .collect();
        mesh
    }

    fn strokes(
        paths: &IntPaths<i32>,
        camera: Camera,
        color: Option<Color32>,
        width: f32,
    ) -> Vec<Shape> {
        let Some(color) = color else {
            return Vec::new();
        };
        paths
            .iter()
            .filter_map(|path| {
                crate::draw::path::stroke_path(
                    path.iter()
                        .map(|&p| camera.int_world_to_view(p).to_pos2())
                        .collect(),
                    true,
                    Stroke::new(width, color),
                )
            })
            .collect()
    }

    pub(crate) fn paint(self, painter: &Painter) {
        let offset = painter.clip_rect().min.to_vec2();
        if let Some(mut mesh) = self.fill {
            mesh.translate(offset);
            painter.add(mesh);
        }
        for mut shape in self.strokes {
            shape.translate(offset);
            painter.add(shape);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use eframe::egui::Vec2;
    use i_triangle::i_overlay::i_float::int::{point::IntPoint, rect::IntRect};

    #[test]
    fn fill_preserves_holes_and_strokes_are_stroke_paths() {
        let paths = vec![
            vec![
                IntPoint::new(0, 0),
                IntPoint::new(0, 100),
                IntPoint::new(100, 100),
                IntPoint::new(100, 0),
            ],
            vec![
                IntPoint::new(25, 25),
                IntPoint::new(75, 25),
                IntPoint::new(75, 75),
                IntPoint::new(25, 75),
            ],
        ];
        let camera = Camera::new(IntRect::new(0, 100, 0, 100), Vec2::new(200.0, 200.0));
        let widget = ShapeWidget::with_paths(
            &paths,
            camera,
            Some(FillRule::EvenOdd),
            Some(Color32::WHITE),
            Some(Color32::RED),
            2.0,
        );
        let mesh = widget.fill.unwrap();
        let area: f32 = mesh
            .indices
            .chunks_exact(3)
            .map(|t| {
                let a = mesh.vertices[t[0] as usize].pos;
                let b = mesh.vertices[t[1] as usize].pos;
                let c = mesh.vertices[t[2] as usize].pos;
                ((b - a).x * (c - a).y - (b - a).y * (c - a).x).abs() * 0.5
            })
            .sum();
        assert!((area - 7500.0).abs() < 0.01);
        assert_eq!(widget.strokes.len(), 2);
        assert!(widget.strokes.iter().all(
            |s| matches!(s, Shape::Mesh(mesh) if mesh.is_valid() && !mesh.indices.is_empty())
        ));
    }
}
