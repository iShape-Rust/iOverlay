use crate::app::main::{AppMessage, EditorApp};
use crate::app::variable_stroke::content::VariableStrokeMessage;
use iced::widget::{checkbox, slider, Column, Container, Row, Text};
use iced::{Alignment, Length};

impl EditorApp {
    pub(crate) fn variable_stroke_control(&self) -> Column<'_, AppMessage> {
        let width_scale = Row::new()
            .push(label("Width Scale:"))
            .push(
                Container::new(
                    slider(
                        0.1f32..=3.0f32,
                        self.state.variable_stroke.width_scale,
                        on_update_width_scale,
                    )
                    .step(0.01_f32),
                )
                .width(160)
                .height(Length::Fill)
                .align_y(Alignment::Center),
            )
            .height(Length::Fixed(40.0));

        let round_angle = Row::new()
            .push(label("Round Detail:"))
            .push(
                Container::new(
                    slider(
                        1..=50,
                        self.state.variable_stroke.round_angle,
                        on_update_round_angle,
                    )
                    .default(12)
                    .shift_step(5),
                )
                .width(160)
                .height(Length::Fill)
                .align_y(Alignment::Center),
            )
            .height(Length::Fixed(40.0));

        let is_closed = Row::new()
            .push(label("Closed:"))
            .push(
                Container::new(
                    checkbox(self.state.variable_stroke.is_closed).on_toggle(on_set_is_closed),
                )
                .height(Length::Fill)
                .align_y(Alignment::Center),
            )
            .height(Length::Fixed(40.0));

        Column::new()
            .push(width_scale)
            .push(round_angle)
            .push(is_closed)
    }
}

fn label(value: &str) -> Text<'_> {
    Text::new(value)
        .width(Length::Fixed(120.0))
        .height(Length::Fill)
        .align_y(Alignment::Center)
}

fn on_update_width_scale(value: f32) -> AppMessage {
    AppMessage::VariableStroke(VariableStrokeMessage::WidthScaleUpdated(value))
}

fn on_update_round_angle(value: u8) -> AppMessage {
    AppMessage::VariableStroke(VariableStrokeMessage::RoundAngleUpdated(value))
}

fn on_set_is_closed(is_closed: bool) -> AppMessage {
    AppMessage::VariableStroke(VariableStrokeMessage::IsClosedUpdated(is_closed))
}
