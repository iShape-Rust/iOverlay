use crate::app::design::{style_sheet_background, Design};
use crate::app::main::{AppMessage, EditorApp};
use crate::app::variable_stroke::content::{VariableStrokeMessage, VariableStrokePoint};
use crate::draw::path::PathWidget;
use crate::draw::shape::ShapeWidget;
use crate::geom::camera::Camera;
use crate::point_editor::point::EditorPoint;
use crate::point_editor::widget::{PointEditUpdate, PointsEditorWidget};
use crate::sheet::widget::SheetWidget;
use i_triangle::i_overlay::core::fill_rule::FillRule;
use i_triangle::i_overlay::i_shape::int::path::IntPaths;
use iced::widget::Container;
use iced::widget::Stack;
use iced::{Length, Padding, Size, Vector};

pub(crate) struct WorkspaceState {
    pub(crate) camera: Camera,
    pub(crate) scale: f32,
    pub(crate) variable_input: Vec<Vec<VariableStrokePoint>>,
    pub(crate) centerline_input: IntPaths<i32>,
    pub(crate) stroke_output: IntPaths<i32>,
    pub(crate) points: Vec<EditorPoint>,
}

impl EditorApp {
    pub(crate) fn variable_stroke_workspace(&self) -> Container<'_, AppMessage> {
        Container::new({
            let mut stack = Stack::new();
            stack = stack.push(
                Container::new(SheetWidget::new(
                    self.state.variable_stroke.workspace.camera,
                    Design::negative_color().scale_alpha(0.5),
                    on_update_size,
                    on_update_zoom,
                    on_update_drag,
                ))
                .width(Length::Fill)
                .height(Length::Fill),
            );

            if self.state.variable_stroke.workspace.camera.is_not_empty() {
                let shapes = &self.state.variable_stroke.workspace.stroke_output;
                if !shapes.is_empty() {
                    stack = stack.push(
                        Container::new(ShapeWidget::with_paths(
                            &self.state.variable_stroke.workspace.stroke_output,
                            self.state.variable_stroke.workspace.camera,
                            Some(FillRule::NonZero),
                            Some(Design::solution_color().scale_alpha(0.1)),
                            Some(Design::solution_color()),
                            2.0,
                        ))
                        .width(Length::Fill)
                        .height(Length::Fill),
                    );
                }
                stack = stack.push(
                    Container::new(PathWidget::with_paths(
                        &self.state.variable_stroke.workspace.centerline_input,
                        self.state.variable_stroke.workspace.camera,
                        Design::subject_color(),
                        1.0,
                        false,
                    ))
                    .width(Length::Fill)
                    .height(Length::Fill),
                );
                stack = stack.push(
                    Container::new(
                        PointsEditorWidget::new(
                            &self.state.variable_stroke.workspace.points,
                            self.state.variable_stroke.workspace.camera,
                            on_update_point,
                        )
                        .set_drag_color(Design::accent_color())
                        .set_hover_color(Design::negative_color()),
                    )
                    .width(Length::Fill)
                    .height(Length::Fill),
                );
            }

            stack.push(
                Container::new(self.variable_stroke_control())
                    .width(Length::Shrink)
                    .height(Length::Shrink)
                    .padding(Padding::new(8.0)),
            )
        })
        .style(style_sheet_background)
    }

    pub(super) fn variable_stroke_update_point(&mut self, update: PointEditUpdate) {
        self.state
            .variable_stroke
            .variable_stroke_update_point(update);
    }

    pub(super) fn variable_stroke_update_zoom(&mut self, camera: Camera) {
        self.state.variable_stroke.workspace.camera = camera;
    }

    pub(super) fn variable_stroke_update_drag(&mut self, new_pos: Vector<f32>) {
        self.state.variable_stroke.workspace.camera.pos = new_pos;
    }
}

fn on_update_point(event: PointEditUpdate) -> AppMessage {
    AppMessage::VariableStroke(VariableStrokeMessage::PointEdited(event))
}

fn on_update_size(size: Size) -> AppMessage {
    AppMessage::VariableStroke(VariableStrokeMessage::WorkspaceSized(size))
}

fn on_update_zoom(zoom: Camera) -> AppMessage {
    AppMessage::VariableStroke(VariableStrokeMessage::WorkspaceZoomed(zoom))
}

fn on_update_drag(drag: Vector<f32>) -> AppMessage {
    AppMessage::VariableStroke(VariableStrokeMessage::WorkspaceDragged(drag))
}

impl Default for WorkspaceState {
    fn default() -> Self {
        WorkspaceState {
            scale: 1.0,
            camera: Camera::empty(),
            variable_input: vec![],
            centerline_input: vec![],
            stroke_output: vec![],
            points: vec![],
        }
    }
}
