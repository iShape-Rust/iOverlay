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
        println!("on event {:?}", event);

        let camera = if let State::Some(stete_box) = &mut tree.state {
            let state = stete_box.downcast_mut::<SubjClipEditorState>().unwrap();
            if let Some(camera) = state.init_camera(&self.stateless.editor_points, viewport) {
                camera
            } else {
                return event::Status::Ignored;
            }
        } else {
            return event::Status::Ignored;
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
        _tree: &Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        // let camera = if let Some(camera) = &self.state.camera {
        //     camera
        // };

        use iced::advanced::graphics::mesh::{
            self, Mesh, Renderer as _, SolidVertex2D,
        };
        use iced::advanced::Renderer as _;

        let bounds = layout.bounds();
        println!("bounds: {:?}", bounds);
        // R O Y G B I V
        let color_r = [1.0, 0.0, 0.0, 1.0];
        let color_o = [1.0, 0.5, 0.0, 1.0];
        let color_y = [1.0, 1.0, 0.0, 1.0];
        let color_g = [0.0, 1.0, 0.0, 1.0];
        let color_gb = [0.0, 1.0, 0.5, 1.0];
        let color_b = [0.0, 0.2, 1.0, 1.0];
        let color_i = [0.5, 0.0, 1.0, 1.0];
        let color_v = [0.75, 0.0, 0.5, 1.0];

        let posn_center = {
            if let Some(cursor_position) = cursor.position_in(bounds) {
                [cursor_position.x, cursor_position.y]
            } else {
                [bounds.width / 2.0, bounds.height / 2.0]
            }
        };

        let posn_tl = [0.0, 0.0];
        let posn_t = [bounds.width / 2.0, 0.0];
        let posn_tr = [bounds.width, 0.0];
        let posn_r = [bounds.width, bounds.height / 2.0];
        let posn_br = [bounds.width, bounds.height];
        let posn_b = [(bounds.width / 2.0), bounds.height];
        let posn_bl = [0.0, bounds.height];
        let posn_l = [0.0, bounds.height / 2.0];

        let mesh = Mesh::Solid {
            buffers: mesh::Indexed {
                vertices: vec![
                    SolidVertex2D {
                        position: posn_center,
                        color: color::pack([1.0, 1.0, 1.0, 1.0]),
                    },
                    SolidVertex2D {
                        position: posn_tl,
                        color: color::pack(color_r),
                    },
                    SolidVertex2D {
                        position: posn_t,
                        color: color::pack(color_o),
                    },
                    SolidVertex2D {
                        position: posn_tr,
                        color: color::pack(color_y),
                    },
                    SolidVertex2D {
                        position: posn_r,
                        color: color::pack(color_g),
                    },
                    SolidVertex2D {
                        position: posn_br,
                        color: color::pack(color_gb),
                    },
                    SolidVertex2D {
                        position: posn_b,
                        color: color::pack(color_b),
                    },
                    SolidVertex2D {
                        position: posn_bl,
                        color: color::pack(color_i),
                    },
                    SolidVertex2D {
                        position: posn_l,
                        color: color::pack(color_v),
                    },
                ],
                indices: vec![
                    0, 1, 2, // TL
                    0, 2, 3, // T
                    0, 3, 4, // TR
                    0, 4, 5, // R
                    0, 5, 6, // BR
                    0, 6, 7, // B
                    0, 7, 8, // BL
                    0, 8, 1, // L
                ],
            },
            transformation: Transformation::IDENTITY,
            clip_bounds: Rectangle::INFINITE,
        };

        renderer.with_translation(
            Vector::new(bounds.x, bounds.y),
            |renderer| {
                renderer.draw_mesh(mesh);
            },
        );
    }
}

impl<'a, Message: 'a> From<SubjClipEditorWidget<'a, Message>> for Element<'a, Message> {
    fn from(editor: SubjClipEditorWidget<'a, Message>) -> Self {
        Self::new(editor)
    }
}