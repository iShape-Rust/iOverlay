use iced::alignment::Vertical;
use iced::widget::scrollable;
use crate::app::main::MainMessage::ActionSelected;
use std::default::Default;
use crate::app::polygon::PolygonMessage;
use crate::app::polygon::PolygonState;
use crate::data::resource::AppReosurce;
use iced::{Alignment, Color, Element, Length, Padding};
use iced::widget::{Button, button, Column, Container, Row, Text};
use crate::app::design::{action_button, action_button_selected, Design};
use crate::fill_view::FillView;

pub(crate) struct EditorApp {
    main_actions: Vec<MainAction>,
    pub(super) state: MainState,
    pub(super) appReosurce: AppReosurce,
    pub(super) design: Design,
}

pub(super) struct MainState {
    selected_action: MainAction,
    pub(super) polygon: PolygonState,
}

#[derive(Debug, Clone, PartialEq)]
enum MainAction {
    Polygons,
    String,
}

impl MainAction {
    fn title(&self) -> &str {
        match self {
            MainAction::Polygons => "Polygons",
            MainAction::String => "String"
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum MainMessage {
    ActionSelected(MainAction),
}

#[derive(Debug, Clone)]
pub(crate) enum Message {
    Main(MainMessage),
    Polygon(PolygonMessage),
}

impl EditorApp {
    fn new() -> Self {
        Self {
            main_actions: vec![MainAction::Polygons, MainAction::String],
            state: MainState {
                selected_action: MainAction::Polygons,
                polygon: Default::default(),
            },
            appReosurce: AppReosurce::new(),
            design: Design::new(),
        }
    }

    pub(crate) fn update(&mut self, message: Message) {
        match message {
            Message::Main(msg) => self.update_main(msg),
            Message::Polygon(msg) => self.update_polygon(msg)
        }
    }

    fn update_main(&mut self, message: MainMessage) {
        match message {
            MainMessage::ActionSelected(action) => self.state.selected_action = action
        }
    }

    pub(crate) fn view(&self) -> Element<Message> {
        let main_items: Column<Message> = self.main_actions.iter().fold(
            Column::new(),
            |column, item| {
                let is_selected = self.state.selected_action.eq(item);
                column.push(
                    Container::new(
                        Button::new(Text::new(item.title()))
                            .width(Length::Fill)
                            .on_press(Message::Main(ActionSelected(item.clone())))
                            .style(if is_selected { action_button_selected } else { action_button })
                    ).padding(self.design.action_padding())
                )
            },
        );

        let top_action_padding = Padding { top: self.design.action_separator, right: 0.0, bottom: 0.0, left: 0.0 };

        let content = Row::new()
            .push(Container::new(main_items)
                .width(Length::Fixed(160.0))
                .height(Length::Shrink)
                .align_x(Alignment::Start))
            .padding(top_action_padding);

        let content = match self.state.selected_action {
            MainAction::Polygons => {
                content
                    .push(
                        scrollable(
                            Container::new(self.polygon_tests_list())
                                .width(Length::Fixed(160.0))
                                .height(Length::Shrink)
                                .align_x(Alignment::Start)
                                .padding(Padding::new(0.0).right(8))
                        ).direction(scrollable::Direction::Vertical(
                            scrollable::Scrollbar::new()
                                .width(8)
                                .margin(0)
                                .scroller_width(8)
                                .anchor(scrollable::Anchor::Start),
                        ))
                    ).padding(top_action_padding)
                    .push(FillView::new(Color {
                        r: 0.0,
                        g: 1.0,
                        b: 1.0,
                        a: 1.0,
                    }))
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
}

impl Default for EditorApp {
    fn default() -> Self {
        Self::new()
    }
}