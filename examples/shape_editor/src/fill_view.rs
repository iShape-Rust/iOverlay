use iced::Color as RGBColor;
use iced::advanced::{Layout, renderer, Widget};
use iced::{Element, Length, Rectangle, Size};
use iced::advanced::layout::{Limits, Node};
use iced::advanced::renderer::Style;
use iced::advanced::widget::Tree;
use iced::Background::Color;
use iced::mouse::Cursor;

pub(crate) struct FillView {
    pub(crate) color: RGBColor
}

impl FillView {
    pub fn new(color: RGBColor) -> Self {
        Self { color }
    }
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for FillView
where
    Renderer: renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Fill,
            height: Length::Fill,
        }
    }

    fn layout(&self, _tree: &mut Tree, _renderer: &Renderer, limits: &Limits) -> Node {
        Node::new(limits.max())
    }

    fn draw(&self, _tree: &Tree, renderer: &mut Renderer, _theme: &Theme, _style: &Style, layout: Layout<'_>, _cursor: Cursor, _viewport: &Rectangle) {
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border: Default::default(),
                shadow: Default::default(),
            },
            Color(self.color),
        );
    }
}

impl<'a, Message> From<FillView> for Element<'a, Message> {
    fn from(item: FillView) -> Self {
        Self::new(item)
    }
}