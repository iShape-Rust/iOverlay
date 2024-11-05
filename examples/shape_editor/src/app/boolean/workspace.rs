use crate::shape::widget::ShapeWidget;
use crate::geom::camera::Camera;
use crate::sheet::widget::SheetWidget;
use crate::app::boolean::control::ModeOption;
use crate::point_editor::point::EditorPoint;
use crate::point_editor::widget::{PointEditUpdate, PointsEditorWidget};
use crate::app::boolean::content::BooleanMessage;
use crate::app::design::style_sheet_background;
use crate::app::main::{EditorApp, AppMessage};
use i_triangle::i_overlay::core::overlay_rule::OverlayRule;
use i_triangle::i_overlay::i_shape::int::count::IntShapes;
use i_triangle::i_overlay::i_shape::int::path::IntPaths;
use iced::widget::Stack;
use iced::widget::Container;
use iced::{Color, Length, Padding, Size, Vector};

pub(crate) struct WorkspaceState {
    pub(crate) camera: Camera,
    pub(crate) subj: IntPaths,
    pub(crate) clip: IntPaths,
    pub(crate) solution: IntShapes,
    pub(crate) points: Vec<EditorPoint>,
}

impl EditorApp {
    pub(crate) fn boolean_workspace(&self) -> Container<AppMessage> {
        Container::new({
            let mut stack = Stack::new();
            stack = stack.push(
                Container::new(SheetWidget::new(
                    self.state.boolean.workspace.camera,
                    on_update_size,
                    on_update_zoom,
                    on_update_drag,
                ))
                    .width(Length::Fill)
                    .height(Length::Fill)
            );

            if self.state.boolean.workspace.camera.is_not_empty() {
                stack = stack
                    .push(
                        Container::new(ShapeWidget::new(
                            self.state.boolean.workspace.camera,
                            &self.state.boolean.workspace.solution,
                            Some(Color::from_rgb8(45, 214, 0).scale_alpha(0.13)),
                            None, 0.0
                        ))
                            .width(Length::Fill)
                            .height(Length::Fill)
                    )
                    .push(
                        Container::new(PointsEditorWidget::new(
                            &self.state.boolean.workspace.points,
                            self.state.boolean.workspace.camera,
                            on_update_point)
                        )
                            .width(Length::Fill)
                            .height(Length::Fill)
                    );
            }

            stack.push(
                Container::new(self.boolean_control())
                    .width(Length::Shrink)
                    .height(Length::Shrink)
                    .padding(Padding::new(8.0))
            )
        })
            .style(style_sheet_background)
    }

    pub(super) fn update_boolean_point(&mut self, update: PointEditUpdate) {
        self.state.boolean.update_boolean_point(update);
    }

    pub(super) fn update_boolean_zoom(&mut self, scale: f32) {
        self.state.boolean.workspace.camera.scale = scale;
    }

    pub(super) fn update_boolean_drag(&mut self, new_pos: Vector<f32>) {
        self.state.boolean.workspace.camera.pos = new_pos;
    }
}

fn on_update_point(event: PointEditUpdate) -> AppMessage {
    AppMessage::Bool(BooleanMessage::PointEdited(event))
}

fn on_update_size(size: Size) -> AppMessage {
    AppMessage::Bool(BooleanMessage::WorkspaceSized(size))
}

fn on_update_zoom(zoom: f32) -> AppMessage {
    AppMessage::Bool(BooleanMessage::WorkspaceZoomed(zoom))
}

fn on_update_drag(drag: Vector<f32>) -> AppMessage {
    AppMessage::Bool(BooleanMessage::WorkspaceDraged(drag))
}

impl Default for WorkspaceState {
    fn default() -> Self {
        WorkspaceState { camera: Camera::empty(), subj: vec![], clip: vec![], solution: vec![], points: vec![] }
    }
}

impl ModeOption {
    fn overlay_rule(&self) -> OverlayRule {
        match self {
            ModeOption::Subject => OverlayRule::Subject,
            ModeOption::Clip => OverlayRule::Clip,
            ModeOption::Intersect => OverlayRule::Intersect,
            ModeOption::Union => OverlayRule::Union,
            ModeOption::Difference => OverlayRule::Difference,
            ModeOption::InverseDifference => OverlayRule::InverseDifference,
            ModeOption::Xor => OverlayRule::Xor,
            _ => OverlayRule::Subject,
        }
    }
}