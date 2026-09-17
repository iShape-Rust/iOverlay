use crate::app::boolean::content::BooleanMessage;
use crate::app::boolean::control::ModeOption;
use crate::app::design::Design;
use crate::app::main::{AppMessage, EditorApp};
use crate::draw::shape::ShapeWidget;
use crate::draw::vectors::VectorsWidget;
use crate::geom::camera::Camera;
use crate::point_editor::point::EditorPoint;
use crate::point_editor::{state::PointsEditorState, widget::PointsEditorWidget};
use crate::sheet::state::SheetState;
use crate::sheet::widget::SheetWidget;
use eframe::egui;
use i_triangle::i_overlay::i_shape::int::count::IntShapes as RawIntShapes;
use i_triangle::i_overlay::i_shape::int::path::IntPaths as RawIntPaths;
use i_triangle::i_overlay::vector::edge::DataVectorEdge;

type IntPaths = RawIntPaths<i32>;
type IntShapes = RawIntShapes<i32>;
type VectorEdge = DataVectorEdge<i32>;

pub(crate) struct WorkspaceState {
    pub(crate) camera: Camera,
    pub(crate) sheet_state: SheetState,
    pub(crate) point_state: PointsEditorState,
    pub(crate) subj: IntPaths,
    pub(crate) clip: IntPaths,
    pub(crate) solution: IntShapes,
    pub(crate) points: Vec<EditorPoint>,
    pub(crate) vectors: Vec<VectorEdge>,
}

impl EditorApp {
    pub(crate) fn boolean_workspace(&mut self, ui: &mut egui::Ui) {
        self.update(AppMessage::Bool(BooleanMessage::WorkspaceSized(
            ui.available_size(),
        )));
        let workspace = &mut self.state.boolean.workspace;
        let (painter, update) = SheetWidget::show(
            ui,
            &mut workspace.camera,
            &workspace.points,
            &mut workspace.sheet_state,
            &mut workspace.point_state,
        );
        if let Some(update) = update {
            self.update(AppMessage::Bool(BooleanMessage::PointEdited(update)));
            ui.ctx().request_repaint();
        }
        let workspace = &self.state.boolean.workspace;
        if workspace.camera.is_not_empty() {
            match self.state.boolean.mode {
                ModeOption::Edit => {
                    ShapeWidget::with_paths(
                        &workspace.subj,
                        workspace.camera,
                        Some(self.state.boolean.fill.fill_rule()),
                        Some(Design::subject_color().gamma_multiply(0.2)),
                        Some(Design::subject_color()),
                        2.0,
                    )
                    .paint(&painter);
                    ShapeWidget::with_paths(
                        &workspace.clip,
                        workspace.camera,
                        Some(self.state.boolean.fill.fill_rule()),
                        Some(Design::clip_color().gamma_multiply(0.2)),
                        Some(Design::clip_color()),
                        2.0,
                    )
                    .paint(&painter);
                }
                ModeOption::Debug => {
                    VectorsWidget::with_vectors(
                        &workspace.vectors,
                        workspace.camera,
                        Design::subject_color(),
                        Design::clip_color(),
                        Design::both_color(),
                        2.0,
                    )
                    .paint(&painter);
                }
                _ => {
                    ShapeWidget::with_paths(
                        &workspace.subj,
                        workspace.camera,
                        Some(self.state.boolean.fill.fill_rule()),
                        None,
                        Some(Design::subject_color()),
                        1.0,
                    )
                    .paint(&painter);
                    ShapeWidget::with_paths(
                        &workspace.clip,
                        workspace.camera,
                        Some(self.state.boolean.fill.fill_rule()),
                        None,
                        Some(Design::clip_color()),
                        1.0,
                    )
                    .paint(&painter);
                    ShapeWidget::with_shapes(
                        &workspace.solution,
                        workspace.camera,
                        None,
                        Some(Design::solution_color().gamma_multiply(0.2)),
                        Some(Design::solution_color()),
                        2.0,
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
    pub(super) fn boolean_update_point(
        &mut self,
        update: crate::point_editor::widget::PointEditUpdate,
    ) {
        self.state.boolean.boolean_update_point(update);
    }
}
impl Default for WorkspaceState {
    fn default() -> Self {
        WorkspaceState {
            camera: Camera::empty(),
            sheet_state: Default::default(),
            point_state: Default::default(),
            subj: vec![],
            clip: vec![],
            solution: vec![],
            points: vec![],
            vectors: vec![],
        }
    }
}
