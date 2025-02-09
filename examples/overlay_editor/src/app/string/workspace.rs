use crate::draw::path::PathWidget;
use crate::draw::shape::ShapeWidget;
use crate::geom::camera::Camera;
use crate::sheet::widget::SheetWidget;
use crate::point_editor::point::EditorPoint;
use crate::point_editor::widget::{PointEditUpdate, PointsEditorWidget};
use crate::app::string::content::StringMessage;
use crate::app::design::{style_sheet_background, Design};
use crate::app::main::{EditorApp, AppMessage};
use i_triangle::i_overlay::i_shape::int::count::IntShapes;
use i_triangle::i_overlay::i_shape::int::path::IntPaths;
use iced::widget::Stack;
use iced::widget::Container;
use iced::{Length, Padding, Size, Vector};
use crate::app::string::control::ModeOption;
use crate::draw::varicolored::VaricoloredWidget;

pub(crate) enum Solution {
    Shapes(IntShapes),
    Paths(IntPaths),
    None,
}


pub(crate) struct WorkspaceState {
    pub(crate) camera: Camera,
    pub(crate) body: IntPaths,
    pub(crate) string: IntPaths,
    pub(crate) solution: Solution,
    pub(crate) points: Vec<EditorPoint>,
}

impl EditorApp {
    pub(crate) fn string_workspace(&self) -> Container<AppMessage> {
        Container::new({
            let mut stack = Stack::new();
            stack = stack.push(
                Container::new(SheetWidget::new(
                    self.state.string.workspace.camera,
                    Design::negative_color().scale_alpha(0.5),
                    on_update_size,
                    on_update_zoom,
                    on_update_drag,
                ))
                    .width(Length::Fill)
                    .height(Length::Fill)
            );

            if self.state.string.workspace.camera.is_not_empty() {
                match self.state.string.mode {
                    ModeOption::Slice => {
                        if let Solution::Shapes(shapes) = &self.state.string.workspace.solution {
                            stack = stack.push(
                                Container::new(VaricoloredWidget::with_shapes(
                                    shapes,
                                    self.state.string.workspace.camera,
                                    None,
                                    4.0,
                                ))
                                    .width(Length::Fill)
                                    .height(Length::Fill)
                            );
                        }
                        stack = stack.push(
                            Container::new(PathWidget::with_paths(
                                &self.state.string.workspace.string,
                                self.state.string.workspace.camera,
                                Design::negative_color(),
                                4.0,
                                true,
                            ))
                                .width(Length::Fill)
                                .height(Length::Fill)
                        )
                    }
                    ModeOption::ClipDirect | ModeOption::ClipInvert => {
                        stack = stack.push(
                            Container::new(ShapeWidget::with_paths(
                                &self.state.string.workspace.body,
                                self.state.string.workspace.camera,
                                Some(self.state.string.fill.fill_rule()),
                                Some(Design::clip_color().scale_alpha(0.3)),
                                Some(Design::clip_color()),
                                4.0,
                            ))
                                .width(Length::Fill)
                                .height(Length::Fill)
                        ).push(
                            Container::new(PathWidget::with_paths(
                                &self.state.string.workspace.string,
                                self.state.string.workspace.camera,
                                Design::negative_color(),
                                4.0,
                                true,
                            ))
                                .width(Length::Fill)
                                .height(Length::Fill)
                        );
                        if let Solution::Paths(paths) = &self.state.string.workspace.solution {
                            stack = stack.push(
                                Container::new(PathWidget::with_paths(
                                    paths,
                                    self.state.string.workspace.camera,
                                    Design::subject_color(),
                                    4.0,
                                    true,
                                ))
                                    .width(Length::Fill)
                                    .height(Length::Fill)
                            );
                        }
                    }
                    _ => {
                        stack = stack.push(
                            Container::new(ShapeWidget::with_paths(
                                &self.state.string.workspace.body,
                                self.state.string.workspace.camera,
                                Some(self.state.string.fill.fill_rule()),
                                Some(Design::subject_color().scale_alpha(0.2)),
                                Some(Design::subject_color()),
                                4.0,
                            ))
                                .width(Length::Fill)
                                .height(Length::Fill)
                        ).push(
                            Container::new(PathWidget::with_paths(
                                &self.state.string.workspace.string,
                                self.state.string.workspace.camera,
                                Design::negative_color(),
                                4.0,
                                true,
                            ))
                                .width(Length::Fill)
                                .height(Length::Fill)
                        )
                    }
                }
                stack = stack.push(
                    Container::new(PointsEditorWidget::new(
                        &self.state.string.workspace.points,
                        self.state.string.workspace.camera,
                        on_update_point)
                        .set_drag_color(Design::accent_color())
                        .set_hover_color(Design::negative_color())
                    )
                        .width(Length::Fill)
                        .height(Length::Fill)
                );
            }

            stack.push(
                Container::new(self.string_control())
                    .width(Length::Shrink)
                    .height(Length::Shrink)
                    .padding(Padding::new(8.0))
            )
        })
            .style(style_sheet_background)
    }

    pub(super) fn string_update_point(&mut self, update: PointEditUpdate) {
        self.state.string.string_update_point(update);
    }

    pub(super) fn string_update_zoom(&mut self, camera: Camera) {
        self.state.string.workspace.camera = camera;
    }

    pub(super) fn string_update_drag(&mut self, new_pos: Vector<f32>) {
        self.state.string.workspace.camera.pos = new_pos;
    }
}

fn on_update_point(event: PointEditUpdate) -> AppMessage {
    AppMessage::String(StringMessage::PointEdited(event))
}

fn on_update_size(size: Size) -> AppMessage {
    AppMessage::String(StringMessage::WorkspaceSized(size))
}

fn on_update_zoom(zoom: Camera) -> AppMessage {
    AppMessage::String(StringMessage::WorkspaceZoomed(zoom))
}

fn on_update_drag(drag: Vector<f32>) -> AppMessage {
    AppMessage::String(StringMessage::WorkspaceDragged(drag))
}

impl Default for WorkspaceState {
    fn default() -> Self {
        WorkspaceState { camera: Camera::empty(), body: vec![], string: vec![], solution: Solution::None, points: vec![] }
    }
}