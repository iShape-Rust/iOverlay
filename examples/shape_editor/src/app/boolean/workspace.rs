use crate::app::boolean::content::BooleanMessage;
use crate::app::boolean::editor::widget::PolygonEditorWidget;
use iced::{Alignment, Length, Padding};
use iced::widget::{Button, Column, Container, pick_list, Row, Space, Stack, Text};
use crate::app::design::{style_action_button, style_action_button_selected, style_sheet_background};
use crate::app::main::{EditorApp, Message};

impl EditorApp {
    pub(crate) fn boolean_workspace(&self) -> Container<Message> {
        Container::new(
            Stack::new()
                .push(
                    Container::new(PolygonEditorWidget::new())
                        // Could you add here event handler for hover, drag, pressed
                        .width(Length::Fill)
                        .height(Length::Fill)
                )
                .push(
                    Container::new(self.boolean_control())
                        .width(Length::Shrink)
                        .height(Length::Shrink)
                        .padding(Padding::new(8.0))
                )
        )
            .style(style_sheet_background)
    }
}
