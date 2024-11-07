use crate::app::string::content::StringMessage;
use crate::app::fill_option::FillOption;
use crate::app::main::{EditorApp, AppMessage};
use crate::app::solver_option::SolverOption;
use iced::{Alignment, Length};
use iced::widget::{Column, Container, pick_list, Row, Space, Text};

impl EditorApp {
    pub(crate) fn string_control(&self) -> Column<AppMessage> {
        let solver_pick_list =
            Row::new()
                .push(Text::new("Solver:")
                    .width(Length::Fixed(90.0))
                    .height(Length::Fill)
                    .align_y(Alignment::Center))
                .push(
                    Container::new(
                        pick_list(
                            &SolverOption::ALL[..],
                            Some(self.state.string.solver),
                            on_select_solver,
                        ).width(Length::Fixed(160.0))
                    )
                        .height(Length::Fill)
                        .align_y(Alignment::Center)
                ).height(Length::Fixed(40.0));

        let fill_pick_list =
            Row::new()
                .push(Text::new("Fill Rule:")
                    .width(Length::Fixed(90.0))
                    .height(Length::Fill)
                    .align_y(Alignment::Center))
                .push(
                    Container::new(
                        pick_list(
                            &FillOption::ALL[..],
                            Some(self.state.string.fill),
                            on_select_fill,
                        ).width(Length::Fixed(160.0))
                    )
                        .height(Length::Fill)
                        .align_y(Alignment::Center)
                ).height(Length::Fixed(40.0));

        let mode_pick_list =
            Row::new()
                .push(Text::new("Mode:")
                    .width(Length::Fixed(90.0))
                    .height(Length::Fill)
                    .align_y(Alignment::Center))
                .push(
                    Container::new(
                        pick_list(
                            &ModeOption::ALL[..],
                            Some(self.state.string.mode),
                            on_select_mode,
                        ).width(Length::Fixed(160.0))
                    )
                        .height(Length::Fill)
                        .align_y(Alignment::Center)
                ).height(Length::Fixed(40.0));

        Column::new()
            .push(solver_pick_list)
            .push(Space::new(Length::Shrink, Length::Fixed(4.0)))
            .push(fill_pick_list)
            .push(Space::new(Length::Shrink, Length::Fixed(4.0)))
            .push(mode_pick_list)
    }
}

fn on_select_fill(option: FillOption) -> AppMessage {
    AppMessage::String(StringMessage::FillSelected(option))
}

fn on_select_mode(option: ModeOption) -> AppMessage {
    AppMessage::String(StringMessage::ModeSelected(option))
}

fn on_select_solver(option: SolverOption) -> AppMessage {
    AppMessage::String(StringMessage::SolverSelected(option))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum ModeOption {
    #[default]
    Edit,
    Debug,
    Slice,
    ClipDirect,
    ClipInvert,
}

impl ModeOption {
    const ALL: [ModeOption; 5] = [
        ModeOption::Edit,
        ModeOption::Debug,
        ModeOption::Slice,
        ModeOption::ClipDirect,
        ModeOption::ClipInvert,
    ];
}

impl std::fmt::Display for ModeOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ModeOption::Edit => "Edit",
                ModeOption::Debug => "Debug",
                ModeOption::Slice => "Slice",
                ModeOption::ClipDirect => "ClipDirect",
                ModeOption::ClipInvert => "ClipInvert",
            }
        )
    }
}