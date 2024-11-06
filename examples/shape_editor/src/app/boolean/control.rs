use i_triangle::i_overlay::core::fill_rule::FillRule;
use i_triangle::i_overlay::core::overlay_rule::OverlayRule;
use crate::app::boolean::content::BooleanMessage;
use iced::{Alignment, Length};
use iced::widget::{Column, Container, pick_list, Row, Space, Text};
use crate::app::main::{EditorApp, AppMessage};

impl EditorApp {
    pub(crate) fn boolean_control(&self) -> Column<AppMessage> {
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
                            Some(self.state.boolean.solver),
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
                            Some(self.state.boolean.fill),
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
                            Some(self.state.boolean.mode),
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
    AppMessage::Bool(BooleanMessage::FillSelected(option))
}

fn on_select_mode(option: ModeOption) -> AppMessage {
    AppMessage::Bool(BooleanMessage::ModeSelected(option))
}

fn on_select_solver(option: SolverOption) -> AppMessage {
    AppMessage::Bool(BooleanMessage::SolverSelected(option))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SolverOption {
    #[default]
    Auto,
    Average,
    Precise,
}

impl SolverOption {
    const ALL: [SolverOption; 3] = [
        SolverOption::Auto,
        SolverOption::Average,
        SolverOption::Precise
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FillOption {
    #[default]
    NonZero,
    EvenOdd,
    Positive,
    Negative,
}

impl FillOption {
    const ALL: [FillOption; 4] = [
        FillOption::NonZero,
        FillOption::EvenOdd,
        FillOption::Positive,
        FillOption::Negative,
    ];

    pub(crate) fn to_fill_rule(&self) -> FillRule {
        match self {
            FillOption::NonZero => FillRule::NonZero,
            FillOption::EvenOdd => FillRule::EvenOdd,
            FillOption::Positive => FillRule::Positive,
            FillOption::Negative => FillRule::Negative,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ModeOption {
    #[default]
    Edit,
    Debug,
    Subject,
    Clip,
    Intersect,
    Union,
    Difference,
    InverseDifference,
    Xor,
}

impl ModeOption {
    const ALL: [ModeOption; 9] = [
        ModeOption::Edit,
        ModeOption::Debug,
        ModeOption::Subject,
        ModeOption::Clip,
        ModeOption::Intersect,
        ModeOption::Union,
        ModeOption::Difference,
        ModeOption::InverseDifference,
        ModeOption::Xor,
    ];

    pub(crate) fn to_overlay_rule(&self) -> Option<OverlayRule> {
        match self {
            ModeOption::Subject => Some(OverlayRule::Subject),
            ModeOption::Clip => Some(OverlayRule::Clip),
            ModeOption::Intersect => Some(OverlayRule::Intersect),
            ModeOption::Union => Some(OverlayRule::Union),
            ModeOption::Difference => Some(OverlayRule::Difference),
            ModeOption::InverseDifference => Some(OverlayRule::InverseDifference),
            ModeOption::Xor => Some(OverlayRule::Xor),
            _ => None
        }
    }
}

impl std::fmt::Display for SolverOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SolverOption::Auto => "Auto",
                SolverOption::Average => "Average",
                SolverOption::Precise => "Precise",
            }
        )
    }
}

impl std::fmt::Display for FillOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FillOption::NonZero => "NonZero",
                FillOption::EvenOdd => "EvenOdd",
                FillOption::Positive => "Positive",
                FillOption::Negative => "Negative",
            }
        )
    }
}

impl std::fmt::Display for ModeOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ModeOption::Edit => "Edit",
                ModeOption::Debug => "Debug",
                ModeOption::Subject => "Subject",
                ModeOption::Clip => "Clip",
                ModeOption::Intersect => "Intersect",
                ModeOption::Union => "Union",
                ModeOption::Difference => "Difference",
                ModeOption::InverseDifference => "InverseDifference",
                ModeOption::Xor => "Xor",
            }
        )
    }
}