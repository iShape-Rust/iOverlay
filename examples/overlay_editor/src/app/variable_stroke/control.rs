use crate::app::main::{AppMessage, EditorApp};
use crate::app::stroke::control::{CapOption, JoinOption};
use crate::app::variable_stroke::content::VariableStrokeMessage;
use iced::widget::{checkbox, pick_list, slider, Column, Container, Row, Space, Text};
use iced::{Alignment, Length, Padding};

impl EditorApp {
    pub(crate) fn variable_stroke_control(&self) -> Column<'_, AppMessage> {
        let width_scale = Row::new()
            .push(
                Text::new("Width Scale:")
                    .width(Length::Fixed(120.0))
                    .height(Length::Fill)
                    .align_y(Alignment::Center),
            )
            .push(
                Container::new(
                    slider(
                        0.1f32..=3.0f32,
                        self.state.variable_stroke.width_scale,
                        on_update_width_scale,
                    )
                    .step(0.01f32),
                )
                .width(160)
                .height(Length::Fill)
                .align_y(Alignment::Center),
            )
            .height(Length::Fixed(40.0));

        let mut start_cap_pick_list = Row::new()
            .push(
                Text::new("Start Cap:")
                    .width(Length::Fixed(120.0))
                    .height(Length::Fill)
                    .align_y(Alignment::Center),
            )
            .push(
                Container::new(
                    pick_list(
                        &CapOption::ALL[..],
                        Some(self.state.variable_stroke.start_cap),
                        on_select_start_cap,
                    )
                    .width(Length::Fixed(160.0)),
                )
                .height(Length::Fill)
                .align_y(Alignment::Center),
            )
            .height(Length::Fixed(40.0));

        if self.state.variable_stroke.start_cap == CapOption::Round {
            let slider = slider(
                1..=100,
                self.state.variable_stroke.start_cap_value,
                on_update_start_cap_value,
            )
            .default(50)
            .shift_step(5);

            start_cap_pick_list = start_cap_pick_list.push(
                Container::new(slider)
                    .padding(Padding::new(0.0).left(20.0))
                    .width(250)
                    .height(Length::Fill)
                    .align_y(Alignment::Center),
            );
        }

        let mut end_cap_pick_list = Row::new()
            .push(
                Text::new("End Cap:")
                    .width(Length::Fixed(120.0))
                    .height(Length::Fill)
                    .align_y(Alignment::Center),
            )
            .push(
                Container::new(
                    pick_list(
                        &CapOption::ALL[..],
                        Some(self.state.variable_stroke.end_cap),
                        on_select_end_cap,
                    )
                    .width(Length::Fixed(160.0)),
                )
                .height(Length::Fill)
                .align_y(Alignment::Center),
            )
            .height(Length::Fixed(40.0));

        if self.state.variable_stroke.end_cap == CapOption::Round {
            let slider = slider(
                1..=100,
                self.state.variable_stroke.end_cap_value,
                on_update_end_cap_value,
            )
            .default(50)
            .shift_step(5);

            end_cap_pick_list = end_cap_pick_list.push(
                Container::new(slider)
                    .padding(Padding::new(0.0).left(20.0))
                    .width(250)
                    .height(Length::Fill)
                    .align_y(Alignment::Center),
            );
        }

        let mut join_pick_list = Row::new()
            .push(
                Text::new("Join:")
                    .width(Length::Fixed(120.0))
                    .height(Length::Fill)
                    .align_y(Alignment::Center),
            )
            .push(
                Container::new(
                    pick_list(
                        &JoinOption::ALL[..],
                        Some(self.state.variable_stroke.join),
                        on_select_join,
                    )
                    .width(Length::Fixed(160.0)),
                )
                .height(Length::Fill)
                .align_y(Alignment::Center),
            )
            .height(Length::Fixed(40.0));

        if self.state.variable_stroke.join != JoinOption::Bevel {
            let slider = slider(
                1..=100,
                self.state.variable_stroke.join_value,
                on_update_join_value,
            )
            .default(50)
            .shift_step(5);

            join_pick_list = join_pick_list.push(
                Container::new(slider)
                    .padding(Padding::new(0.0).left(20.0))
                    .width(250)
                    .height(Length::Fill)
                    .align_y(Alignment::Center),
            );
        }

        let is_closed = Row::new()
            .push(
                Text::new("Closed:")
                    .width(Length::Fixed(120.0))
                    .height(Length::Fill)
                    .align_y(Alignment::Center),
            )
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
            .push(start_cap_pick_list)
            .push(
                Space::new()
                    .width(Length::Shrink)
                    .height(Length::Fixed(4.0)),
            )
            .push(end_cap_pick_list)
            .push(join_pick_list)
            .push(is_closed)
    }
}

fn on_update_width_scale(value: f32) -> AppMessage {
    AppMessage::VariableStroke(VariableStrokeMessage::WidthScaleUpdated(value))
}

fn on_select_start_cap(option: CapOption) -> AppMessage {
    AppMessage::VariableStroke(VariableStrokeMessage::StartCapSelected(option))
}

fn on_update_start_cap_value(value: u8) -> AppMessage {
    AppMessage::VariableStroke(VariableStrokeMessage::StartCapValueUpdated(value))
}

fn on_select_end_cap(option: CapOption) -> AppMessage {
    AppMessage::VariableStroke(VariableStrokeMessage::EndCapSelected(option))
}

fn on_update_end_cap_value(value: u8) -> AppMessage {
    AppMessage::VariableStroke(VariableStrokeMessage::EndCapValueUpdated(value))
}

fn on_select_join(option: JoinOption) -> AppMessage {
    AppMessage::VariableStroke(VariableStrokeMessage::JoinSelected(option))
}

fn on_update_join_value(value: u8) -> AppMessage {
    AppMessage::VariableStroke(VariableStrokeMessage::JoinValueUpdated(value))
}

fn on_set_is_closed(is_closed: bool) -> AppMessage {
    AppMessage::VariableStroke(VariableStrokeMessage::IsClosedUpdated(is_closed))
}
