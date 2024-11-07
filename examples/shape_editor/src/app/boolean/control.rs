use i_triangle::i_overlay::core::overlay_rule::OverlayRule;
use crate::app::boolean::content::BooleanMessage;
use crate::app::fill_option::FillOption;
use crate::app::main::{EditorApp, AppMessage};
use crate::app::solver_option::SolverOption;
use iced::{Alignment, Length};
use iced::widget::{Column, Container, pick_list, Row, Space, Text};

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
                            &BooleanModeOption::ALL[..],
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

fn on_select_mode(option: BooleanModeOption) -> AppMessage {
    AppMessage::Bool(BooleanMessage::ModeSelected(option))
}

fn on_select_solver(option: SolverOption) -> AppMessage {
    AppMessage::Bool(BooleanMessage::SolverSelected(option))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BooleanModeOption {
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

impl BooleanModeOption {
    const ALL: [BooleanModeOption; 9] = [
        BooleanModeOption::Edit,
        BooleanModeOption::Debug,
        BooleanModeOption::Subject,
        BooleanModeOption::Clip,
        BooleanModeOption::Intersect,
        BooleanModeOption::Union,
        BooleanModeOption::Difference,
        BooleanModeOption::InverseDifference,
        BooleanModeOption::Xor,
    ];

    pub(crate) fn to_overlay_rule(&self) -> Option<OverlayRule> {
        match self {
            BooleanModeOption::Subject => Some(OverlayRule::Subject),
            BooleanModeOption::Clip => Some(OverlayRule::Clip),
            BooleanModeOption::Intersect => Some(OverlayRule::Intersect),
            BooleanModeOption::Union => Some(OverlayRule::Union),
            BooleanModeOption::Difference => Some(OverlayRule::Difference),
            BooleanModeOption::InverseDifference => Some(OverlayRule::InverseDifference),
            BooleanModeOption::Xor => Some(OverlayRule::Xor),
            _ => None
        }
    }
}

impl std::fmt::Display for BooleanModeOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BooleanModeOption::Edit => "Edit",
                BooleanModeOption::Debug => "Debug",
                BooleanModeOption::Subject => "Subject",
                BooleanModeOption::Clip => "Clip",
                BooleanModeOption::Intersect => "Intersect",
                BooleanModeOption::Union => "Union",
                BooleanModeOption::Difference => "Difference",
                BooleanModeOption::InverseDifference => "InverseDifference",
                BooleanModeOption::Xor => "Xor",
            }
        )
    }
}