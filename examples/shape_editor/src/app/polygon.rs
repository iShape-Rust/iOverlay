use iced::{Alignment, Length, Padding};
use iced::widget::{Button, Column, Container, pick_list, Row, Space, Stack, Text};
use crate::app::design::{style_action_button, style_action_button_selected, style_sheet_background};
use crate::app::main::{EditorApp, Message};
use crate::app::polygon_editor::{PolygonEditor, PolygonEditorState};

pub(super) struct PolygonState {
    test: usize,
    fill: FillOption,
    mode: ModeOption,
    solver: SolverOption,
    editor: PolygonEditorState
}

#[derive(Debug, Clone)]
pub(super) enum PolygonMessage {
    TestSelected(usize),
    FillSelected(FillOption),
    ModeSelected(ModeOption),
    SolverSelected(SolverOption),
}

impl EditorApp {
    pub(super) fn polygon_tests_list(&self) -> Column<Message> {
        let count = self.app_resource.polygon.count;
        let mut column = Column::new().push(Space::new(Length::Fill, Length::Fixed(2.0)));
        for index in 0..count {
            let is_selected = self.state.polygon.test == index;

            column = column.push(
                Container::new(
                    Button::new(Text::new(format!("test_{}", index)))
                        .width(Length::Fill)
                        .on_press(Message::Polygon(PolygonMessage::TestSelected(index)))
                        .style(if is_selected { style_action_button_selected } else { style_action_button })
                ).padding(self.design.action_padding())
            );
        }

        column
    }

    pub(super) fn polygon_test_view(&self) -> Container<Message> {
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
                            Some(self.state.polygon.solver),
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
                            Some(self.state.polygon.fill),
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
                            Some(self.state.polygon.mode),
                            on_select_mode,
                        ).width(Length::Fixed(160.0))
                    )
                        .height(Length::Fill)
                        .align_y(Alignment::Center)
                ).height(Length::Fixed(40.0));

        Container::new(
            Stack::new()
                .push(
                    Container::new(PolygonEditor::new(&self.state.polygon.editor))
                        // Could you add here event handler for hover, drag, pressed
                        .width(Length::Fill)
                        .height(Length::Fill)
                )
                .push(
                    Container::new(Column::new()
                        .push(solver_pick_list)
                        .push(Space::new(Length::Shrink, Length::Fixed(4.0)))
                        .push(fill_pick_list)
                        .push(Space::new(Length::Shrink, Length::Fixed(4.0)))
                        .push(mode_pick_list)
                    )
                        .width(Length::Shrink)
                        .height(Length::Shrink)
                        .padding(Padding::new(8.0))
                )
        )
            .style(style_sheet_background)
    }

    pub(super) fn update_polygon(&mut self, message: PolygonMessage) {
        match message {
            PolygonMessage::TestSelected(index) => self.state.polygon.test = index,
            PolygonMessage::SolverSelected(solver) => self.state.polygon.solver = solver,
            PolygonMessage::FillSelected(fill) => self.state.polygon.fill = fill,
            PolygonMessage::ModeSelected(mode) => self.state.polygon.mode = mode,
        }
    }
}

fn on_select_fill(option: FillOption) -> Message {
    Message::Polygon(PolygonMessage::FillSelected(option))
}

fn on_select_mode(option: ModeOption) -> Message {
    Message::Polygon(PolygonMessage::ModeSelected(option))
}

fn on_select_solver(option: SolverOption) -> Message {
    Message::Polygon(PolygonMessage::SolverSelected(option))
}

impl Default for PolygonState {
    fn default() -> Self {
        PolygonState {
            test: 0,
            fill: FillOption::NonZero,
            mode: ModeOption::Edit,
            solver: SolverOption::Auto,
            editor: Default::default(),
        }
    }
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