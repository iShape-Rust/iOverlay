use crate::geom::camera::Camera;
use crate::point_editor::state::SelectState;
use crate::point_editor::state::PointsEditorState;
use crate::point_editor::point::EditorPoint;
use iced::advanced::widget::tree::State;
use iced::advanced::widget::tree;
use iced::advanced::layout::{self, Layout};
use iced::advanced::{Clipboard, renderer, Shell};
use iced::advanced::widget::{Tree, Widget};
use iced::{Event, event, mouse, Point, Color};
use iced::{Element, Length, Rectangle, Renderer, Size, Theme};


#[derive(Debug, Clone)]
pub(crate) struct PointEditUpdate {
    pub(crate) point: EditorPoint,
    pub(crate) index: usize,
}

pub(crate) struct PointsEditorWidget<'a, Message> {
    pub(super) points: &'a Vec<EditorPoint>,
    pub(super) camera: Camera,
    main_color: Color,
    drag_color: Color,
    hover_color: Color,
    pub(super) mesh_radius: f32,
    pub(super) hover_radius: f32,
    on_update: Box<dyn Fn(PointEditUpdate) -> Message + 'a>,
}

impl<'a, Message> PointsEditorWidget<'a, Message> {
    pub(crate) fn new(points: &'a Vec<EditorPoint>, camera: Camera, on_update: impl Fn(PointEditUpdate) -> Message + 'a) -> Self {
        let binding = Theme::default();
        let palette = binding.extended_palette();

        let (main_color, hover_color, drag_color) = if palette.is_dark {
            (Color::WHITE, palette.primary.base.color, palette.primary.weak.color)
        } else {
            (Color::BLACK, palette.primary.base.color, palette.primary.weak.color)
        };

        Self {
            points,
            camera,
            mesh_radius: 6.0,
            hover_radius: 12.0,
            main_color,
            hover_color,
            drag_color,
            on_update: Box::new(on_update),
        }
    }

    pub(crate) fn set_hover_color(mut self, color: Color) -> Self {
        self.hover_color = color;
        self
    }

    pub(crate) fn set_drag_color(mut self, color: Color) -> Self {
        self.drag_color = color;
        self
    }
}

impl<Message> Widget<Message, Theme, Renderer> for PointsEditorWidget<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<PointsEditorState>()
    }

    fn state(&self) -> State {
        State::new(PointsEditorState::default())
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
                    self.mesh_radius,
                    self.main_color,
                    self.hover_color,
                    self.drag_color,
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
        _viewport: &Rectangle,
    ) -> event::Status {
        let state = tree.state.downcast_mut::<PointsEditorState>();


        let bounds = layout.bounds();
        if let Event::Mouse(mouse_event) = event {
            match mouse_event {
                mouse::Event::CursorMoved { position } => {
                    if bounds.contains(position) {
                        let view_cursor = position - bounds.position();
                        if let Some(updated_point) = state.mouse_move(
                            &*self,
                            view_cursor,
                        ) {
                            shell.publish((self.on_update)(updated_point));
                            return event::Status::Captured;
                        }
                    }
                }
                mouse::Event::ButtonPressed(mouse::Button::Left) => {
                    let position = cursor.position().unwrap_or(Point::ORIGIN);
                    if bounds.contains(position) {
                        let view_cursor = position - bounds.position();
                        if state.mouse_press(
                            &*self,
                            view_cursor,
                        ) {
                            return event::Status::Captured;
                        }
                    }
                }
                mouse::Event::ButtonReleased(mouse::Button::Left) => {
                    let position = cursor.position().unwrap_or(Point::ORIGIN);
                    let view_cursor = position - bounds.position();
                    if state.mouse_release(
                        &*self,
                        view_cursor,
                    ) {
                        return event::Status::Captured;
                    }
                }
                _ => {}
            }
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

        let offset = layout.position() - Point::new(self.mesh_radius, self.mesh_radius);

        for (index, p) in self.points.iter().enumerate() {
            let position = self.camera.world_to_screen(offset, p.pos);
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

            renderer.with_translation(position, |renderer| renderer.draw_mesh(mesh));
        }
    }
}

impl<'a, Message: 'a> From<PointsEditorWidget<'a, Message>> for Element<'a, Message> {
    fn from(editor: PointsEditorWidget<'a, Message>) -> Self {
        Self::new(editor)
    }
}