use i_triangle::i_overlay::core::fill_rule::FillRule;
use crate::draw::shape::ShapeWidget;
use crate::geom::camera::Camera;
use crate::sheet::widget::SheetWidget;
use crate::point_editor::point::EditorPoint;
use crate::point_editor::widget::{PointEditUpdate, PointsEditorWidget};
use crate::app::design::{style_sheet_background, Design};
use crate::app::main::{EditorApp, AppMessage};
use crate::app::outline::content::OutlineMessage;
use i_triangle::i_overlay::i_shape::int::path::IntPaths;
use iced::widget::Stack;
use iced::widget::Container;
use iced::{Length, Padding, Size, Vector};


pub(crate) struct WorkspaceState {
    pub(crate) camera: Camera,
    pub(crate) scale: f32,
    pub(crate) outline_input: IntPaths,
    pub(crate) outline_output: IntPaths,
    pub(crate) points: Vec<EditorPoint>,
}

impl EditorApp {
    pub(crate) fn outline_workspace(&self) -> Container<'_, AppMessage> {
        Container::new({
            let mut stack = Stack::new();
            stack = stack.push(
                Container::new(SheetWidget::new(
                    self.state.outline.workspace.camera,
                    Design::negative_color().scale_alpha(0.5),
                    on_update_size,
                    on_update_zoom,
                    on_update_drag,
                ))
                    .width(Length::Fill)
                    .height(Length::Fill)
            );

            if self.state.outline.workspace.camera.is_not_empty() {
                stack = stack.push(
                    Container::new(ShapeWidget::with_paths(
                        &self.state.outline.workspace.outline_output,
                        self.state.outline.workspace.camera,
                        Some(FillRule::NonZero),
                        Some(Design::solution_color().scale_alpha(0.1)),
                        Some(Design::solution_color()),
                        2.0,
                    ))
                        .width(Length::Fill)
                        .height(Length::Fill)
                );
                stack = stack.push(
                    Container::new(ShapeWidget::with_paths(
                        &self.state.outline.workspace.outline_input,
                        self.state.outline.workspace.camera,
                        Some(FillRule::NonZero),
                        Some(Design::subject_color().scale_alpha(0.1)),
                        Some(Design::subject_color()),
                        1.0,
                    ))
                        .width(Length::Fill)
                        .height(Length::Fill)
                );
                stack = stack.push(
                    Container::new(PointsEditorWidget::new(
                        &self.state.outline.workspace.points,
                        self.state.outline.workspace.camera,
                        on_update_point)
                        .set_drag_color(Design::accent_color())
                        .set_hover_color(Design::negative_color())
                    )
                        .width(Length::Fill)
                        .height(Length::Fill)
                );
            }

            stack.push(
                Container::new(self.outline_control())
                    .width(Length::Shrink)
                    .height(Length::Shrink)
                    .padding(Padding::new(8.0))
            )
        })
            .style(style_sheet_background)
    }

    pub(super) fn outline_update_point(&mut self, update: PointEditUpdate) {
        self.state.outline.outline_update_point(update);
    }

    pub(super) fn outline_update_zoom(&mut self, camera: Camera) {
        self.state.outline.workspace.camera = camera;
    }

    pub(super) fn outline_update_drag(&mut self, new_pos: Vector<f32>) {
        self.state.outline.workspace.camera.pos = new_pos;
    }
}

fn on_update_point(event: PointEditUpdate) -> AppMessage {
    AppMessage::Outline(OutlineMessage::PointEdited(event))
}

fn on_update_size(size: Size) -> AppMessage {
    AppMessage::Outline(OutlineMessage::WorkspaceSized(size))
}

fn on_update_zoom(zoom: Camera) -> AppMessage {
    AppMessage::Outline(OutlineMessage::WorkspaceZoomed(zoom))
}

fn on_update_drag(drag: Vector<f32>) -> AppMessage {
    AppMessage::Outline(OutlineMessage::WorkspaceDragged(drag))
}

impl Default for WorkspaceState {
    fn default() -> Self {
        WorkspaceState { scale: 1.0, camera: Camera::empty(), outline_input: vec![], outline_output: vec![], points: vec![] }
    }
}