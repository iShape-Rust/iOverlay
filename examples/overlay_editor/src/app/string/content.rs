use std::collections::HashMap;
use i_triangle::i_overlay::i_shape::int::count::PointsCount;
use i_triangle::i_overlay::i_float::int::rect::IntRect;
use i_triangle::i_overlay::string::clip::{ClipRule, IntClip};
use i_triangle::i_overlay::string::slice::IntSlice;
use iced::widget::scrollable;
use iced::{Alignment, Length, Padding, Size, Vector};
use iced::widget::{Button, Column, Container, Row, Space, Text};
use crate::app::design;
use crate::app::string::control::ModeOption;
use crate::app::string::workspace::{Solution, WorkspaceState};
use crate::app::fill_option::FillOption;
use crate::app::main::{EditorApp, AppMessage};
use crate::app::solver_option::SolverOption;
use crate::geom::camera::Camera;
use crate::data::string::StringResource;
use crate::point_editor::point::PathsToEditorPoints;
use crate::point_editor::widget::PointEditUpdate;

pub(crate) struct StringState {
    pub(crate) test: usize,
    pub(crate) fill: FillOption,
    pub(crate) mode: ModeOption,
    pub(crate) solver: SolverOption,
    pub(crate) workspace: WorkspaceState,
    pub(crate) size: Size,
    pub(crate) cameras: HashMap<usize, Camera>,
}

#[derive(Debug, Clone)]
pub(crate) enum StringMessage {
    TestSelected(usize),
    FillSelected(FillOption),
    ModeSelected(ModeOption),
    SolverSelected(SolverOption),
    PointEdited(PointEditUpdate),
    WorkspaceSized(Size),
    WorkspaceZoomed(Camera),
    WorkspaceDraged(Vector<f32>),
}

impl EditorApp {
    fn string_sidebar(&self) -> Column<AppMessage> {
        let count = self.app_resource.string.count;
        let mut column = Column::new().push(Space::new(Length::Fill, Length::Fixed(2.0)));
        for index in 0..count {
            let is_selected = self.state.string.test == index;
            column = column.push(
                Container::new(
                    Button::new(
                        Text::new(format!("test_{}", index))
                            .style(if is_selected { design::style_sidebar_text_selected } else { design::style_sidebar_text })
                            .size(14)
                    )
                        .width(Length::Fill)
                        .on_press(AppMessage::String(StringMessage::TestSelected(index)))
                        .style(if is_selected { design::style_sidebar_button_selected } else { design::style_sidebar_button })
                ).padding(self.design.action_padding())
            );
        }

        column
    }

    pub(crate) fn string_content(&self) -> Row<AppMessage> {
        Row::new()
            .push(
                scrollable(
                    Container::new(self.string_sidebar())
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
            .push(self.string_workspace())
    }

    pub(crate) fn string_update(&mut self, message: StringMessage) {
        match message {
            StringMessage::TestSelected(index) => self.string_set_test(index),
            StringMessage::SolverSelected(solver) => self.string_update_solver(solver),
            StringMessage::FillSelected(fill) => self.string_update_fill(fill),
            StringMessage::ModeSelected(mode) => self.string_update_mode(mode),
            StringMessage::PointEdited(update) => self.string_update_point(update),
            StringMessage::WorkspaceSized(size) => self.string_update_size(size),
            StringMessage::WorkspaceZoomed(zoom) => self.string_update_zoom(zoom),
            StringMessage::WorkspaceDraged(drag) => self.string_update_drag(drag),
        }
    }

    fn string_set_test(&mut self, index: usize) {
        self.state.string.set_test(index, &mut self.app_resource.string);
        self.state.string.update_solution();
    }

    pub(crate) fn string_init(&mut self) {
        self.string_set_test(self.state.string.test);
    }

    pub(crate) fn string_next_test(&mut self) {
        let next_test = self.state.string.test + 1;
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

    fn string_update_size(&mut self, size: Size) {
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
            size: Size::ZERO,
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
                let clip = body.clip_paths(string, fill_rule, ClipRule { invert: false, boundary_included: false });
                self.workspace.solution = Solution::Paths(clip);
            }
            ModeOption::ClipInvert => {
                let clip = body.clip_paths(string, fill_rule, ClipRule { invert: true, boundary_included: false });
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

