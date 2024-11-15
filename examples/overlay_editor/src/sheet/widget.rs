use crate::geom::camera::Camera;
use crate::sheet::state::SheetState;
use iced::{Color, Point, Transformation};
use iced::advanced::widget::tree;
use iced::advanced::layout::{self, Layout};
use iced::advanced::{Clipboard, renderer, Shell};
use iced::advanced::widget::{Tree, Widget};
use iced::{Event, event, mouse};
use iced::{Element, Length, Rectangle, Renderer, Size, Theme, Vector};
use iced::advanced::graphics::color::pack;
use iced::advanced::graphics::Mesh;
use iced::advanced::graphics::mesh::{SolidVertex2D, Indexed};

pub(crate) struct SheetWidget<'a, Message> {
    camera: Camera,
    grid_color: Color,
    on_size: Box<dyn Fn(Size) -> Message + 'a>,
    on_zoom: Box<dyn Fn(Camera) -> Message + 'a>,
    on_drag: Box<dyn Fn(Vector<f32>) -> Message + 'a>,
}

impl<'a, Message: 'a> SheetWidget<'a, Message> {
    pub(crate) fn new(
        camera: Camera,
        grid_color: Color,
        on_size: impl Fn(Size) -> Message + 'a,
        on_zoom: impl Fn(Camera) -> Message + 'a,
        on_drag: impl Fn(Vector<f32>) -> Message + 'a,
    ) -> Self {
        Self {
            camera,
            grid_color,
            on_size: Box::new(on_size),
            on_zoom: Box::new(on_zoom),
            on_drag: Box::new(on_drag),
        }
    }

    pub(super) fn is_size_changed(&self, size: Size) -> bool {
        let w = (size.width - self.camera.size.width).abs();
        let h = (size.height - self.camera.size.height).abs();
        w > 0.01 || h > 0.01
    }

    fn line_mesh(&self, min_x: f32, min_y: f32, max_x: f32, max_y: f32, opacity: f32) -> Mesh {
        let color_pack = pack(self.grid_color.scale_alpha(opacity));
        let mut vertices = Vec::with_capacity(4);
        let mut indices = Vec::with_capacity(6);

        vertices.push(SolidVertex2D { position: [min_x, min_y], color: color_pack });
        vertices.push(SolidVertex2D { position: [min_x, max_y], color: color_pack });
        vertices.push(SolidVertex2D { position: [max_x, max_y], color: color_pack });
        vertices.push(SolidVertex2D { position: [max_x, min_y], color: color_pack });

        indices.push(0);
        indices.push(1);
        indices.push(2);

        indices.push(0);
        indices.push(2);
        indices.push(3);

        Mesh::Solid {
            buffers: Indexed { vertices, indices },
            transformation: Transformation::IDENTITY,
            clip_bounds: Rectangle::INFINITE,
        }
    }
}

impl<Message> Widget<Message, Theme, Renderer> for SheetWidget<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<SheetState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(SheetState::default())
    }

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
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> event::Status {
        let state = tree.state.downcast_mut::<SheetState>();

        let bounds = layout.bounds();

        let size = bounds.size();
        if self.is_size_changed(bounds.size()) {
            shell.publish((self.on_size)(size));
        }

        if let Event::Mouse(mouse_event) = event {
            match mouse_event {
                mouse::Event::CursorMoved { position } => {
                    if bounds.contains(position) {
                        let view_cursor = position - bounds.position();
                        if let Some(drag) = state.mouse_move(self.camera, view_cursor) {
                            shell.publish((self.on_drag)(drag));
                            return event::Status::Captured;
                        }
                    }
                }
                mouse::Event::ButtonPressed(mouse::Button::Left) => {
                    let position = cursor.position().unwrap_or(Point::ORIGIN);
                    if bounds.contains(position) {
                        let view_cursor = position - bounds.position();
                        state.mouse_press(self.camera, view_cursor);
                        return event::Status::Captured;
                    }
                }
                mouse::Event::ButtonReleased(mouse::Button::Left) => {
                    state.mouse_release();
                    return event::Status::Captured;
                }
                mouse::Event::WheelScrolled { delta } => {
                    let position = cursor.position().unwrap_or(Point::ORIGIN);
                    if bounds.contains(position) {
                        let cursor = position - bounds.position();
                        if let Some(scale) = state.mouse_wheel_scrolled(self.camera, bounds.size(), delta, cursor) {
                            shell.publish((self.on_zoom)(scale));
                            return event::Status::Captured;
                        }
                    }
                }
                _ => {
                    // println!("other mouse event: {:?}", mouse_event);
                }
            }
        }

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
        const MIN_SCALE: f32 = 20.0;
        let scale = self.camera.scale - MIN_SCALE;
        if scale <= 0.0 {
            return;
        }
        const SCALE_RANGE: f32 = 50.0;
        const INVERT_RANGE: f32 = 1.0 / SCALE_RANGE;

        // normalize scale
        let s = (scale * INVERT_RANGE).min(1.0);
        // stroke radius
        let r = 1.0;

        let rect = layout.bounds();

        let view_min = Vector::new(0.0, 0.0);
        let view_max = Vector::new(rect.width, rect.height);

        let world_min = self.camera.view_to_world(view_min);
        let world_max = self.camera.view_to_world(view_max);

        let round_world_min_x = world_min.x.ceil();
        let round_world_min_y = world_min.y.ceil();

        let round_world_max_x = world_max.x.trunc();
        let round_world_max_y = world_max.y.trunc();

        let nfx = (round_world_max_x - round_world_min_x + 0.001).round().abs();
        let nfy = (round_world_max_y - round_world_min_y + 0.001).round().abs();

        let nx = nfx as usize;
        let ny = nfy as usize;

        let vr_mesh = self.line_mesh(-r, view_min.y, r, view_max.y, s);
        let hz_mesh = self.line_mesh(view_min.x, -r, view_max.x, r, s);

        use iced::advanced::graphics::mesh::Renderer as _;
        use iced::advanced::Renderer as _;

        let round_view_min = self.camera.world_to_view(Vector::new(round_world_min_x, round_world_min_y));
        let round_view_max = self.camera.world_to_view(Vector::new(round_world_max_x, round_world_max_y));

        let dx = (round_view_max.x - round_view_min.x) / nfx;
        let dy = (round_view_max.y - round_view_min.y) / nfy;

        let mut position = Vector::new(round_view_min.x + rect.x, view_min.y + rect.y);
        for _ in 0..=nx {
            renderer.with_translation(position, |renderer| renderer.draw_mesh(vr_mesh.clone()));
            position.x += dx;
        }

        let mut position = Vector::new(view_min.x + rect.x, round_view_min.y + rect.y);
        for _ in 0..=ny {
            renderer.with_translation(position, |renderer| renderer.draw_mesh(hz_mesh.clone()));
            position.y += dy;
        }
    }
}

impl<'a, Message: 'a> From<SheetWidget<'a, Message>> for Element<'a, Message> {
    fn from(sheet: SheetWidget<'a, Message>) -> Self {
        Self::new(sheet)
    }
}
