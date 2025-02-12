use crate::app::main::{AppMessage, EditorApp};
use crate::app::stroke::content::StrokeMessage;
use iced::widget::{checkbox, pick_list, slider, Column, Container, Row, Space, Text};
use iced::{Alignment, Length, Padding};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum CapOption {
    #[default]
    Butt,
    Round,
    Square,
    Arrow,
}

impl CapOption {
    const ALL: [CapOption; 4] = [CapOption::Butt, CapOption::Round, CapOption::Square, CapOption::Arrow];
}

impl std::fmt::Display for CapOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CapOption::Butt => "Butt",
                CapOption::Round => "Round",
                CapOption::Square => "Square",
                CapOption::Arrow => "Arrow",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum JoinOption {
    #[default]
    Miter,
    Round,
    Bevel,
}

impl JoinOption {
    const ALL: [JoinOption; 3] = [JoinOption::Miter, JoinOption::Round, JoinOption::Bevel];
}

impl std::fmt::Display for JoinOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                JoinOption::Miter => "Miter",
                JoinOption::Round => "Round",
                JoinOption::Bevel => "Bevel",
            }
        )
    }
}

impl EditorApp {
    pub(crate) fn stroke_control(&self) -> Column<AppMessage> {
        let width_list = Row::new()
            .push(
                Text::new("Stroke Width:")
                    .width(Length::Fixed(120.0))
                    .height(Length::Fill)
                    .align_y(Alignment::Center),
            )
            .push(
                Container::new(
                    slider(0.1f32..=10.0f32, self.state.stroke.width, on_update_width).step(0.01f32)
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
                        Some(self.state.stroke.start_cap),
                        on_select_start_cap,
                    )
                    .width(Length::Fixed(160.0)),
                )
                .height(Length::Fill)
                .align_y(Alignment::Center),
            )
            .height(Length::Fixed(40.0));

        if self.state.stroke.start_cap == CapOption::Round {
            let slider = slider(1..=100, self.state.stroke.start_cap_value, on_update_start_cap_value)
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
                        Some(self.state.stroke.end_cap),
                        on_select_end_cap,
                    )
                        .width(Length::Fixed(160.0)),
                )
                    .height(Length::Fill)
                    .align_y(Alignment::Center),
            )
            .height(Length::Fixed(40.0));

        if self.state.stroke.end_cap == CapOption::Round {
            let slider = slider(1..=100, self.state.stroke.end_cap_value, on_update_end_cap_value)
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
                Text::new("Line Join:")
                    .width(Length::Fixed(120.0))
                    .height(Length::Fill)
                    .align_y(Alignment::Center),
            )
            .push(
                Container::new(
                    pick_list(
                        &JoinOption::ALL[..],
                        Some(self.state.stroke.join),
                        on_select_join,
                    )
                    .width(Length::Fixed(160.0)),
                )
                .height(Length::Fill)
                .align_y(Alignment::Center),
            )
            .height(Length::Fixed(40.0));

        if self.state.stroke.join != JoinOption::Bevel {
            let slider = slider(1..=100, self.state.stroke.join_value, on_update_join_value)
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

        Column::new()
            .push(width_list)
            .push(start_cap_pick_list)
            .push(Space::new(Length::Shrink, Length::Fixed(4.0)))
            .push(end_cap_pick_list)
            .push(Space::new(Length::Shrink, Length::Fixed(4.0)))
            .push(join_pick_list)
            .push(
                checkbox("Is Closed", self.state.stroke.is_closed)
                    .on_toggle(on_set_is_closed)

            )
    }
}

fn on_update_width(value: f32) -> AppMessage {
    AppMessage::Stroke(StrokeMessage::WidthValueUpdated(value))
}

fn on_set_is_closed(value: bool) -> AppMessage {
    AppMessage::Stroke(StrokeMessage::IsClosedUpdated(value))
}

fn on_select_start_cap(option: CapOption) -> AppMessage {
    AppMessage::Stroke(StrokeMessage::StartCapSelected(option))
}

fn on_update_start_cap_value(value: u8) -> AppMessage {
    AppMessage::Stroke(StrokeMessage::StartCapValueUpdated(value))
}

fn on_select_end_cap(option: CapOption) -> AppMessage {
    AppMessage::Stroke(StrokeMessage::EndCapSelected(option))
}

fn on_update_end_cap_value(value: u8) -> AppMessage {
    AppMessage::Stroke(StrokeMessage::EndCapValueUpdated(value))
}

fn on_select_join(option: JoinOption) -> AppMessage {
    AppMessage::Stroke(StrokeMessage::JoinSelected(option))
}

fn on_update_join_value(value: u8) -> AppMessage {
    AppMessage::Stroke(StrokeMessage::JoinValueUpdated(value))
}