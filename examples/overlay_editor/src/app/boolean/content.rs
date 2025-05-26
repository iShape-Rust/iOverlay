use std::collections::HashMap;
use i_triangle::i_overlay::core::overlay::Overlay;
use i_triangle::i_overlay::i_shape::int::count::PointsCount;
use i_triangle::i_overlay::i_float::int::rect::IntRect;
use iced::widget::scrollable;
use iced::{Alignment, Length, Padding, Size, Vector};
use iced::widget::{Button, Column, Container, Row, Space, Text};
use crate::app::design;
use crate::app::boolean::control::ModeOption;
use crate::app::boolean::workspace::WorkspaceState;
use crate::app::fill_option::FillOption;
use crate::app::main::{EditorApp, AppMessage};
use crate::app::solver_option::SolverOption;
use crate::geom::camera::Camera;
use crate::data::boolean::BooleanResource;
use crate::point_editor::point::PathsToEditorPoints;
use crate::point_editor::widget::PointEditUpdate;

pub(crate) struct BooleanState {
    pub(crate) test: usize,
    pub(crate) fill: FillOption,
    pub(crate) mode: ModeOption,
    pub(crate) solver: SolverOption,
    pub(crate) workspace: WorkspaceState,
    pub(crate) size: Size,
    pub(crate) cameras: HashMap<usize, Camera>,
}

#[derive(Debug, Clone)]
pub(crate) enum BooleanMessage {
    TestSelected(usize),
    FillSelected(FillOption),
    ModeSelected(ModeOption),
    SolverSelected(SolverOption),
    PointEdited(PointEditUpdate),
    WorkspaceSized(Size),
    WorkspaceZoomed(Camera),
    WorkspaceDragged(Vector<f32>),
}

impl EditorApp {
    fn boolean_sidebar(&self) -> Column<AppMessage> {
        let count = self.app_resource.boolean.count;
        let mut column = Column::new().push(Space::new(Length::Fill, Length::Fixed(2.0)));
        for index in 0..count {
            let is_selected = self.state.boolean.test == index;

            column = column.push(
                Container::new(
                    Button::new(
                        Text::new(format!("test_{}", index))
                            .style(if is_selected { design::style_sidebar_text_selected } else { design::style_sidebar_text })
                            .size(14)
                    )
                        .width(Length::Fill)
                        .on_press(AppMessage::Bool(BooleanMessage::TestSelected(index)))
                        .style(if is_selected { design::style_sidebar_button_selected } else { design::style_sidebar_button })
                ).padding(self.design.action_padding())
            );
        }

        column
    }

    pub(crate) fn boolean_content(&self) -> Row<AppMessage> {
        Row::new()
            .push(
                scrollable(
                    Container::new(self.boolean_sidebar())
                        .width(Length::Fixed(160.0))
                        .height(Length::Shrink)
                        .align_x(Alignment::Start)
                        .padding(Padding::new(0.0).right(8))
                        .style(design::style_sidebar_background)
                ).direction(scrollable::Direction::Vertical(
                    scrollable::Scrollbar::new()
                        .width(4)
                        .margin(0)
                        .scroller_width(4)
                        .anchor(scrollable::Anchor::Start),
                ))
            )
            .push(self.boolean_workspace())
    }

    pub(crate) fn boolean_update(&mut self, message: BooleanMessage) {
        match message {
            BooleanMessage::TestSelected(index) => self.boolean_set_test(index),
            BooleanMessage::SolverSelected(solver) => self.boolean_update_solver(solver),
            BooleanMessage::FillSelected(fill) => self.boolean_update_fill(fill),
            BooleanMessage::ModeSelected(mode) => self.boolean_update_mode(mode),
            BooleanMessage::PointEdited(update) => self.boolean_update_point(update),
            BooleanMessage::WorkspaceSized(size) => self.boolean_update_size(size),
            BooleanMessage::WorkspaceZoomed(zoom) => self.boolean_update_zoom(zoom),
            BooleanMessage::WorkspaceDragged(drag) => self.boolean_update_drag(drag),
        }
    }

    fn boolean_set_test(&mut self, index: usize) {
        self.state.boolean.load_test(index, &mut self.app_resource.boolean);
        self.state.boolean.update_solution();
    }

    pub(crate) fn boolean_init(&mut self) {
        self.boolean_set_test(self.state.boolean.test);
    }

    pub(crate) fn boolean_next_test(&mut self) {
        let next_test = self.state.boolean.test + 1;
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

    fn boolean_update_size(&mut self, size: Size) {
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
            size: Size::ZERO,
        };

        state.load_test(0, resource);
        state.update_solution();
        state
    }

    fn load_test(&mut self, index: usize, resource: &mut BooleanResource) {
        if let Some(test) = resource.load(index) {
            let editor_points = &mut self.workspace.points;

            if editor_points.is_empty() {
                editor_points.reserve(test.clip_paths.points_count() + test.subj_paths.points_count())
            } else {
                editor_points.clear();
            }

            self.workspace.subj = test.subj_paths.clone();
            self.workspace.clip = test.clip_paths.clone();

            self.workspace.subj.feed_edit_points(0, editor_points);
            self.workspace.clip.feed_edit_points(1, editor_points);

            self.cameras.insert(self.test, self.workspace.camera);
            let mut camera = *self.cameras.get(&index).unwrap_or(&Camera::empty());
            if camera.is_empty() && self.size.width > 0.001 {
                let rect = IntRect::with_iter(editor_points.iter().map(|p| &p.pos))
                    .unwrap_or(IntRect::new(-10_000, 10_000, -10_000, 10_000));
                camera = Camera::new(rect, self.size);
            }

            self.workspace.camera = camera;

            self.test = index;
        }
    }

    fn update_solution(&mut self) {
        let subj = &self.workspace.subj;
        let clip = &self.workspace.clip;
        let fill_rule = self.fill.fill_rule();
        match self.mode {
            ModeOption::Edit => {},
            ModeOption::Debug => {
                self.workspace.vectors = Overlay::with_contours(subj, clip).build_separate_vectors(fill_rule);
            },
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

