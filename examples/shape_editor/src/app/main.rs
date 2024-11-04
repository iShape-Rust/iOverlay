use crate::app::design::style_separator;
use iced::widget::{Space, vertical_rule};
use std::default::Default;
use crate::app::boolean::content::BooleanMessage;
use crate::app::boolean::content::BooleanState;
use crate::data::resource::AppResource;
use iced::{Alignment, Color, Element, Length};
use iced::widget::{Button, Column, Container, Row, Text};
use crate::app::design::{style_action_button, style_action_button_selected, Design};
use crate::fill_view::FillView;

pub(crate) struct EditorApp {
    main_actions: Vec<MainAction>,
    pub(super) state: MainState,
    pub(super) app_resource: AppResource,
    pub(super) design: Design,
}

pub(super) struct MainState {
    selected_action: MainAction,
    pub(super) boolean: BooleanState,
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
}

impl EditorApp {
    fn new() -> Self {
        let mut app_resource = AppResource::new();
        Self {
            main_actions: vec![MainAction::Boolean, MainAction::String],
            state: MainState {
                selected_action: MainAction::Boolean,
                boolean: BooleanState::new(&mut app_resource.boolean),
            },
            app_resource,
            design: Design::new(),
        }
    }

    pub(crate) fn update(&mut self, message: AppMessage) {
        match message {
            AppMessage::Main(msg) => self.update_main(msg),
            AppMessage::Bool(msg) => self.update_boolean(msg)
        }
    }

    fn update_main(&mut self, message: MainMessage) {
        match message {
            MainMessage::ActionSelected(action) => self.state.selected_action = action
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
                content.push(FillView::new(Color {
                    r: 1.0,
                    g: 1.0,
                    b: 0.0,
                    a: 1.0,
                }))
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
                            .style(if is_selected { style_action_button_selected } else { style_action_button })
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