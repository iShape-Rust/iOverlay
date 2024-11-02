use crate::app::boolean::editor::data::StatelessData;
use i_triangle::i_overlay::i_shape::int::count::IntShapes;
use i_triangle::i_overlay::i_shape::int::path::IntPaths;
use iced::widget::Stack;
use iced::widget::Container;
use crate::app::boolean::editor::widget::SubjClipEditorWidget;
use iced::{Length, Padding};
use crate::app::design::style_sheet_background;
use crate::app::main::{EditorApp, Message};

pub(crate) struct WoerkspaceState {
    pub(crate) subj: IntPaths,
    pub(crate) clip: IntPaths,
    pub(crate) solution: IntShapes,
    pub(crate) stateless: StatelessData,
}

impl EditorApp {
    pub(crate) fn boolean_workspace(&self) -> Container<Message> {
        Container::new(
            Stack::new()
                .push(
                    Container::new(SubjClipEditorWidget::new(&self.state.boolean.workspace.stateless))
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


impl Default for WoerkspaceState {
    fn default() -> Self {
        WoerkspaceState { subj: vec![], clip: vec![], solution: vec![], stateless: StatelessData::default() }
    }
}