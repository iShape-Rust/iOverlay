use std::default::Default;
use iced::Subscription;
use iced::keyboard::Key::Named as NamedBox;
use iced::event::{self, Event as MainEvent};
use iced::widget::{Space, vertical_rule};
use iced::{Alignment, Element, Length};
use iced::keyboard::Event as KeyboardEvent;
use iced::keyboard::key::Named;
use iced::widget::{Button, Column, Container, Row, Text};
use crate::app::string::content::StringMessage;
use crate::app::string::content::StringState;
use crate::app::boolean::content::BooleanMessage;
use crate::app::boolean::content::BooleanState;
use crate::app::design::style_separator;
use crate::app::design::{style_sidebar_button, style_sidebar_button_selected, Design};
use crate::data::resource::AppResource;

pub(crate) struct EditorApp {
    main_actions: Vec<MainAction>,
    pub(super) state: MainState,
    pub(super) app_resource: AppResource,
    pub(super) design: Design,
}

pub(super) struct MainState {
    selected_action: MainAction,
    pub(super) boolean: BooleanState,
    pub(super) string: StringState,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum MainAction {
    Boolean,
    String,
}

impl MainAction {
    fn title(&self) -> &str {
        match self {
            MainAction::Boolean => "Boolean",
            MainAction::String => "String"
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
    EventOccurred(MainEvent),
}

impl EditorApp {
    fn new() -> Self {
        let mut app_resource = AppResource::new();
        Self {
            main_actions: vec![MainAction::Boolean, MainAction::String],
            state: MainState {
                selected_action: MainAction::Boolean,
                boolean: BooleanState::new(&mut app_resource.boolean),
                string: StringState::new(&mut app_resource.string),
            },
            app_resource,
            design: Design::new(),
        }
    }

    pub(crate) fn update(&mut self, message: AppMessage) {
        match message {
            AppMessage::Main(msg) => self.update_main(msg),
            AppMessage::Bool(msg) => self.boolean_update(msg),
            AppMessage::String(msg) => self.string_update(msg),
            AppMessage::EventOccurred(event) => {
                if let MainEvent::Keyboard(keyboard) = event {
                     if let KeyboardEvent::KeyPressed{
                         key,
                         modified_key: _,
                         physical_key: _,
                         location: _,
                         modifiers: _,
                         text: _,
                     } = keyboard  {
                         if let NamedBox(named) = key {
                            match named {
                                Named::ArrowDown => {
                                    match self.state.selected_action {
                                        MainAction::Boolean => self.boolean_next_test(),
                                        MainAction::String => self.string_next_test(),
                                    }
                                },
                                Named::ArrowUp => {
                                    match self.state.selected_action {
                                        MainAction::Boolean => self.boolean_prev_test(),
                                        MainAction::String => self.string_prev_test(),
                                    }
                                },
                                _ => {}
                            }
                         }
                     }
                }
            }
        }
    }

    pub(crate) fn subscription(&self) -> Subscription<AppMessage> {
        event::listen().map(AppMessage::EventOccurred)
    }

    fn update_main(&mut self, message: MainMessage) {
        match message {
            MainMessage::ActionSelected(action) => {
                self.state.selected_action = action;
                match self.state.selected_action {
                    MainAction::Boolean => self.boolean_init(),
                    MainAction::String => self.string_init(),
                }
            }
        }
    }

    pub(crate) fn view(&self) -> Element<AppMessage> {
        let content = Row::new()
            .push(Container::new(self.main_navigation())
                .width(Length::Fixed(160.0))
                .height(Length::Shrink)
                .align_x(Alignment::Start));

        let content = match self.state.selected_action {
            MainAction::Boolean => {
                content
                    .push(
                        vertical_rule(1).style(style_separator)
                    )
                    .push(self.boolean_content())
            }
            MainAction::String => {
                content
                    .push(
                        vertical_rule(1).style(style_separator)
                    )
                    .push(self.string_content())
            }
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
                        Button::new(Text::new(item.title()))
                            .width(Length::Fill)
                            .on_press(AppMessage::Main(MainMessage::ActionSelected(item.clone())))
                            .style(if is_selected { style_sidebar_button_selected } else { style_sidebar_button })
                    ).padding(self.design.action_padding())
                )
            },
        )
    }

}

impl Default for EditorApp {
    fn default() -> Self {
        Self::new()
    }
}