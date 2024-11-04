use crate::point_editor::state::SelectState;
use iced::advanced::widget::tree::State;
use crate::point_editor::state::PointsEditorState;
use iced::advanced::widget::tree;
use crate::point_editor::state::PolygonEditorMessage;
use crate::point_editor::point::EditorPoint;
use i_triangle::i_overlay::i_shape::int::shape::IntShape;
use i_triangle::triangulation::int::Triangulation;
use iced::advanced::graphics::{color, Mesh};
use iced::advanced::layout::{self, Layout};
use iced::advanced::{Clipboard, renderer, Shell};
use iced::advanced::widget::{Tree, Widget};
use iced::{Event, event, mouse, Point, Color};
use iced::{
    Element, Length, Rectangle, Renderer, Size, Theme, Transformation,
    Vector,
};

#[derive(Debug, Clone)]
pub(crate) struct PointEditUpdate {
    pub(crate) point: EditorPoint,
    pub(crate) index: usize
}

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

pub(crate) struct PointsEditorWidget<'a, Message> {
    pub(super) points: &'a Vec<EditorPoint>,
    main_color: Option<Color>,
    hover_color: Option<Color>,
    pub(super) radius: f32,
    on_update: Box<dyn Fn(PointEditUpdate) -> Message + 'a>,
}

impl<'a, Message> PointsEditorWidget<'a, Message> {
    pub(crate) fn new(points: &'a Vec<EditorPoint>, on_update: impl Fn(PointEditUpdate) -> Message + 'a) -> Self {
        Self { points, radius: 15.0, main_color: None, hover_color: None, on_update: Box::new(on_update) }
    }

    pub(crate) fn set_main_color(mut self, color: Color) -> Self {
        self.main_color = Some(color);
        self
    }

    pub(crate) fn set_hover_color(mut self, color: Color) -> Self {
        self.hover_color = Some(color);
        self
    }
}

impl<Message> Widget<Message, Theme, Renderer> for PointsEditorWidget<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<PointsEditorState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(PointsEditorState::default())
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Fill,
            height: Length::Shrink,
        }
    }

    fn layout(
        &self,
        tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        if let State::Some(stete_box) = &mut tree.state {
            stete_box.downcast_mut::<PointsEditorState>().unwrap()
                .update_mesh(
                    self.radius,
                    self.main_color.unwrap_or(Color::BLACK),
                    self.hover_color.unwrap_or(Color::from_rgb(1.0, 0.2, 0.2)),
                )
        };

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
        viewport: &Rectangle,
    ) -> event::Status {
        let state = tree.state.downcast_mut::<PointsEditorState>();
        state.update_camera(&self.points, viewport);

        let bounds = layout.bounds();

        match event {
            Event::Mouse(mouse_event) => match mouse_event {
                mouse::Event::CursorMoved { position } => {
                    if bounds.contains(position) {
                        let cursor = Vector { x: position.x, y: position.y };
                        let offset = bounds.offset();
                        if let Some(updated_point) = state.mouse_move(
                            &*self,
                            cursor,
                            offset,
                        ) {
                            shell.publish((self.on_update)(updated_point));
                        }
                    }
                    event::Status::Captured
                }
                mouse::Event::ButtonPressed(mouse::Button::Left) => {
                    let position = cursor.position().unwrap_or(Point::ORIGIN);
                    if bounds.contains(position) {
                        let cursor = Vector { x: position.x, y: position.y };
                        let offset = bounds.offset();
                        state.mouse_press(
                            &*self,
                            cursor,
                            offset,
                        );
                    }
                    event::Status::Captured
                }
                mouse::Event::ButtonReleased(mouse::Button::Left) => {
                    let position = cursor.position().unwrap_or(Point::ORIGIN);
                    let cursor = Vector { x: position.x, y: position.y };
                    let offset = bounds.offset();
                    state.mouse_release(
                        &*self,
                        cursor,
                        offset
                    );
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
        let state = tree.state.downcast_ref::<PointsEditorState>();

        let camera = if let Some(camera) = state.camera { camera } else { return; };
        let mesh = if let Some(mesh) = &state.mesh_cache { mesh } else { return; };

        use iced::advanced::graphics::mesh::{
            self, Mesh, Renderer as _, SolidVertex2D,
        };
        use iced::advanced::Renderer as _;

        let bounds = layout.bounds();
        let offset = bounds.offset() - Vector::new(self.radius, self.radius);

        for (index, p) in self.points.iter().enumerate() {
            let posistion = camera.point_to_screen(offset, p.pos);
            let mesh = match &state.select {
                SelectState::Hover(hover_index) => if index == *hover_index {
                    mesh.hover.clone()
                } else {
                    mesh.main.clone()
                },
                SelectState::Drag(drag) => if index == drag.index {
                    mesh.hover.clone()
                } else {
                    mesh.main.clone()
                },
                SelectState::None => mesh.main.clone(),
            };

            renderer.with_translation(posistion, |renderer| renderer.draw_mesh(mesh));
        }
    }
}

impl<'a, Message: 'a> From<PointsEditorWidget<'a, Message>> for Element<'a, Message> {
    fn from(editor: PointsEditorWidget<'a, Message>) -> Self {
        Self::new(editor)
    }
}

trait ViewPortOffset {
    fn offset(&self) -> Vector<f32>;
}

impl ViewPortOffset for Rectangle {
    fn offset(&self) -> Vector {
        Vector::new(
            self.x + 0.5 * self.width,
            self.y + 0.5 * self.height,
        )
    }
}