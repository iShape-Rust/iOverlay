use crate::app::design::Design;
use crate::app::main::{AppMessage, EditorApp};
use crate::app::string::content::StringMessage;
use crate::app::string::control::ModeOption;
use crate::draw::path::PathWidget;
use crate::draw::shape::ShapeWidget;
use crate::draw::varicolored::VaricoloredWidget;
use crate::geom::camera::Camera;
use crate::point_editor::point::EditorPoint;
use crate::point_editor::{state::PointsEditorState, widget::PointsEditorWidget};
use crate::sheet::state::SheetState;
use crate::sheet::widget::SheetWidget;
use eframe::egui;
use i_triangle::i_overlay::i_shape::int::count::IntShapes as RawIntShapes;
use i_triangle::i_overlay::i_shape::int::path::IntPaths as RawIntPaths;

type IntPaths = RawIntPaths<i32>;
type IntShapes = RawIntShapes<i32>;

pub(crate) enum Solution {
    Shapes(IntShapes),
    Paths(IntPaths),
    None,
}

pub(crate) struct WorkspaceState {
    pub(crate) camera: Camera,
    pub(crate) sheet_state: SheetState,
    pub(crate) point_state: PointsEditorState,
    pub(crate) body: IntPaths,
    pub(crate) string: IntPaths,
    pub(crate) solution: Solution,
    pub(crate) points: Vec<EditorPoint>,
}

impl EditorApp {
    pub(crate) fn string_workspace(&mut self, ui: &mut egui::Ui) {
        self.update(AppMessage::String(StringMessage::WorkspaceSized(
            ui.available_size(),
        )));
        let workspace = &mut self.state.string.workspace;
        let (painter, update) = SheetWidget::show(
            ui,
            &mut workspace.camera,
            &workspace.points,
            &mut workspace.sheet_state,
            &mut workspace.point_state,
        );
        if let Some(update) = update {
            self.update(AppMessage::String(StringMessage::PointEdited(update)));
            ui.ctx().request_repaint();
        }
        let workspace = &self.state.string.workspace;
        if workspace.camera.is_not_empty() {
            match self.state.string.mode {
                ModeOption::Slice => {
                    if let Solution::Shapes(shapes) = &workspace.solution {
                        VaricoloredWidget::with_shapes(shapes, workspace.camera, 2.0)
                            .paint(&painter);
                    }
                    PathWidget::with_paths(
                        &workspace.string,
                        workspace.camera,
                        Design::negative_color(),
                        2.0,
                        true,
                    )
                    .paint(&painter);
                }
                ModeOption::ClipDirect | ModeOption::ClipInvert => {
                    ShapeWidget::with_paths(
                        &workspace.body,
                        workspace.camera,
                        Some(self.state.string.fill.fill_rule()),
                        Some(Design::clip_color().gamma_multiply(0.3)),
                        Some(Design::clip_color()),
                        2.0,
                    )
                    .paint(&painter);
                    PathWidget::with_paths(
                        &workspace.string,
                        workspace.camera,
                        Design::negative_color(),
                        2.0,
                        true,
                    )
                    .paint(&painter);
                    if let Solution::Paths(paths) = &workspace.solution {
                        PathWidget::with_paths(
                            paths,
                            workspace.camera,
                            Design::subject_color(),
                            2.0,
                            true,
                        )
                        .paint(&painter);
                    }
                }
                _ => {
                    ShapeWidget::with_paths(
                        &workspace.body,
                        workspace.camera,
                        Some(self.state.string.fill.fill_rule()),
                        Some(Design::subject_color().gamma_multiply(0.2)),
                        Some(Design::subject_color()),
                        2.0,
                    )
                    .paint(&painter);
                    PathWidget::with_paths(
                        &workspace.string,
                        workspace.camera,
                        Design::negative_color(),
                        2.0,
                        true,
                    )
                    .paint(&painter);
                }
            }
        }
        PointsEditorWidget::paint(
            &painter,
            workspace.camera,
            &workspace.points,
            &workspace.point_state,
        );
    }
    pub(super) fn string_update_point(
        &mut self,
        update: crate::point_editor::widget::PointEditUpdate,
    ) {
        self.state.string.string_update_point(update);
    }
}
impl Default for WorkspaceState {
    fn default() -> Self {
        WorkspaceState {
            camera: Camera::empty(),
            sheet_state: Default::default(),
            point_state: Default::default(),
            body: vec![],
            string: vec![],
            solution: Solution::None,
            points: vec![],
        }
    }
}
