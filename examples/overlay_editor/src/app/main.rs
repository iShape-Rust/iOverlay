use crate::app::boolean::content::BooleanMessage;
use crate::app::boolean::content::BooleanState;
use crate::app::outline::content::OutlineMessage;
use crate::app::outline::content::OutlineState;
use crate::app::string::content::StringMessage;
use crate::app::string::content::StringState;
use crate::app::stroke::content::StrokeMessage;
use crate::app::stroke::content::StrokeState;
use iced::event::{self, Event as MainEvent};
use iced::keyboard::key::Named;
use iced::keyboard::Event as KeyboardEvent;
use iced::keyboard::Key::Named as NamedBox;
use iced::widget::{vertical_rule, Space};
use iced::widget::{Button, Column, Container, Row, Text};
use iced::{Alignment, Element, Font, Length};
use iced::{Subscription, Task};

use crate::app::design::style_separator;
use crate::app::design::{style_sidebar_button, style_sidebar_button_selected, Design};
use crate::data::resource::AppResource;

pub struct EditorApp {
    main_actions: Vec<MainAction>,
    pub(super) state: MainState,
    pub(super) app_resource: AppResource,
    pub(super) design: Design,
}

pub(super) struct MainState {
    custom_font: Font,
    selected_action: MainAction,
    pub(super) boolean: BooleanState,
    pub(super) string: StringState,
    pub(super) stroke: StrokeState,
    pub(super) outline: OutlineState,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum MainAction {
    Boolean,
    String,
    Stroke,
    Outline,
}

impl MainAction {
    fn title(&self) -> &str {
        match self {
            MainAction::Boolean => "Boolean",
            MainAction::String => "String",
            MainAction::Stroke => "Stroke",
            MainAction::Outline => "Outline",
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum MainMessage {
    ActionSelected(MainAction),
}

#[derive(Debug, Clone)]
pub(crate) enum AppMessage {
    Main(MainMessage),
    Bool(BooleanMessage),
    String(StringMessage),
    Stroke(StrokeMessage),
    Outline(OutlineMessage),
    EventOccurred(MainEvent),
}

impl EditorApp {
    pub fn with_resource(mut app_resource: AppResource, custom_font: Font) -> Self {
        Self {
            main_actions: vec![
                MainAction::Boolean,
                MainAction::String,
                MainAction::Stroke,
                MainAction::Outline,
            ],
            state: MainState {
                custom_font,
                selected_action: MainAction::Boolean,
                boolean: BooleanState::new(&mut app_resource.boolean),
                string: StringState::new(&mut app_resource.string),
                stroke: StrokeState::new(&mut app_resource.stroke),
                outline: OutlineState::new(&mut app_resource.outline),
            },
            app_resource,
            design: Design::new(),
        }
    }
}

impl EditorApp {
    pub fn update(&mut self, message: AppMessage) -> Task<AppMessage> {
        match message {
            AppMessage::Main(msg) => self.update_main(msg),
            AppMessage::Bool(msg) => self.boolean_update(msg),
            AppMessage::String(msg) => self.string_update(msg),
            AppMessage::Stroke(msg) => self.stroke_update(msg),
            AppMessage::Outline(msg) => self.outline_update(msg),
            AppMessage::EventOccurred(MainEvent::Keyboard(KeyboardEvent::KeyPressed {
                key: NamedBox(named @ (Named::ArrowDown | Named::ArrowUp)),
                ..
            })) => match (named, self.state.selected_action.clone()) {
                (Named::ArrowDown, MainAction::Boolean) => self.boolean_next_test(),
                (Named::ArrowDown, MainAction::String) => self.string_next_test(),
                (Named::ArrowDown, MainAction::Stroke) => self.stroke_next_test(),
                (Named::ArrowDown, MainAction::Outline) => self.outline_next_test(),
                (Named::ArrowUp, MainAction::Boolean) => self.boolean_prev_test(),
                (Named::ArrowUp, MainAction::String) => self.string_prev_test(),
                (Named::ArrowUp, MainAction::Stroke) => self.stroke_prev_test(),
                (Named::ArrowUp, MainAction::Outline) => self.outline_prev_test(),
                _ => {}
            },
            _ => {}
        }

        Task::none()
    }

    pub fn subscription(&self) -> Subscription<AppMessage> {
        event::listen().map(AppMessage::EventOccurred)
    }

    fn update_main(&mut self, message: MainMessage) {
        match message {
            MainMessage::ActionSelected(action) => {
                self.state.selected_action = action;
                match self.state.selected_action {
                    MainAction::Boolean => self.boolean_init(),
                    MainAction::String => self.string_init(),
                    MainAction::Stroke => self.stroke_init(),
                    MainAction::Outline => self.outline_init(),
                }
            }
        }
    }

    pub fn view(&self) -> Element<AppMessage> {
        let content = Row::new().push(
            Container::new(self.main_navigation())
                .width(Length::Fixed(160.0))
                .height(Length::Shrink)
                .align_x(Alignment::Start),
        );

        let content = match self.state.selected_action {
            MainAction::Boolean => content
                .push(vertical_rule(1).style(style_separator))
                .push(self.boolean_content()),
            MainAction::String => content
                .push(vertical_rule(1).style(style_separator))
                .push(self.string_content()),
            MainAction::Stroke => content
                .push(vertical_rule(1).style(style_separator))
                .push(self.stroke_content()),
            MainAction::Outline => content
                .push(vertical_rule(1).style(style_separator))
                .push(self.outline_content()),
        };

        content.height(Length::Fill).into()
    }

    fn main_navigation(&self) -> Column<AppMessage> {
        self.main_actions.iter().fold(
            Column::new().push(Space::new(Length::Fill, Length::Fixed(2.0))),
            |column, item| {
                let is_selected = self.state.selected_action.eq(item);
                column.push(
                    Container::new(
                        Button::new(Text::new(item.title()).font(self.state.custom_font))
                            .width(Length::Fill)
                            .on_press(AppMessage::Main(MainMessage::ActionSelected(item.clone())))
                            .style(if is_selected {
                                style_sidebar_button_selected
                            } else {
                                style_sidebar_button
                            }),
                    )
                    .padding(self.design.action_padding()),
                )
            },
        )
    }
}
