use crate::app::design::Design;
use crate::app::main::{AppMessage, EditorApp};
use crate::app::outline::content::OutlineMessage;
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
    pub(crate) outline_input: IntPaths,
    pub(crate) outline_output: IntPaths,
    pub(crate) points: Vec<EditorPoint>,
}

impl EditorApp {
    pub(crate) fn outline_workspace(&mut self, ui: &mut egui::Ui) {
        self.update(AppMessage::Outline(OutlineMessage::WorkspaceSized(
            ui.available_size(),
        )));
        let workspace = &mut self.state.outline.workspace;
        let (painter, update) = SheetWidget::show(
            ui,
            &mut workspace.camera,
            &workspace.points,
            &mut workspace.sheet_state,
            &mut workspace.point_state,
        );
        if let Some(update) = update {
            self.update(AppMessage::Outline(OutlineMessage::PointEdited(update)));
            ui.ctx().request_repaint();
        }
        let workspace = &self.state.outline.workspace;
        if workspace.camera.is_not_empty() {
            ShapeWidget::with_paths(
                &workspace.outline_output,
                workspace.camera,
                Some(FillRule::NonZero),
                Some(Design::solution_color().gamma_multiply(0.1)),
                Some(Design::solution_color()),
                2.0,
            )
            .paint(&painter);
            ShapeWidget::with_paths(
                &workspace.outline_input,
                workspace.camera,
                Some(FillRule::NonZero),
                Some(Design::subject_color().gamma_multiply(0.1)),
                Some(Design::subject_color()),
                1.0,
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
    pub(super) fn outline_update_point(
        &mut self,
        update: crate::point_editor::widget::PointEditUpdate,
    ) {
        self.state.outline.outline_update_point(update);
    }
}
impl Default for WorkspaceState {
    fn default() -> Self {
        WorkspaceState {
            scale: 1.0,
            camera: Camera::empty(),
            sheet_state: Default::default(),
            point_state: Default::default(),
            outline_input: vec![],
            outline_output: vec![],
            points: vec![],
        }
    }
}
