use crate::app::design::Design;
use crate::app::main::{AppMessage, EditorApp};
use crate::app::stroke::content::StrokeMessage;
use crate::draw::path::PathWidget;
use crate::draw::shape::ShapeWidget;
use crate::geom::camera::Camera;
use crate::point_editor::point::EditorPoint;
use crate::point_editor::{state::PointsEditorState, widget::PointsEditorWidget};
use crate::sheet::state::SheetState;
use crate::sheet::widget::SheetWidget;
use eframe::egui;
use i_triangle::i_overlay::core::fill_rule::FillRule;
use i_triangle::i_overlay::i_shape::int::path::IntPaths as RawIntPaths;

type IntPaths = RawIntPaths<i32>;

pub(crate) struct WorkspaceState {
    pub(crate) camera: Camera,
    pub(crate) sheet_state: SheetState,
    pub(crate) point_state: PointsEditorState,
    pub(crate) scale: f32,
    pub(crate) stroke_input: IntPaths,
    pub(crate) stroke_output: IntPaths,
    pub(crate) points: Vec<EditorPoint>,
}

impl EditorApp {
    pub(crate) fn stroke_workspace(&mut self, ui: &mut egui::Ui) {
        self.update(AppMessage::Stroke(StrokeMessage::WorkspaceSized(
            ui.available_size(),
        )));
        let workspace = &mut self.state.stroke.workspace;
        let (painter, update) = SheetWidget::show(
            ui,
            &mut workspace.camera,
            &workspace.points,
            &mut workspace.sheet_state,
            &mut workspace.point_state,
        );
        if let Some(update) = update {
            self.update(AppMessage::Stroke(StrokeMessage::PointEdited(update)));
            ui.ctx().request_repaint();
        }
        let workspace = &self.state.stroke.workspace;
        if workspace.camera.is_not_empty() {
            let shapes = &workspace.stroke_output;
            if !shapes.is_empty() {
                ShapeWidget::with_paths(
                    &workspace.stroke_output,
                    workspace.camera,
                    Some(FillRule::NonZero),
                    Some(Design::solution_color().gamma_multiply(0.1)),
                    Some(Design::solution_color()),
                    2.0,
                )
                .paint(&painter);
            }
            PathWidget::with_paths(
                &workspace.stroke_input,
                workspace.camera,
                Design::subject_color(),
                1.0,
                false,
            )
            .paint(&painter);
        }
        PointsEditorWidget::paint(
            &painter,
            workspace.camera,
            &workspace.points,
            &workspace.point_state,
        );
    }
    pub(super) fn stroke_update_point(
        &mut self,
        update: crate::point_editor::widget::PointEditUpdate,
    ) {
        self.state.stroke.stroke_update_point(update);
    }
}
impl Default for WorkspaceState {
    fn default() -> Self {
        WorkspaceState {
            scale: 1.0,
            camera: Camera::empty(),
            sheet_state: Default::default(),
            point_state: Default::default(),
            stroke_input: vec![],
            stroke_output: vec![],
            points: vec![],
        }
    }
}
