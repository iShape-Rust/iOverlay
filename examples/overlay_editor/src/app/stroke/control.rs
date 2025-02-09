use iced::{Alignment, Length};
use iced::widget::{pick_list, Column, Container, Row, Space, Text};
use crate::app::fill_option::FillOption;
use crate::app::main::{AppMessage, EditorApp};
use crate::app::solver_option::SolverOption;
use crate::app::stroke::content::StrokeMessage;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum CapOption {
    #[default]
    Butt,
    Round,
    Square,
}

impl CapOption {
    const ALL: [CapOption; 3] = [
        CapOption::Butt,
        CapOption::Round,
        CapOption::Square,
    ];
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
    const ALL: [JoinOption; 3] = [
        JoinOption::Miter,
        JoinOption::Round,
        JoinOption::Bevel,
    ];
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
        let cap_pick_list =
            Row::new()
                .push(Text::new("Line Cap:")
                    .width(Length::Fixed(90.0))
                    .height(Length::Fill)
                    .align_y(Alignment::Center))
                .push(
                    Container::new(
                        pick_list(
                            &CapOption::ALL[..],
                            Some(self.state.stroke.cap),
                            on_select_cap,
                        ).width(Length::Fixed(160.0))
                    )
                        .height(Length::Fill)
                        .align_y(Alignment::Center)
                ).height(Length::Fixed(40.0));
        let join_pick_list =
            Row::new()
                .push(Text::new("Line Join:")
                    .width(Length::Fixed(90.0))
                    .height(Length::Fill)
                    .align_y(Alignment::Center))
                .push(
                    Container::new(
                        pick_list(
                            &JoinOption::ALL[..],
                            Some(self.state.stroke.join),
                            on_select_join,
                        ).width(Length::Fixed(160.0))
                    )
                        .height(Length::Fill)
                        .align_y(Alignment::Center)
                ).height(Length::Fixed(40.0));

        Column::new()
            .push(cap_pick_list)
            .push(Space::new(Length::Shrink, Length::Fixed(4.0)))
            .push(join_pick_list)
    }
}

fn on_select_cap(option: CapOption) -> AppMessage {
    AppMessage::Stroke(StrokeMessage::CapSelected(option))
}

fn on_select_join(option: JoinOption) -> AppMessage {
    AppMessage::Stroke(StrokeMessage::JoinSelected(option))
}
