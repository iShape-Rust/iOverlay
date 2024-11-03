use crate::app::boolean::editor::widget::tree::State;
use crate::app::boolean::editor::state::SubjClipEditorState;
use iced::advanced::widget::tree;
use crate::app::boolean::editor::state::PolygonEditorMessage;
use crate::app::boolean::editor::data::StatelessData;
use crate::util::point::EditorPoint;
use i_triangle::i_overlay::i_shape::int::shape::IntShape;
use i_triangle::triangulation::int::Triangulation;
use iced::advanced::graphics::color;
use iced::advanced::layout::{self, Layout};
use iced::advanced::{Clipboard, renderer, Shell};
use iced::advanced::widget::{Tree, Widget};
use iced::{Event, event, mouse, Point};
use iced::{
    Element, Length, Rectangle, Renderer, Size, Theme, Transformation,
    Vector,
};

enum OnPress<'a, Message> {
    Direct(Message),
    Closure(Box<dyn Fn() -> Message + 'a>),
}

impl<'a, Message: Clone> OnPress<'a, Message> {
    fn get(&self) -> Message {
        match self {
            OnPress::Direct(message) => message.clone(),
            OnPress::Closure(f) => f(),
        }
    }
}

pub(crate) struct SubjClipEditorWidget<'a, Message> {
    stateless: &'a StatelessData,
    on_press: Option<OnPress<'a, Message>>,
}

impl<'a, Message> SubjClipEditorWidget<'a, Message> {
    pub(crate) fn new(stateless: &'a StatelessData) -> Self {
        Self { stateless, on_press: None }
    }
}


impl<Message> Widget<Message, Theme, Renderer> for SubjClipEditorWidget<'_, Message> {

    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<SubjClipEditorState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(SubjClipEditorState::default())
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Fill,
            height: Length::Shrink,
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
        _shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        if let State::Some(stete_box) = &mut tree.state {
            let state = stete_box.downcast_mut::<SubjClipEditorState>().unwrap();
            state.update_camera(&self.stateless.editor_points, viewport)
        };

        match event {
            Event::Mouse(mouse_event) => match mouse_event {
                mouse::Event::CursorMoved { position } => {
                    if layout.bounds().contains(position) {
                        println!("is_hovered = true");
                        //self.is_hovered = true;
                    } else {
                        println!("is_hovered = false");
                        // self.is_hovered = false;
                    }
                    event::Status::Captured
                }
                mouse::Event::ButtonPressed(mouse::Button::Left) => {
                    if layout.bounds().contains(cursor.position().unwrap_or(Point::ORIGIN)) {
                        // self.is_pressed = true;
                        println!("is_pressed = true");
                    }
                    event::Status::Captured
                }
                mouse::Event::ButtonReleased(mouse::Button::Left) => {
                    println!("is_pressed = false");
                    // self.is_pressed = false;
                    event::Status::Captured
                }
                _ => event::Status::Ignored,
            },
            _ => event::Status::Ignored,
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let state = if let State::Some(stete_box) = &tree.state {
            stete_box.downcast_ref::<SubjClipEditorState>().unwrap()
        } else {
            return;
        };

        use iced::advanced::graphics::mesh::{
            self, Mesh, Renderer as _, SolidVertex2D,
        };
        use iced::advanced::Renderer as _;

        if let Some(mesh) = state.point_mesh(&self.stateless.editor_points) {
            let bounds = layout.bounds();
            renderer.with_translation(
                Vector::new(bounds.x, bounds.y),
                |renderer| {
                    renderer.draw_mesh(mesh);
                },
            );
        }
    }
}

impl<'a, Message: 'a> From<SubjClipEditorWidget<'a, Message>> for Element<'a, Message> {
    fn from(editor: SubjClipEditorWidget<'a, Message>) -> Self {
        Self::new(editor)
    }
}