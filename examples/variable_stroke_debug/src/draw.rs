use crate::geom::camera::Camera;
use i_triangle::i_overlay::i_shape::int::path::IntPaths;
use iced::advanced::graphics::Mesh;
use iced::advanced::graphics::color::pack;
use iced::advanced::graphics::mesh::{Indexed, SolidVertex2D};
use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::widget::{Tree, Widget};
use iced::{Color, Element, Length, Rectangle, Renderer, Size, Theme, Transformation, mouse};

pub(crate) struct LinesWidget {
    mesh: Option<Mesh>,
}

impl LinesWidget {
    pub(crate) fn new(
        paths: &IntPaths<i32>,
        camera: Camera,
        color: Color,
        width: f32,
        arrows: bool,
        closed: bool,
    ) -> Self {
        let color = pack(color);
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for path in paths {
            for pair in path.windows(2) {
                append_segment(
                    camera,
                    pair[0],
                    pair[1],
                    color,
                    width,
                    arrows,
                    &mut vertices,
                    &mut indices,
                );
            }
            if closed && path.len() > 2 {
                append_segment(
                    camera,
                    *path.last().unwrap(),
                    path[0],
                    color,
                    width,
                    false,
                    &mut vertices,
                    &mut indices,
                );
            }
        }

        let mesh = (!indices.is_empty()).then_some(Mesh::Solid {
            buffers: Indexed { vertices, indices },
            transformation: Transformation::IDENTITY,
            clip_bounds: Rectangle::INFINITE,
        });
        Self { mesh }
    }
}

fn append_segment(
    camera: Camera,
    a: i_triangle::i_overlay::i_float::int::point::IntPoint<i32>,
    b: i_triangle::i_overlay::i_float::int::point::IntPoint<i32>,
    color: iced::advanced::graphics::color::Packed,
    width: f32,
    arrows: bool,
    vertices: &mut Vec<SolidVertex2D>,
    indices: &mut Vec<u32>,
) {
    let a = camera.int_world_to_view(a);
    let b = camera.int_world_to_view(b);
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    let length = (dx * dx + dy * dy).sqrt();
    if length < 0.001 {
        return;
    }
    let half = 0.5 * width;
    let nx = -dy * half / length;
    let ny = dx * half / length;
    let base = vertices.len() as u32;
    vertices.extend([
        vertex(a.x + nx, a.y + ny, color),
        vertex(a.x - nx, a.y - ny, color),
        vertex(b.x - nx, b.y - ny, color),
        vertex(b.x + nx, b.y + ny, color),
    ]);
    indices.extend([base, base + 1, base + 2, base, base + 2, base + 3]);

    if arrows {
        let ux = dx / length;
        let uy = dy / length;
        let size = (4.0 * width).max(5.0);
        let tip_x = a.x + 0.68 * dx;
        let tip_y = a.y + 0.68 * dy;
        let back_x = tip_x - ux * size;
        let back_y = tip_y - uy * size;
        let wing = 0.55 * size;
        let arrow_base = vertices.len() as u32;
        vertices.extend([
            vertex(tip_x, tip_y, color),
            vertex(back_x - uy * wing, back_y + ux * wing, color),
            vertex(back_x + uy * wing, back_y - ux * wing, color),
        ]);
        indices.extend([arrow_base, arrow_base + 1, arrow_base + 2]);
    }
}

fn vertex(x: f32, y: f32, color: iced::advanced::graphics::color::Packed) -> SolidVertex2D {
    SolidVertex2D {
        position: [x, y],
        color,
    }
}

impl<Message> Widget<Message, Theme, Renderer> for LinesWidget {
    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fill)
    }

    fn layout(
        &mut self,
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
        use iced::advanced::Renderer as _;
        use iced::advanced::graphics::mesh::Renderer as _;

        let bounds = layout.bounds();
        renderer.with_layer(bounds, |renderer| {
            if let Some(mesh) = &self.mesh {
                renderer.with_translation(layout.position() - iced::Point::ORIGIN, |renderer| {
                    renderer.draw_mesh(mesh.clone());
                });
            }
        });
    }
}

impl<'a, Message: 'a> From<LinesWidget> for Element<'a, Message> {
    fn from(widget: LinesWidget) -> Self {
        Self::new(widget)
    }
}
