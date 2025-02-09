use crate::draw::path::PathWidget;
use crate::draw::shape::ShapeWidget;
use crate::geom::camera::Camera;
use crate::sheet::widget::SheetWidget;
use crate::point_editor::point::EditorPoint;
use crate::point_editor::widget::{PointEditUpdate, PointsEditorWidget};
use crate::app::design::{style_sheet_background, Design};
use crate::app::main::{EditorApp, AppMessage};
use crate::app::stroke::content::StrokeMessage;
use i_triangle::i_overlay::i_shape::int::path::IntPaths;
use i_triangle::i_overlay::i_shape::int::shape::IntShapes;
use iced::widget::Stack;
use iced::widget::Container;
use iced::{Length, Padding, Size, Vector};
use crate::draw::varicolored::VaricoloredWidget;


pub(crate) struct WorkspaceState {
    pub(crate) camera: Camera,
    pub(crate) scale: f32,
    pub(crate) stroke_input: IntPaths,
    pub(crate) stroke_output: IntPaths,
    pub(crate) points: Vec<EditorPoint>,
}

impl EditorApp {
    pub(crate) fn stroke_workspace(&self) -> Container<AppMessage> {
        Container::new({
            let mut stack = Stack::new();
            stack = stack.push(
                Container::new(SheetWidget::new(
                    self.state.stroke.workspace.camera,
                    Design::negative_color().scale_alpha(0.5),
                    on_update_size,
                    on_update_zoom,
                    on_update_drag,
                ))
                    .width(Length::Fill)
                    .height(Length::Fill)
            );

            if self.state.stroke.workspace.camera.is_not_empty() {
                let shapes = &self.state.stroke.workspace.stroke_output;
                if !shapes.is_empty() {
                    stack = stack.push(
                        Container::new(PathWidget::with_paths(
                            &self.state.stroke.workspace.stroke_output,
                            self.state.stroke.workspace.camera,
                            Design::solution_color(),
                            4.0,
                            true,
                        ))
                            .width(Length::Fill)
                            .height(Length::Fill)
                    );
                }
                stack = stack.push(
                    Container::new(PathWidget::with_paths(
                        &self.state.stroke.workspace.stroke_input,
                        self.state.stroke.workspace.camera,
                        Design::subject_color(),
                        4.0,
                        true,
                    ))
                        .width(Length::Fill)
                        .height(Length::Fill)
                );
                stack = stack.push(
                    Container::new(PointsEditorWidget::new(
                        &self.state.stroke.workspace.points,
                        self.state.stroke.workspace.camera,
                        on_update_point)
                        .set_drag_color(Design::accent_color())
                        .set_hover_color(Design::negative_color())
                    )
                        .width(Length::Fill)
                        .height(Length::Fill)
                );
            }

            stack.push(
                Container::new(self.stroke_control())
                    .width(Length::Shrink)
                    .height(Length::Shrink)
                    .padding(Padding::new(8.0))
            )
        })
            .style(style_sheet_background)
    }

    pub(super) fn stroke_update_point(&mut self, update: PointEditUpdate) {
        self.state.stroke.stroke_update_point(update);
    }

    pub(super) fn stroke_update_zoom(&mut self, camera: Camera) {
        self.state.stroke.workspace.camera = camera;
    }

    pub(super) fn stroke_update_drag(&mut self, new_pos: Vector<f32>) {
        self.state.stroke.workspace.camera.pos = new_pos;
    }
}

fn on_update_point(event: PointEditUpdate) -> AppMessage {
    AppMessage::Stroke(StrokeMessage::PointEdited(event))
}

fn on_update_size(size: Size) -> AppMessage {
    AppMessage::Stroke(StrokeMessage::WorkspaceSized(size))
}

fn on_update_zoom(zoom: Camera) -> AppMessage {
    AppMessage::Stroke(StrokeMessage::WorkspaceZoomed(zoom))
}

fn on_update_drag(drag: Vector<f32>) -> AppMessage {
    AppMessage::Stroke(StrokeMessage::WorkspaceDragged(drag))
}

impl Default for WorkspaceState {
    fn default() -> Self {
        WorkspaceState { scale: 1.0, camera: Camera::empty(), stroke_input: vec![], stroke_output: vec![], points: vec![] }
    }
}