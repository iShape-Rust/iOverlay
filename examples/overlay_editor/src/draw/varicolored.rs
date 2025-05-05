use i_mesh::path::butt::ButtStrokeBuilder;
use i_mesh::path::style::StrokeStyle;
use i_triangle::float::builder::TriangulationBuilder;
use i_triangle::float::triangulation::Triangulation;
use i_triangle::i_overlay::core::fill_rule::FillRule;
use i_triangle::i_overlay::i_float::float::point::FloatPoint;
use i_triangle::i_overlay::i_float::int::point::IntPoint;
use i_triangle::i_overlay::i_shape::int::path::IntPaths;
use i_triangle::i_overlay::i_shape::int::shape::IntShapes;
use i_triangle::int::triangulation::IntTriangulation;
use i_triangle::int::triangulator::Triangulator;
use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::widget::{Tree, Widget};
use iced::{mouse, Color, Vector, Transformation};
use iced::{Element, Length, Rectangle, Renderer, Size, Theme};
use iced::advanced::graphics::color::pack;
use iced::advanced::graphics::Mesh;
use iced::advanced::graphics::mesh::{Indexed, SolidVertex2D};
use crate::geom::camera::Camera;
use crate::geom::vector::VectorExt;

pub(crate) struct VaricoloredWidget {
    fill: Vec<Mesh>,
    stroke: Vec<Mesh>,
}

impl VaricoloredWidget {
    const SHAPE_COLOR_STORE: [[u8; 3]; 12] = [
        [255, 149, 0],    // Orange
        [88, 86, 214],    // Purple
        [255, 45, 85],    // Pink
        [90, 200, 250],   // Blue
        [76, 217, 100],   // Green
        [255, 204, 0],    // Yellow
        [142, 142, 147],  // Gray
        [255, 59, 48],    // Red
        [52, 199, 89],    // Green
        [0, 122, 255],    // Blue
        [175, 82, 222],   // Indigo
        [255, 214, 10],   // Teal
    ];

    pub(crate) fn with_shapes(shapes: &IntShapes, camera: Camera, fill_rule: Option<FillRule>, stroke_width: f32) -> Self {
        let offset = Self::offset_for_shapes(shapes, camera);

        let mut fill = Vec::new();
        let mut stroke = Vec::new();
        for (index, shape) in shapes.iter().enumerate() {
            let data = Self::SHAPE_COLOR_STORE[index % Self::SHAPE_COLOR_STORE.len()];
            let color = Color::from_rgb8(data[0], data[1], data[2]);

            if let Some(mesh) = Self::fill_mesh_for_paths(shape, camera, offset, fill_rule, color.scale_alpha(0.2)) {
                fill.push(mesh);
            }
            if let Some(mesh) = Self::stroke_mesh_for_paths(shape, camera, offset, color, stroke_width) {
                stroke.push(mesh);
            }
        }

        Self {
            fill,
            stroke,
        }
    }

    fn fill_mesh_for_paths(paths: &IntPaths, camera: Camera, offset: Vector<f32>, fill_rule: Option<FillRule>, color: Color) -> Option<Mesh> {
        if paths.is_empty() {
            return None;
        }

        let triangulation = Triangulator::with_fill_rule(fill_rule.unwrap_or(FillRule::NonZero))
            .triangulate_shape(paths).into_triangulation();
        Self::fill_mesh_for_triangulation(triangulation, camera, offset, color)
    }

    fn fill_mesh_for_triangulation(triangulation: IntTriangulation<usize>, camera: Camera, offset: Vector<f32>, color: Color) -> Option<Mesh> {
        if triangulation.indices.is_empty() {
            return None;
        }
        let color_pack = pack(color);
        let vertices = triangulation.points.iter().map(|&p| {
            let v = camera.int_world_to_view(p);
            SolidVertex2D { position: [v.x - offset.x, v.y - offset.y], color: color_pack }
        }).collect();

        let indices = triangulation.indices.iter().map(|&i| i as u32).collect();

        Some(Mesh::Solid {
            buffers: Indexed { vertices, indices },
            transformation: Transformation::translate(offset.x, offset.y),
            clip_bounds: Rectangle::INFINITE,
        })
    }

    fn stroke_mesh_for_paths(paths: &IntPaths, camera: Camera, offset: Vector<f32>, color: Color, width: f32) -> Option<Mesh> {
        if paths.is_empty() {
            return None;
        }
        let stroke_builder = ButtStrokeBuilder::new(StrokeStyle::with_width(width));

        let mut builder = TriangulationBuilder::default();

        for path in paths.iter() {
            let world_path: Vec<_> = path.iter().map(|&p| {
                let v = camera.int_world_to_view(p);
                FloatPoint::new(v.x, v.y)
            }).collect();

            let sub_triangulation = stroke_builder.build_closed_path_mesh(&world_path);
            builder.append(sub_triangulation);
        }

        let r = 0.5 * width;
        let offset = Vector::new(offset.x - r, offset.y - r);

        let triangulation = builder.build();

        Self::stroke_mesh_for_triangulation(triangulation, offset, color)
    }

    fn stroke_mesh_for_triangulation(triangulation: Triangulation<FloatPoint<f32>, usize>, offset: Vector<f32>, color: Color) -> Option<Mesh> {
        if triangulation.indices.is_empty() {
            return None;
        }
        let color_pack = pack(color);
        let vertices = triangulation.points.iter().map(|&p| {
            SolidVertex2D { position: [p.x - offset.x, p.y - offset.y], color: color_pack }
        }).collect();

        let indices = triangulation.indices.iter().map(|&i| i as u32).collect();

        Some(Mesh::Solid {
            buffers: Indexed { vertices, indices },
            transformation: Transformation::translate(offset.x, offset.y),
            clip_bounds: Rectangle::INFINITE,
        })
    }

    fn offset_for_shapes(shapes: &IntShapes, camera: Camera) -> Vector<f32> {
        if shapes.is_empty() {
            return Vector::new(0.0, 0.0);
        }

        let mut min_x = i32::MAX;
        let mut max_y = i32::MIN;

        for p in shapes.iter().flatten().flatten() {
            min_x = min_x.min(p.x);
            max_y = max_y.max(p.y);
        }

        camera.int_world_to_view(IntPoint::new(min_x, max_y))
    }
}

impl<Message> Widget<Message, Theme, Renderer> for VaricoloredWidget {
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Fill,
            height: Length::Fill,
        }
    }

    fn layout(
        &self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::new(limits.max())
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        use iced::advanced::graphics::mesh::Renderer as _;
        use iced::advanced::Renderer as _;

        let offset = Vector::point(layout.position());

        renderer.with_translation(offset, |renderer| {
            for mesh in self.fill.iter() {
                renderer.draw_mesh(mesh.clone())
            }
        });


        renderer.with_translation(offset, |renderer| {
            for mesh in self.stroke.iter() {
                renderer.draw_mesh(mesh.clone())
            }
        });
    }
}

impl<'a, Message: 'a> From<VaricoloredWidget> for Element<'a, Message> {
    fn from(editor: VaricoloredWidget) -> Self {
        Self::new(editor)
    }
}