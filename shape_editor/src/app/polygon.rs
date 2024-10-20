use iced::{Alignment, Length, Padding};
use iced::widget::{Button, button, Column, Container, Row, Text};
use crate::app::design::{action_button, action_button_selected};
use crate::app::main::{EditorApp, MainMessage, Message};

pub(super) struct PolygonState {
    selected_test: usize,
}

#[derive(Debug, Clone)]
pub(super) enum PolygonMessage {
    TestSelected(usize),
}

impl EditorApp {
    pub(super) fn polygon_tests_list(&self) -> Column<Message> {
        let count = self.appReosurce.polygon.count;
        let mut column = Column::new();
        for index in 0..count {
            let is_selected = self.state.polygon.selected_test == index;

            column = column.push(
                Container::new(
                    Button::new(Text::new(format!("test_{}", index)))
                        .width(Length::Fill)
                        .on_press(Message::Polygon(PolygonMessage::TestSelected(index)))
                        .style(if is_selected { action_button_selected } else { action_button })
                ).padding(self.design.action_padding())
            );
        }

        column
    }

    // pub(super) fn polygon_test_view(&self, index: usize) -> Column<Message> {
    //
    // }

    pub(super) fn update_polygon(&mut self, message: PolygonMessage) {
        match message {
            PolygonMessage::TestSelected(index) => self.state.polygon.selected_test = index
        }
    }
}

impl Default for PolygonState {
    fn default() -> Self {
        PolygonState { selected_test: 0 }
    }
}