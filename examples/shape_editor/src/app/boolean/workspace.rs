use crate::point_editor::point::EditorPoint;
use i_triangle::i_overlay::i_shape::int::count::IntShapes;
use i_triangle::i_overlay::i_shape::int::path::IntPaths;
use iced::widget::Stack;
use iced::widget::Container;
use crate::point_editor::widget::{PointEditUpdate, PointsEditorWidget};
use iced::{Length, Padding};
use crate::app::boolean::content::BooleanMessage;
use crate::app::boolean::control::FillOption;
use crate::app::design::style_sheet_background;
use crate::app::main::{EditorApp, AppMessage};

pub(crate) struct WoerkspaceState {
    pub(crate) subj: IntPaths,
    pub(crate) clip: IntPaths,
    pub(crate) solution: IntShapes,
    pub(crate) points: Vec<EditorPoint>,
}

impl EditorApp {
    pub(crate) fn boolean_workspace(&self) -> Container<AppMessage> {
        Container::new(
            Stack::new()
                .push(
                    Container::new(PointsEditorWidget::new(&self.state.boolean.workspace.points, on_update_point))
                        .width(Length::Fill)
                        .height(Length::Fill)
                )
                .push(
                    Container::new(self.boolean_control())
                        .width(Length::Shrink)
                        .height(Length::Shrink)
                        .padding(Padding::new(8.0))
                )
        )
            .style(style_sheet_background)
    }
}

fn on_update_point(event: PointEditUpdate) -> AppMessage {
    AppMessage::Bool(BooleanMessage::PointEdited(event))
}


impl Default for WoerkspaceState {
    fn default() -> Self {
        WoerkspaceState { subj: vec![], clip: vec![], solution: vec![], points: vec![] }
    }
}