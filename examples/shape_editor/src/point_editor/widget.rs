use crate::geom::camera::Camera;
use crate::geom::viewport::ViewPortExt;
use crate::point_editor::state::SelectState;
use crate::point_editor::state::PointsEditorState;
use crate::point_editor::point::EditorPoint;
use iced::advanced::widget::tree::State;
use iced::advanced::widget::tree;
use iced::advanced::layout::{self, Layout};
use iced::advanced::{Clipboard, renderer, Shell};
use iced::advanced::widget::{Tree, Widget};
use iced::{Event, event, mouse, Point, Color};
use iced::{Element, Length, Rectangle, Renderer, Size, Theme, Vector};

#[derive(Debug, Clone)]
pub(crate) struct PointEditUpdate {
    pub(crate) point: EditorPoint,
    pub(crate) index: usize,
}

pub(crate) struct PointsEditorWidget<'a, Message> {
    pub(super) points: &'a Vec<EditorPoint>,
    pub(super) camera: Camera,
    main_color: Option<Color>,
    drag_color: Option<Color>,
    hover_color: Option<Color>,
    pub(super) radius: f32,
    on_update: Box<dyn Fn(PointEditUpdate) -> Message + 'a>,
}

impl<'a, Message> PointsEditorWidget<'a, Message> {
    pub(crate) fn new(points: &'a Vec<EditorPoint>, camera: Camera, on_update: impl Fn(PointEditUpdate) -> Message + 'a) -> Self {
        Self {
            points,
            camera,
            radius: 15.0,
            main_color: None,
            hover_color: None,
            drag_color: None,
            on_update: Box::new(on_update),
        }
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
            height: Length::Fill,
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
                    self.hover_color.unwrap_or(Color::from_rgb(1.0, 0.6, 0.4)),
                    self.drag_color.unwrap_or(Color::from_rgb(1.0, 0.2, 0.2)),
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
                            return event::Status::Captured;
                        }
                    }
                }
                mouse::Event::ButtonPressed(mouse::Button::Left) => {
                    let position = cursor.position().unwrap_or(Point::ORIGIN);
                    if bounds.contains(position) {
                        let cursor = Vector { x: position.x, y: position.y };
                        let offset = bounds.offset();
                        if state.mouse_press(
                            &*self,
                            cursor,
                            offset,
                        ) {
                            return event::Status::Captured;
                        }
                    }
                }
                mouse::Event::ButtonReleased(mouse::Button::Left) => {
                    let position = cursor.position().unwrap_or(Point::ORIGIN);
                    let cursor = Vector { x: position.x, y: position.y };
                    let offset = bounds.offset();
                    if state.mouse_release(
                        &*self,
                        cursor,
                        offset,
                    ) {
                        return event::Status::Captured;
                    }
                }
                _ => {},
            },
            _ => {},
        }

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
        let state = tree.state.downcast_ref::<PointsEditorState>();

        let mesh = if let Some(mesh) = &state.mesh_cache { mesh } else { return; };

        use iced::advanced::graphics::mesh::Renderer as _;
        use iced::advanced::Renderer as _;

        let bounds = layout.bounds();
        let offset = bounds.offset() - Vector::new(self.radius, self.radius);

        for (index, p) in self.points.iter().enumerate() {
            let posistion = self.camera.point_to_screen_offset(offset, p.pos);
            let mesh = match &state.select {
                SelectState::Hover(hover_index) => if index == *hover_index {
                    mesh.hover.clone()
                } else {
                    mesh.main.clone()
                },
                SelectState::Drag(drag) => if index == drag.index {
                    mesh.drag.clone()
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