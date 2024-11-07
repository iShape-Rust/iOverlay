use crate::geom::vector::VectorExt;
use crate::geom::camera::Camera;
use crate::sheet::state::SheetState;
use iced::Point;
use iced::advanced::widget::tree;
use iced::advanced::layout::{self, Layout};
use iced::advanced::{Clipboard, renderer, Shell};
use iced::advanced::widget::{Tree, Widget};
use iced::{Event, event, mouse};
use iced::{Element, Length, Rectangle, Renderer, Size, Theme, Vector};

pub(crate) struct SheetWidget<'a, Message> {
    camera: Camera,
    on_size: Box<dyn Fn(Size) -> Message + 'a>,
    on_zoom: Box<dyn Fn(f32) -> Message + 'a>,
    on_drag: Box<dyn Fn(Vector<f32>) -> Message + 'a>,
}

impl<'a, Message: 'a> SheetWidget<'a, Message> {
    pub(crate) fn new(
        camera: Camera,
        on_size: impl Fn(Size) -> Message + 'a,
        on_zoom: impl Fn(f32) -> Message + 'a,
        on_drag: impl Fn(Vector<f32>) -> Message + 'a,
    ) -> Self {
        Self {
            camera,
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

        match event {
            Event::Mouse(mouse_event) => match mouse_event {
                mouse::Event::CursorMoved { position } => {
                    if bounds.contains(position) {
                        let cursor = Vector::point(position);
                        if let Some(drag) = state.mouse_move(self.camera, cursor) {
                            shell.publish((self.on_drag)(drag));
                            return event::Status::Captured;
                        }
                    }
                }
                mouse::Event::ButtonPressed(mouse::Button::Left) => {
                    let position = cursor.position().unwrap_or(Point::ORIGIN);
                    if bounds.contains(position) {
                        state.mouse_press(self.camera, Vector::point(position));
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
                        if let Some(scale) = state.mouse_wheel_scrolled(self.camera, bounds.size(), delta) {
                            shell.publish((self.on_zoom)(scale));
                            return event::Status::Captured;
                        }
                    }
                }
                _ => {
                    // println!("other mouse event: {:?}", mouse_event);
                }
            },
            _ => {}
        }
        event::Status::Ignored
    }

    fn draw(
        &self,
        _tree: &Tree,
        _renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        _layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {}
}

impl<'a, Message: 'a> From<SheetWidget<'a, Message>> for Element<'a, Message> {
    fn from(sheet: SheetWidget<'a, Message>) -> Self {
        Self::new(sheet)
    }
}
