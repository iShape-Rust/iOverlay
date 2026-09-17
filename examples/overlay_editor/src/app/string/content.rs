use crate::app::fill_option::FillOption;
use crate::app::main::{AppMessage, EditorApp};
use crate::app::solver_option::SolverOption;
use crate::app::string::control::ModeOption;
use crate::app::string::workspace::{Solution, WorkspaceState};
use crate::data::string::StringResource;
use crate::geom::camera::Camera;
use crate::point_editor::point::PathsToEditorPoints;
use crate::point_editor::widget::PointEditUpdate;
use eframe::egui::{self, Vec2};
use i_triangle::i_overlay::i_float::int::rect::IntRect;
use i_triangle::i_overlay::i_shape::int::count::PointsCount;
use i_triangle::i_overlay::string::clip::{ClipRule, IntClip};
use i_triangle::i_overlay::string::slice::IntSlice;
use std::collections::HashMap;

pub(crate) struct StringState {
    pub(crate) test: usize,
    pub(crate) fill: FillOption,
    pub(crate) mode: ModeOption,
    pub(crate) solver: SolverOption,
    pub(crate) workspace: WorkspaceState,
    pub(crate) size: Vec2,
    pub(crate) cameras: HashMap<usize, Camera>,
}

#[derive(Debug, Clone)]
pub(crate) enum StringMessage {
    TestSelected(usize),
    FillSelected(FillOption),
    ModeSelected(ModeOption),
    SolverSelected(SolverOption),
    PointEdited(PointEditUpdate),
    WorkspaceSized(Vec2),
}

impl EditorApp {
    pub(crate) fn string_content(&mut self, ui: &mut egui::Ui) {
        egui::Panel::left("string_tests")
            .exact_size(150.0)
            .resizable(false)
            .show_inside(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for index in 0..self.app_resource.string.count {
                        let response = ui.selectable_label(
                            self.state.string.test == index,
                            format!("test_{index}"),
                        );
                        if response.clicked() {
                            response.surrender_focus();
                            self.update(AppMessage::String(StringMessage::TestSelected(index)));
                        }
                    }
                });
            });
        egui::CentralPanel::default().show_inside(ui, |ui| {
            self.string_control(ui);
            ui.separator();
            self.string_workspace(ui);
        });
    }

    pub(crate) fn string_update(&mut self, message: StringMessage) {
        match message {
            StringMessage::TestSelected(index) => self.string_set_test(index),
            StringMessage::SolverSelected(solver) => self.string_update_solver(solver),
            StringMessage::FillSelected(fill) => self.string_update_fill(fill),
            StringMessage::ModeSelected(mode) => self.string_update_mode(mode),
            StringMessage::PointEdited(update) => self.string_update_point(update),
            StringMessage::WorkspaceSized(size) => self.string_update_size(size),
        }
    }

    fn string_set_test(&mut self, index: usize) {
        self.state
            .string
            .set_test(index, &mut self.app_resource.string);
        self.state.string.update_solution();
    }

    pub(crate) fn string_init(&mut self) {
        self.string_set_test(self.state.string.test);
    }

    pub(crate) fn string_next_test(&mut self) {
        let next_test = self.state.string.test.saturating_add(1);
        if next_test < self.app_resource.string.count {
            self.string_set_test(next_test);
        }
    }

    pub(crate) fn string_prev_test(&mut self) {
        let test = self.state.string.test;
        if test >= 1 {
            self.string_set_test(test - 1);
        }
    }

    fn string_update_size(&mut self, size: Vec2) {
        self.state.string.size = size;
        let points = &self.state.string.workspace.points;
        if self.state.string.workspace.camera.is_empty() && !points.is_empty() {
            let rect = IntRect::with_iter(points.iter().map(|p| &p.pos))
                .unwrap_or(IntRect::new(-10_000, 10_000, -10_000, 10_000));
            let camera = Camera::new(rect, size);
            self.state.string.workspace.camera = camera;
        } else {
            self.state.string.workspace.camera.size = size;
        }
    }

    fn string_update_solver(&mut self, solver: SolverOption) {
        self.state.string.solver = solver;
        self.state.string.update_solution();
    }

    fn string_update_fill(&mut self, fill: FillOption) {
        self.state.string.fill = fill;
        self.state.string.update_solution();
    }

    fn string_update_mode(&mut self, mode: ModeOption) {
        self.state.string.mode = mode;
        self.state.string.update_solution();
    }
}

impl StringState {
    pub(crate) fn new(resource: &mut StringResource) -> Self {
        let mut state = StringState {
            test: usize::MAX,
            fill: FillOption::NonZero,
            mode: ModeOption::Slice,
            solver: SolverOption::Auto,
            workspace: Default::default(),
            cameras: HashMap::with_capacity(resource.count),
            size: Vec2::ZERO,
        };

        state.set_test(0, resource);
        state.update_solution();
        state
    }

    fn set_test(&mut self, index: usize, resource: &mut StringResource) {
        if let Some(test) = resource.load(index) {
            let editor_points = &mut self.workspace.points;

            if editor_points.is_empty() {
                editor_points.reserve(test.body.points_count() + test.string.points_count())
            } else {
                editor_points.clear();
            }

            self.workspace.body = test.body.clone();
            self.workspace.string = test.string.clone();

            self.workspace.body.feed_edit_points(0, editor_points);
            self.workspace.string.feed_edit_points(1, editor_points);

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
        let body = &self.workspace.body;
        let string = &self.workspace.string;
        let fill_rule = self.fill.fill_rule();
        match self.mode {
            ModeOption::Edit => {
                self.workspace.solution = Solution::None;
            }
            ModeOption::Debug => {
                self.workspace.solution = Solution::None;
            }
            ModeOption::Slice => {
                let slice = body.slice_by_paths(string, fill_rule);
                self.workspace.solution = Solution::Shapes(slice);
            }
            ModeOption::ClipDirect => {
                let clip = body.clip_paths(
                    string,
                    fill_rule,
                    ClipRule {
                        invert: false,
                        boundary_included: false,
                    },
                );
                self.workspace.solution = Solution::Paths(clip);
            }
            ModeOption::ClipInvert => {
                let clip = body.clip_paths(
                    string,
                    fill_rule,
                    ClipRule {
                        invert: true,
                        boundary_included: false,
                    },
                );
                self.workspace.solution = Solution::Paths(clip);
            }
        }
    }

    pub(super) fn string_update_point(&mut self, update: PointEditUpdate) {
        self.workspace.points[update.index] = update.point.clone();
        let m_index = update.point.index;
        if m_index.group_index == 0 {
            self.workspace.body[m_index.path_index][m_index.point_index] = update.point.pos;
        } else {
            self.workspace.string[m_index.path_index][m_index.point_index] = update.point.pos;
        }
        self.update_solution();
    }
}
