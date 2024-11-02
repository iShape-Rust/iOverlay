use i_triangle::i_overlay::i_shape::int::count::PointsCount;
use i_triangle::i_overlay::i_shape::int::path::IntPaths;
use crate::app::boolean::workspace::WoerkspaceState;
use crate::app::design::style_second_background;
use iced::widget::scrollable;
use crate::app::boolean::control::ModeOption;
use crate::app::boolean::control::FillOption;
use crate::app::boolean::control::SolverOption;
use iced::{Alignment, Length, Padding};
use iced::widget::{Button, Column, Container, Row, Space, Text};
use crate::app::design::{style_action_button, style_action_button_selected};
use crate::app::main::{EditorApp, Message};
use crate::data::polygon::BooleanResource;
use crate::util::point::PathsToEditorPoints;

pub(crate) struct BooleanState {
    pub(crate) test: usize,
    pub(crate) fill: FillOption,
    pub(crate) mode: ModeOption,
    pub(crate) solver: SolverOption,
    pub(crate) workspace: WoerkspaceState,
}

#[derive(Debug, Clone)]
pub(crate) enum BooleanMessage {
    TestSelected(usize),
    FillSelected(FillOption),
    ModeSelected(ModeOption),
    SolverSelected(SolverOption),
}

impl EditorApp {
    fn sidebar(&self) -> Column<Message> {
        let count = self.app_resource.boolean.count;
        let mut column = Column::new().push(Space::new(Length::Fill, Length::Fixed(2.0)));
        for index in 0..count {
            let is_selected = self.state.boolean.test == index;

            column = column.push(
                Container::new(
                    Button::new(Text::new(format!("test_{}", index)))
                        .width(Length::Fill)
                        .on_press(Message::Bool(BooleanMessage::TestSelected(index)))
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

    pub(crate) fn update_boolean(&mut self, message: BooleanMessage) {
        match message {
            BooleanMessage::TestSelected(index) => self.set_test(index),
            BooleanMessage::SolverSelected(solver) => self.state.boolean.solver = solver,
            BooleanMessage::FillSelected(fill) => self.state.boolean.fill = fill,
            BooleanMessage::ModeSelected(mode) => self.state.boolean.mode = mode,
        }
    }

    fn set_test(&mut self, index: usize) {
        self.state.boolean.set_test(index, &mut self.app_resource.boolean);
    }
}

impl BooleanState {
    pub(crate) fn new(resource: &mut BooleanResource) -> Self {
        let mut state = BooleanState {
            test: usize::MAX,
            fill: FillOption::NonZero,
            mode: ModeOption::Edit,
            solver: SolverOption::Auto,
            workspace: Default::default(),
        };

        state.set_test(0, resource);
        state
    }

    fn set_test(&mut self, index: usize, resource: &mut BooleanResource) {
        if let Some(test) = resource.load(index) {
            let editor_points = &mut self.workspace.stateless.editor_points;
            if editor_points.is_empty() {
                editor_points.reserve(test.clip_paths.points_count() + test.subj_paths.points_count())
            } else {
                editor_points.clear();
            }

            test.subj_paths.feed_edit_points(0, editor_points);
            test.clip_paths.feed_edit_points(1, editor_points);

            self.test = index;
        }
    }
}