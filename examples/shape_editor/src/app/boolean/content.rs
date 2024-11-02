use crate::app::design::style_second_background;
use iced::widget::scrollable;
use crate::app::boolean::control::ModeOption;
use crate::app::boolean::control::FillOption;
use crate::app::boolean::control::SolverOption;
use crate::app::boolean::editor::widget::PolygonEditorWidget;
use iced::{Alignment, Length, Padding};
use iced::widget::{Button, Column, Container, pick_list, Row, Space, Stack, Text};
use crate::app::design::{style_action_button, style_action_button_selected, style_sheet_background};
use crate::app::main::{EditorApp, Message};

pub(crate) struct PolygonState {
    pub(crate) test: usize,
    pub(crate) fill: FillOption,
    pub(crate) mode: ModeOption,
    pub(crate) solver: SolverOption,
    pub(crate) editor: PolygonEditorState,
}

pub(crate) struct PolygonEditorState {}

#[derive(Debug, Clone)]
pub(crate) enum BooleanMessage {
    TestSelected(usize),
    FillSelected(FillOption),
    ModeSelected(ModeOption),
    SolverSelected(SolverOption),
}

impl EditorApp {
    fn sidebar(&self) -> Column<Message> {
        let count = self.app_resource.polygon.count;
        let mut column = Column::new().push(Space::new(Length::Fill, Length::Fixed(2.0)));
        for index in 0..count {
            let is_selected = self.state.polygon.test == index;

            column = column.push(
                Container::new(
                    Button::new(Text::new(format!("test_{}", index)))
                        .width(Length::Fill)
                        .on_press(Message::Polygon(BooleanMessage::TestSelected(index)))
                        .style(if is_selected { style_action_button_selected } else { style_action_button })
                ).padding(self.design.action_padding())
            );
        }

        column
    }

    pub(crate) fn boolean_content(&self) -> Row<Message> {
        Row::new()
            .push(
                scrollable(
                    Container::new(self.sidebar())
                        .width(Length::Fixed(160.0))
                        .height(Length::Shrink)
                        .align_x(Alignment::Start)
                        .padding(Padding::new(0.0).right(8))
                        .style(style_second_background)
                ).direction(scrollable::Direction::Vertical(
                    scrollable::Scrollbar::new()
                        .width(4)
                        .margin(0)
                        .scroller_width(4)
                        .anchor(scrollable::Anchor::Start),
                ))
            )
            .push(self.boolean_workspace())
    }

    pub(crate) fn boolean_update(&mut self, message: BooleanMessage) {
        match message {
            BooleanMessage::TestSelected(index) => self.state.polygon.test = index,
            BooleanMessage::SolverSelected(solver) => self.state.polygon.solver = solver,
            BooleanMessage::FillSelected(fill) => self.state.polygon.fill = fill,
            BooleanMessage::ModeSelected(mode) => self.state.polygon.mode = mode,
        }
    }
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

impl Default for PolygonEditorState {
    fn default() -> Self {
        PolygonEditorState {}
    }
}