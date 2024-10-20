use iced::Color as RGBColor;
use iced::advanced::{Layout, renderer, Widget};
use iced::{Element, Length, Pixels, Rectangle, Size};
use iced::advanced::layout::{Limits, Node};
use iced::advanced::renderer::Style;
use iced::advanced::widget::Tree;
use iced::Background::Color;
use iced::mouse::Cursor;

pub(crate) struct Square {
    pub(crate) color: RGBColor,
    pub(crate) radius: f32,
}

impl Square {
    pub fn new(radius: f32, color: RGBColor) -> Self {
        Self { color, radius }
    }
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Square
where
    Renderer: renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }

    fn layout(&self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        Node::new(Size::new(self.radius * 2.0, self.radius * 2.0))
    }

    fn draw(&self, tree: &Tree, renderer: &mut Renderer, theme: &Theme, style: &Style, layout: Layout<'_>, cursor: Cursor, viewport: &Rectangle) {
        let bounds = layout.bounds();

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: Default::default(),
                shadow: Default::default(),
            },
            Color(self.color),
        );
    }
}

impl<'a, Message> From<Square> for Element<'a, Message> {
    fn from(item: Square) -> Self {
        Self::new(item)
    }
}