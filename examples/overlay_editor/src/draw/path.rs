use i_triangle::i_overlay::i_float::float::point::FloatPoint;
use i_triangle::i_overlay::i_float::int::point::IntPoint;
use i_triangle::i_overlay::i_shape::int::path::{IntPath, IntPaths};
use i_triangle::triangulation::float::Triangulation;
use i_triangle::stroke::butt::ButtStrokeBuilder;
use i_triangle::stroke::style::StrokeStyle;
use i_triangle::triangulation::float::TriangulationBuilder;
use iced::advanced::layout::{self, Layout};
use iced::advanced::{Clipboard, renderer, Shell};
use iced::advanced::widget::{Tree, Widget};
use iced::{Event, event, mouse, Color, Vector, Transformation};
use iced::{Element, Length, Rectangle, Renderer, Size, Theme};
use iced::advanced::graphics::color::pack;
use iced::advanced::graphics::Mesh;
use iced::advanced::graphics::mesh::{Indexed, SolidVertex2D};
use crate::geom::camera::Camera;
use crate::geom::vector::VectorExt;

pub(crate) struct PathWidget {
    stroke: Option<Mesh>,
}

impl PathWidget {
    pub(crate) fn with_paths(paths: &IntPaths, camera: Camera, stroke_color: Color, stroke_width: f32, arrows: bool) -> Self {
        let offset = Self::offset_for_paths(paths, camera);
        let stroke = Self::stroke_mesh_for_paths(paths, camera, offset, stroke_color, stroke_width, arrows);
        Self {
            stroke,
        }
    }

    fn stroke_mesh_for_paths(paths: &IntPaths, camera: Camera, offset: Vector<f32>, color: Color, width: f32, arrows: bool) -> Option<Mesh> {
        if paths.is_empty() {
            return None;
        }

        let mut builder = TriangulationBuilder::new();

        for path in paths.iter() {
            Self::append_path(&mut builder, camera, path, width, arrows);
        }

        let s = if arrows {
            2.5 * width
        } else {
            0.5 * width
        };

        let offset = Vector::new(offset.x - s, offset.y - s);

        let triangulation = builder.build();

        Self::stroke_mesh_for_triangulation(triangulation, offset, color)
    }

    fn stroke_mesh_for_triangulation(triangulation: Triangulation<FloatPoint<f32>>, offset: Vector<f32>, color: Color) -> Option<Mesh> {
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

    fn offset_for_paths(paths: &IntPaths, camera: Camera) -> Vector<f32> {
        if paths.is_empty() {
            return Vector::new(0.0, 0.0);
        }

        let mut min_x = i32::MAX;
        let mut max_y = i32::MIN;

        for p in paths.iter().flatten() {
            min_x = min_x.min(p.x);
            max_y = max_y.max(p.y);
        }

        camera.int_world_to_view(IntPoint::new(min_x, max_y))
    }

    fn append_path(builder: &mut TriangulationBuilder<FloatPoint<f32>>, camera: Camera, path: &IntPath, width: f32, arrows: bool) {
        let stroke_builder = ButtStrokeBuilder::new(StrokeStyle::with_width(width));
        let screen_path: Vec<_> = path.iter().map(|&p| {
            let v = camera.int_world_to_view(p);
            FloatPoint::new(v.x, v.y)
        }).collect();

        let sub_triangulation = stroke_builder.build_open_path_mesh(&screen_path);
        builder.append(sub_triangulation);

        let r2 = 2.0 * width;
        let r4 = 4.0 * width;

        if arrows {
            let mut a = screen_path[0];
            for &b in screen_path.iter().skip(1) {
                let m = (a + b) * 0.5;
                let n = (b - a).normalize();
                let m0 = m - n * r4;
                let t0 = FloatPoint::new(-n.y, n.x) * r2;
                let t1 = FloatPoint::new(n.y, -n.x) * r2;
                let v0 = m0 + t0;
                let v1 = m0 + t1;

                let arrow_triangulation = stroke_builder.build_open_path_mesh(&[v0, m, v1]);
                builder.append(arrow_triangulation);

                a = b;
            }
        }
    }
}

impl<Message> Widget<Message, Theme, Renderer> for PathWidget {
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

    fn on_event(
        &mut self,
        _tree: &mut Tree,
        _event: Event,
        _layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        _shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> event::Status {
        event::Status::Ignored
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
        if let Some(mesh) = &self.stroke {
            renderer.with_translation(offset, |renderer| {
                renderer.draw_mesh(mesh.clone())
            });
        }
    }
}

impl<'a, Message: 'a> From<PathWidget> for Element<'a, Message> {
    fn from(editor: PathWidget) -> Self {
        Self::new(editor)
    }
}