use crate::app::main::{AppMessage, EditorApp};
use crate::app::outline::content::OutlineMessage;
use iced::widget::{pick_list, slider, Column, Container, Row, Space, Text};
use iced::{Alignment, Length, Padding};

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
    pub(crate) fn outline_control(&self) -> Column<AppMessage> {
        let outer_offset_list = Row::new()
            .push(
                Text::new("Outer Offset:")
                    .width(Length::Fixed(120.0))
                    .height(Length::Fill)
                    .align_y(Alignment::Center),
            )
            .push(
                Container::new(
                    slider(-50.0f32..=50.0f32, self.state.outline.outer_offset, on_update_outer_offset).step(0.01f32)
                )
                    .width(410)
                    .height(Length::Fill)
                    .align_y(Alignment::Center),
            )
            .height(Length::Fixed(40.0));
        let inner_offset_list = Row::new()
            .push(
                Text::new("Inner Offset:")
                    .width(Length::Fixed(120.0))
                    .height(Length::Fill)
                    .align_y(Alignment::Center),
            )
            .push(
                Container::new(
                    slider(-50.0f32..=50.0f32, self.state.outline.inner_offset, on_update_inner_offset).step(0.01f32)
                )
                    .width(410)
                    .height(Length::Fill)
                    .align_y(Alignment::Center),
            )
            .height(Length::Fixed(40.0));

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
                        Some(self.state.outline.join),
                        on_select_join,
                    )
                    .width(Length::Fixed(160.0)),
                )
                .height(Length::Fill)
                .align_y(Alignment::Center),
            )
            .height(Length::Fixed(40.0));

        if self.state.outline.join != JoinOption::Bevel {
            let slider = slider(1..=100, self.state.outline.join_value, on_update_join_value)
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
            .push(outer_offset_list)
            .push(inner_offset_list)
            .push(Space::new(Length::Shrink, Length::Fixed(4.0)))
            .push(join_pick_list)
    }
}

fn on_update_outer_offset(value: f32) -> AppMessage {
    AppMessage::Outline(OutlineMessage::OuterOffsetValueUpdated(value))
}

fn on_update_inner_offset(value: f32) -> AppMessage {
    AppMessage::Outline(OutlineMessage::InnerOffsetValueUpdated(value))
}

fn on_select_join(option: JoinOption) -> AppMessage {
    AppMessage::Outline(OutlineMessage::JoinSelected(option))
}

fn on_update_join_value(value: u8) -> AppMessage {
    AppMessage::Outline(OutlineMessage::JoinValueUpdated(value))
}