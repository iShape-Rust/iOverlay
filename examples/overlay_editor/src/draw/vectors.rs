use std::f32::consts::PI;
use i_triangle::i_overlay::i_float::float::point::FloatPoint;
use i_triangle::i_overlay::i_float::int::point::IntPoint;
use i_triangle::i_overlay::vector::edge::{SideFill, SUBJ_LEFT, SUBJ_RIGHT, CLIP_LEFT, CLIP_RIGHT, VectorEdge};
use i_triangle::triangulation::float::Triangulation;
use i_triangle::stroke::butt::ButtStrokeBuilder;
use i_triangle::stroke::style::StrokeStyle;
use iced::advanced::layout::{self, Layout};
use iced::advanced::{Clipboard, renderer, Shell};
use iced::advanced::widget::{Tree, Widget};
use iced::{Event, event, mouse, Color, Vector, Transformation};
use iced::{Element, Length, Rectangle, Renderer, Size, Theme};
use iced::advanced::graphics::color::pack;
use iced::advanced::graphics::{color, Mesh};
use iced::advanced::graphics::mesh::{Indexed, SolidVertex2D};
use crate::geom::camera::Camera;
use crate::geom::vector::VectorExt;

struct ColorSchema {
    subj: color::Packed,
    subj_none: color::Packed,
    clip: color::Packed,
    clip_none: color::Packed,
    both: color::Packed,
    none: color::Packed,
}

impl ColorSchema {
    fn new(subj: Color, clip: Color, both: Color) -> Self {
        Self {
            subj: pack(subj),
            subj_none: pack(subj.scale_alpha(0.05)),
            clip: pack(clip),
            clip_none: pack(clip.scale_alpha(0.05)),
            both: pack(both),
            none: pack(Color::from_rgb8(127, 127, 127)),
        }
    }

    fn color(&self, fill: SideFill) -> color::Packed {
        let subj = fill & (SUBJ_LEFT | SUBJ_RIGHT) != 0;
        let clip = fill & (CLIP_LEFT | CLIP_RIGHT) != 0;
        match (subj, clip) {
            (true, true) => self.both,
            (true, false) => self.subj,
            (false, true) => self.clip,
            (false, false) => self.none,
        }
    }

    fn subj_right(&self, fill: SideFill) -> color::Packed {
        if fill & SUBJ_RIGHT != 0 { self.subj } else { self.subj_none }
    }

    fn subj_left(&self, fill: SideFill) -> color::Packed {
        if fill & SUBJ_LEFT != 0 { self.subj } else { self.subj_none }
    }

    fn clip_right(&self, fill: SideFill) -> color::Packed {
        if fill & CLIP_RIGHT != 0 { self.clip } else { self.clip_none }
    }

    fn clip_left(&self, fill: SideFill) -> color::Packed {
        if fill & CLIP_LEFT != 0 { self.clip } else { self.clip_none }
    }
}

pub(crate) struct VectorsWidget {
    stroke: Option<Mesh>,
}

impl VectorsWidget {
    pub(crate) fn with_vectors(vectors: &[VectorEdge], camera: Camera, subj: Color, clip: Color, both: Color, stroke_width: f32) -> Self {
        let schema = ColorSchema::new(subj, clip, both);
        let offset = Self::offset_for_vectors(vectors, camera);
        let stroke = Self::stroke_mesh_for_paths(vectors, camera, offset, schema, stroke_width);
        Self {
            stroke,
        }
    }

    fn stroke_mesh_for_paths(vectors: &[VectorEdge], camera: Camera, offset: Vector<f32>, schema: ColorSchema, width: f32) -> Option<Mesh> {
        if vectors.is_empty() {
            return None;
        }

        let mut builder = MeshBuilder::new();

        let s = 8.0 * width;
        let offset = Vector::new(offset.x - s, offset.y - s);

        for vector in vectors.iter() {
            Self::append_vector(&mut builder, camera, vector, offset, &schema, width);
        }

        let buffers = builder.build();

        Some(Mesh::Solid {
            buffers,
            transformation: Transformation::translate(offset.x, offset.y),
            clip_bounds: Rectangle::INFINITE,
        })
    }

    fn offset_for_vectors(vectors: &[VectorEdge], camera: Camera) -> Vector<f32> {
        if vectors.is_empty() {
            return Vector::new(0.0, 0.0);
        }

        let mut min_x = i32::MAX;
        let mut max_y = i32::MIN;

        for v in vectors.iter() {
            min_x = min_x.min(v.a.x);
            min_x = min_x.min(v.b.x);
            max_y = max_y.max(v.a.y);
            max_y = max_y.max(v.b.y);
        }

        camera.int_world_to_view(IntPoint::new(min_x, max_y))
    }

    fn append_vector(builder: &mut MeshBuilder, camera: Camera, vector: &VectorEdge, offset: Vector<f32>, schema: &ColorSchema, width: f32) {
        let stroke_builder = ButtStrokeBuilder::new(StrokeStyle::with_width(width));
        let path = [vector.a, vector.b];
        let screen_path: Vec<_> = path.iter().map(|&p| {
            let v = camera.int_world_to_view(p);
            FloatPoint::new(v.x - offset.x, v.y - offset.y)
        }).collect();

        let segment_color = schema.color(vector.fill);

        let sub_triangulation = stroke_builder.build_open_path_mesh(&screen_path);
        builder.append(sub_triangulation, segment_color);

        let r2 = 4.0 * width;

        let a = screen_path[0];
        let b = screen_path[1];

        let n = (b - a).normalize();
        let m = (a + b) * 0.5;
        let m0 = b - n * r2;
        let t0 = FloatPoint::new(-n.y, n.x) * r2;
        let t1 = FloatPoint::new(n.y, -n.x) * r2;
        let s0 = n * r2;
        let s1 = -n * r2;
        let v0 = m0 + t0 * 0.5;
        let v1 = m0 + t1 * 0.5;

        let subj_right = schema.subj_right(vector.fill);
        let clip_right = schema.clip_right(vector.fill);
        let subj_left = schema.subj_left(vector.fill);
        let clip_left = schema.clip_left(vector.fill);

        let subj_right_pos = m + t0 + s0;
        let subj_left_pos = m + t1 + s0;

        let clip_right_pos = m + t0 + s1;
        let clip_left_pos = m + t1 + s1;

        Self::append_circle(builder, subj_right_pos, subj_right, 2.0 * width);
        Self::append_circle(builder, clip_right_pos, clip_right, 2.0 * width);
        Self::append_circle(builder, subj_left_pos, subj_left, 2.0 * width);
        Self::append_circle(builder, clip_left_pos, clip_left, 2.0 * width);

        let arrow_triangulation = stroke_builder.build_open_path_mesh(&[v0, b, v1]);
        builder.append(arrow_triangulation, segment_color);

    }

    fn append_circle(builder: &mut MeshBuilder, pos: FloatPoint<f32>, color: color::Packed, radius: f32) {
        let n = 8;
        let da = 2.0 * PI / n as f32;
        let mut a = 0.0f32;
        let mut indices = Vec::with_capacity(3 * n);
        let mut points = Vec::with_capacity(n);
        for i in 0..n {
            let sc = a.sin_cos();
            let x = pos.x + sc.1 * radius;
            let y = pos.y + sc.0 * radius;
            points.push(FloatPoint::new(x, y));
            indices.extend(&[n, i, (i + 1) % n]);
            a += da;
        };

        points.push(pos);

        builder.append(Triangulation { points, indices }, color);
    }
}

impl<Message> Widget<Message, Theme, Renderer> for VectorsWidget {
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

impl<'a, Message: 'a> From<VectorsWidget> for Element<'a, Message> {
    fn from(editor: VectorsWidget) -> Self {
        Self::new(editor)
    }
}

struct MeshBuilder {
    vertices: Vec<SolidVertex2D>,
    indices: Vec<u32>,
}

impl MeshBuilder {
    fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    fn append(&mut self, triangulation: Triangulation<FloatPoint<f32>>, color: color::Packed) -> &mut Self {
        let offset = self.vertices.len();
        for p in triangulation.points.iter() {
            self.vertices.push(SolidVertex2D { position: [p.x, p.y], color });
        }
        self.indices.extend(
            triangulation
                .indices
                .iter()
                .map(|&i| (i + offset) as u32),
        );
        self
    }

    fn build(self) -> Indexed<SolidVertex2D> {
        Indexed {
            vertices: self.vertices,
            indices: self.indices,
        }
    }
}