use crate::app::boolean::control::ModeOption;
use crate::app::boolean::workspace::WorkspaceState;
use crate::app::fill_option::FillOption;
use crate::app::main::{AppMessage, EditorApp};
use crate::app::solver_option::SolverOption;
use crate::data::boolean::BooleanResource;
use crate::geom::camera::Camera;
use crate::point_editor::point::PathsToEditorPoints;
use crate::point_editor::widget::PointEditUpdate;
use eframe::egui::{self, Vec2};
use i_triangle::i_overlay::core::overlay::Overlay;
use i_triangle::i_overlay::i_float::int::rect::IntRect;
use i_triangle::i_overlay::i_shape::int::count::PointsCount;
use std::collections::HashMap;

pub(crate) struct BooleanState {
    pub(crate) test: usize,
    pub(crate) fill: FillOption,
    pub(crate) mode: ModeOption,
    pub(crate) solver: SolverOption,
    pub(crate) workspace: WorkspaceState,
    pub(crate) size: Vec2,
    pub(crate) cameras: HashMap<usize, Camera>,
}

#[derive(Debug, Clone)]
pub(crate) enum BooleanMessage {
    TestSelected(usize),
    FillSelected(FillOption),
    ModeSelected(ModeOption),
    SolverSelected(SolverOption),
    PointEdited(PointEditUpdate),
    WorkspaceSized(Vec2),
}

impl EditorApp {
    pub(crate) fn boolean_content(&mut self, ui: &mut egui::Ui) {
        egui::Panel::left("boolean_tests")
            .exact_size(150.0)
            .resizable(false)
            .show_inside(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for index in 0..self.app_resource.boolean.count {
                        let response = ui.selectable_label(
                            self.state.boolean.test == index,
                            format!("test_{index}"),
                        );
                        if response.clicked() {
                            response.surrender_focus();
                            self.update(AppMessage::Bool(BooleanMessage::TestSelected(index)));
                        }
                    }
                });
            });
        egui::CentralPanel::default().show_inside(ui, |ui| {
            self.boolean_control(ui);
            ui.separator();
            self.boolean_workspace(ui);
        });
    }

    pub(crate) fn boolean_update(&mut self, message: BooleanMessage) {
        match message {
            BooleanMessage::TestSelected(index) => self.boolean_set_test(index),
            BooleanMessage::SolverSelected(solver) => self.boolean_update_solver(solver),
            BooleanMessage::FillSelected(fill) => self.boolean_update_fill(fill),
            BooleanMessage::ModeSelected(mode) => self.boolean_update_mode(mode),
            BooleanMessage::PointEdited(update) => self.boolean_update_point(update),
            BooleanMessage::WorkspaceSized(size) => self.boolean_update_size(size),
        }
    }

    fn boolean_set_test(&mut self, index: usize) {
        self.state
            .boolean
            .load_test(index, &mut self.app_resource.boolean);
        self.state.boolean.update_solution();
    }

    pub(crate) fn boolean_init(&mut self) {
        self.boolean_set_test(self.state.boolean.test);
    }

    pub(crate) fn boolean_next_test(&mut self) {
        let next_test = self.state.boolean.test.saturating_add(1);
        if next_test < self.app_resource.boolean.count {
            self.boolean_set_test(next_test);
        }
    }

    pub(crate) fn boolean_prev_test(&mut self) {
        let test = self.state.boolean.test;
        if test >= 1 {
            self.boolean_set_test(test - 1);
        }
    }

    fn boolean_update_size(&mut self, size: Vec2) {
        self.state.boolean.size = size;
        let points = &self.state.boolean.workspace.points;
        if self.state.boolean.workspace.camera.is_empty() && !points.is_empty() {
            let rect = IntRect::with_iter(points.iter().map(|p| &p.pos))
                .unwrap_or(IntRect::new(-10_000, 10_000, -10_000, 10_000));
            let camera = Camera::new(rect, size);
            self.state.boolean.workspace.camera = camera;
        } else {
            self.state.boolean.workspace.camera.size = size;
        }
    }

    fn boolean_update_solver(&mut self, solver: SolverOption) {
        self.state.boolean.solver = solver;
        self.state.boolean.update_solution();
    }

    fn boolean_update_fill(&mut self, fill: FillOption) {
        self.state.boolean.fill = fill;
        self.state.boolean.update_solution();
    }

    fn boolean_update_mode(&mut self, mode: ModeOption) {
        self.state.boolean.mode = mode;
        self.state.boolean.update_solution();
    }
}

impl BooleanState {
    pub(crate) fn new(resource: &mut BooleanResource) -> Self {
        let mut state = BooleanState {
            test: usize::MAX,
            fill: FillOption::NonZero,
            mode: ModeOption::Xor,
            solver: SolverOption::Auto,
            workspace: Default::default(),
            cameras: HashMap::with_capacity(resource.count),
            size: Vec2::ZERO,
        };

        state.load_test(0, resource);
        state.update_solution();
        state
    }

    fn load_test(&mut self, index: usize, resource: &mut BooleanResource) {
        if let Some(test) = resource.load(index) {
            let editor_points = &mut self.workspace.points;

            if editor_points.is_empty() {
                editor_points
                    .reserve(test.clip_paths.points_count() + test.subj_paths.points_count())
            } else {
                editor_points.clear();
            }

            self.workspace.subj = test.subj_paths.clone();
            self.workspace.clip = test.clip_paths.clone();

            self.workspace.subj.feed_edit_points(0, editor_points);
            self.workspace.clip.feed_edit_points(1, editor_points);

            self.cameras.insert(self.test, self.workspace.camera);
            let mut camera = *self.cameras.get(&index).unwrap_or(&Camera::empty());
            if camera.is_empty() && self.size.x > 0.001 {
                let rect = IntRect::with_iter(editor_points.iter().map(|p| &p.pos))
                    .unwrap_or(IntRect::new(-10_000, 10_000, -10_000, 10_000));
                camera = Camera::new(rect, self.size);
            }

            self.workspace.camera = camera;
            self.workspace.sheet_state = Default::default();
            self.workspace.point_state = Default::default();

            self.test = index;
        }
    }

    fn update_solution(&mut self) {
        let subj = &self.workspace.subj;
        let clip = &self.workspace.clip;
        let fill_rule = self.fill.fill_rule();
        match self.mode {
            ModeOption::Edit => {}
            ModeOption::Debug => {
                self.workspace.vectors =
                    Overlay::with_contours(subj, clip).build_separate_vectors(fill_rule);
            }
            _ => {
                let overlay_rule = self.mode.overlay_rule().unwrap();
                let solution = Overlay::with_contours(subj, clip).overlay(overlay_rule, fill_rule);
                self.workspace.solution = solution;
            }
        }
    }

    pub(super) fn boolean_update_point(&mut self, update: PointEditUpdate) {
        self.workspace.points[update.index] = update.point.clone();
        let m_index = update.point.index;
        if m_index.group_index == 0 {
            self.workspace.subj[m_index.path_index][m_index.point_index] = update.point.pos;
        } else {
            self.workspace.clip[m_index.path_index][m_index.point_index] = update.point.pos;
        }
        self.update_solution();
    }
}
