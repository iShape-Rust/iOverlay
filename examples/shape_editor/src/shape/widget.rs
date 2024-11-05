use i_triangle::i_overlay::i_float::int::point::IntPoint;
use i_triangle::i_overlay::i_shape::int::shape::IntShapes;
use i_triangle::triangulation::int::IntTriangulate;
use iced::advanced::widget::tree;
use iced::advanced::layout::{self, Layout};
use iced::advanced::{Clipboard, renderer, Shell};
use iced::advanced::widget::{Tree, Widget};
use iced::{Event, event, mouse, Color, Vector, Transformation};
use iced::{Element, Length, Rectangle, Renderer, Size, Theme};
use iced::advanced::graphics::color::pack;
use iced::advanced::graphics::Mesh;
use iced::advanced::graphics::mesh::{Indexed, SolidVertex2D};
use crate::geom::camera::Camera;
use crate::geom::viewport::ViewPortExt;
use crate::point_editor::state::PointsEditorState;

pub(crate) struct ShapeWidget {
    camera: Camera,
    fill: Option<Mesh>,
    stroke: Option<Mesh>,
}

impl ShapeWidget {
    pub(crate) fn new(camera: Camera, shapes: &IntShapes, fill_color: Option<Color>, stroke_color: Option<Color>, stroke_width: f32) -> Self {
        let offset = Self::offset(camera, shapes);
        let fill = Self::fill_mesh(camera, shapes, offset, fill_color);
        let stroke = Self::stroke_mesh(camera, shapes, offset, fill_color, stroke_width);
        Self {
            camera,
            fill,
            stroke,
        }
    }

    fn fill_mesh(camera: Camera, shapes: &IntShapes, offset: Vector<f32>, color: Option<Color>) -> Option<Mesh> {
        if shapes.is_empty() {
            return None;
        }
        let color = color?;

        let triangulation = shapes.to_triangulation(None, 0);
        if triangulation.indices.is_empty() {
            return None;
        }
        let color_pack = pack(color);
        let vertices = triangulation.points.iter().map(|&p| {
            let v = camera.point_to_screen(p);
            SolidVertex2D { position: [v.x - offset.x, v.y - offset.y], color: color_pack }
        }).collect();

        let indices = triangulation.indices.iter().map(|&i| i as u32).collect();

        Some(Mesh::Solid {
            buffers: Indexed { vertices, indices },
            transformation: Transformation::translate(offset.x, offset.y),
            clip_bounds: Rectangle::INFINITE,
        })
    }

    fn stroke_mesh(camera: Camera, shapes: &IntShapes, offset: Vector<f32>, color: Option<Color>, width: f32) -> Option<Mesh> {
        if shapes.is_empty() {
            return None;
        }
        let color = color?;
        None
    }

    fn offset(camera: Camera, shapes: &IntShapes) -> Vector<f32> {
        if shapes.is_empty() {
            return Vector::new(0.0, 0.0);
        }

        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;

        for p in shapes.iter().flatten().flatten() {
            min_x = min_x.min(p.x);
            min_y = min_y.min(p.y);
        }

        camera.point_to_screen(IntPoint::new(min_x, min_y))
    }
}

impl<Message> Widget<Message, Theme, Renderer> for ShapeWidget {
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Fill,
            height: Length::Fill,
        }
    }

    fn layout(
        &self,
        tree: &mut Tree,
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
        tree: &Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        use iced::advanced::graphics::mesh::Renderer as _;
        use iced::advanced::Renderer as _;

        let offset = layout.bounds().offset();
        if let Some(mesh) = &self.fill {
            renderer.with_translation(offset, |renderer| {
                renderer.draw_mesh(mesh.clone())
            });
        }
        if let Some(mesh) = &self.stroke {
            renderer.with_translation(offset, |renderer| {
                renderer.draw_mesh(mesh.clone())
            });
        }
    }
}

impl<'a, Message: 'a> From<ShapeWidget> for Element<'a, Message> {
    fn from(editor: ShapeWidget) -> Self {
        Self::new(editor)
    }
}


