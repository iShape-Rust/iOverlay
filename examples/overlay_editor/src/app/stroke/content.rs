use crate::app::main::{AppMessage, EditorApp};
use crate::app::stroke::control::{CapOption, JoinOption};
use crate::app::stroke::workspace::WorkspaceState;
use crate::data::stroke::StrokeResource;
use crate::geom::camera::Camera;
use crate::point_editor::point::PathsToEditorPoints;
use crate::point_editor::widget::PointEditUpdate;
use eframe::egui::{self, Vec2};
use i_triangle::i_overlay::i_float::int::point::IntPoint;
use i_triangle::i_overlay::i_float::int::rect::IntRect;
use i_triangle::i_overlay::mesh::float::stroke::offset::StrokeOffset;
use i_triangle::i_overlay::mesh::float::style::{LineCap, LineJoin, StrokeStyle};
use std::collections::HashMap;
use std::rc::Rc;

pub(crate) struct StrokeState {
    pub(crate) test: usize,
    pub(crate) width: f32,
    pub(crate) is_closed: bool,
    pub(crate) start_cap: CapOption,
    pub(crate) start_cap_value: u8,
    pub(crate) end_cap: CapOption,
    pub(crate) end_cap_value: u8,
    pub(crate) join: JoinOption,
    pub(crate) join_value: u8,
    pub(crate) workspace: WorkspaceState,
    pub(crate) size: Vec2,
    pub(crate) cameras: HashMap<usize, Camera>,
}

#[derive(Debug, Clone)]
pub(crate) enum StrokeMessage {
    TestSelected(usize),
    WidthValueUpdated(f32),
    IsClosedUpdated(bool),
    StartCapSelected(CapOption),
    StartCapValueUpdated(u8),
    EndCapSelected(CapOption),
    EndCapValueUpdated(u8),
    JoinSelected(JoinOption),
    JoinValueUpdated(u8),
    PointEdited(PointEditUpdate),
    WorkspaceSized(Vec2),
}

impl EditorApp {
    pub(crate) fn stroke_content(&mut self, ui: &mut egui::Ui) {
        egui::Panel::left("stroke_tests")
            .exact_size(150.0)
            .resizable(false)
            .show_inside(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for index in 0..self.app_resource.stroke.count {
                        let response = ui.selectable_label(
                            self.state.stroke.test == index,
                            format!("test_{index}"),
                        );
                        if response.clicked() {
                            response.surrender_focus();
                            self.update(AppMessage::Stroke(StrokeMessage::TestSelected(index)));
                        }
                    }
                });
            });
        egui::CentralPanel::default().show_inside(ui, |ui| {
            self.stroke_control(ui);
            ui.separator();
            self.stroke_workspace(ui);
        });
    }

    pub(crate) fn stroke_update(&mut self, message: StrokeMessage) {
        match message {
            StrokeMessage::TestSelected(index) => self.stroke_set_test(index),
            StrokeMessage::IsClosedUpdated(value) => self.stroke_update_is_closed(value),
            StrokeMessage::WidthValueUpdated(value) => self.stroke_update_width(value),
            StrokeMessage::StartCapSelected(cap) => self.stroke_update_start_cap(cap),
            StrokeMessage::StartCapValueUpdated(value) => self.stroke_update_start_cap_value(value),
            StrokeMessage::EndCapSelected(cap) => self.stroke_update_end_cap(cap),
            StrokeMessage::EndCapValueUpdated(value) => self.stroke_update_end_cap_value(value),
            StrokeMessage::JoinSelected(join) => self.stroke_update_join(join),
            StrokeMessage::JoinValueUpdated(value) => self.stroke_update_join_value(value),
            StrokeMessage::PointEdited(update) => self.stroke_update_point(update),
            StrokeMessage::WorkspaceSized(size) => self.stroke_update_size(size),
        }
    }

    fn stroke_set_test(&mut self, index: usize) {
        self.state
            .stroke
            .set_test(index, &mut self.app_resource.stroke);
        self.state.stroke.update_solution();
    }

    pub(crate) fn stroke_init(&mut self) {
        self.stroke_set_test(self.state.stroke.test);
    }

    pub(crate) fn stroke_next_test(&mut self) {
        let next_test = self.state.stroke.test.saturating_add(1);
        if next_test < self.app_resource.stroke.count {
            self.stroke_set_test(next_test);
        }
    }

    pub(crate) fn stroke_prev_test(&mut self) {
        let test = self.state.stroke.test;
        if test >= 1 {
            self.stroke_set_test(test - 1);
        }
    }

    fn stroke_update_size(&mut self, size: Vec2) {
        self.state.stroke.size = size;
        let points = &self.state.stroke.workspace.points;
        if self.state.stroke.workspace.camera.is_empty() && !points.is_empty() {
            let rect = IntRect::with_iter(points.iter().map(|p| &p.pos))
                .unwrap_or(IntRect::new(-10_000, 10_000, -10_000, 10_000));
            let camera = Camera::new(rect, size);
            self.state.stroke.workspace.camera = camera;
        } else {
            self.state.stroke.workspace.camera.size = size;
        }
    }

    fn stroke_update_is_closed(&mut self, is_closed: bool) {
        self.state.stroke.is_closed = is_closed;
        self.state.stroke.update_solution();
    }

    fn stroke_update_width(&mut self, width: f32) {
        self.state.stroke.width = width;
        self.state.stroke.update_solution();
    }

    fn stroke_update_start_cap(&mut self, cap: CapOption) {
        self.state.stroke.start_cap = cap;
        self.state.stroke.update_solution();
    }

    fn stroke_update_start_cap_value(&mut self, cap_value: u8) {
        self.state.stroke.start_cap_value = cap_value;
        self.state.stroke.update_solution();
    }

    fn stroke_update_end_cap(&mut self, cap: CapOption) {
        self.state.stroke.end_cap = cap;
        self.state.stroke.update_solution();
    }

    fn stroke_update_end_cap_value(&mut self, cap_value: u8) {
        self.state.stroke.end_cap_value = cap_value;
        self.state.stroke.update_solution();
    }

    fn stroke_update_join(&mut self, join: JoinOption) {
        self.state.stroke.join = join;
        self.state.stroke.update_solution();
    }

    fn stroke_update_join_value(&mut self, value: u8) {
        self.state.stroke.join_value = value;
        self.state.stroke.update_solution();
    }
}

impl StrokeState {
    pub(crate) fn new(resource: &mut StrokeResource) -> Self {
        let mut state = StrokeState {
            test: usize::MAX,
            width: 1.0,
            is_closed: false,
            start_cap: CapOption::Butt,
            start_cap_value: 50,
            end_cap: CapOption::Butt,
            end_cap_value: 50,
            join: JoinOption::Bevel,
            join_value: 50,
            workspace: Default::default(),
            cameras: HashMap::with_capacity(resource.count),
            size: Vec2::ZERO,
        };

        state.set_test(0, resource);
        state.update_solution();
        state
    }

    fn set_test(&mut self, index: usize, resource: &mut StrokeResource) {
        if let Some(test) = resource.load(index) {
            self.workspace.scale = test.scale;
            let editor_points = &mut self.workspace.points;
            if editor_points.is_empty() {
                let count = test.stroke.iter().fold(0, |acc, path| acc + path.len());
                editor_points.reserve(count)
            } else {
                editor_points.clear();
            }

            let mut stroke_input = Vec::with_capacity(test.stroke.len());
            for path in test.stroke.iter() {
                let mut int_path = Vec::with_capacity(path.len());
                for p in path.iter() {
                    let x = (test.scale * p[0]) as i32;
                    let y = (test.scale * p[1]) as i32;
                    int_path.push(IntPoint::new(x, y));
                }
                stroke_input.push(int_path);
            }

            self.workspace.stroke_input = stroke_input;
            self.workspace
                .stroke_input
                .feed_edit_points(0, editor_points);

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
        let stroke_input = &self.workspace.stroke_input;
        let mut float_paths = Vec::with_capacity(stroke_input.len());
        let scale = 1.0 / self.workspace.scale;
        for path in stroke_input.iter() {
            let mut float_path = Vec::with_capacity(path.len());
            for p in path.iter() {
                let x = scale * p.x as f32;
                let y = scale * p.y as f32;
                float_path.push([x, y]);
            }
            float_paths.push(float_path);
        }

        let mut style = StrokeStyle::new(self.width);
        match self.join {
            JoinOption::Miter => {
                let ratio = 0.03 * self.join_value as f32;
                println!("ratio: {}", ratio);
                style = style.line_join(LineJoin::Miter(ratio))
            }
            JoinOption::Round => {
                let ratio = 0.015 * self.join_value as f32;
                style = style.line_join(LineJoin::Round(ratio))
            }
            JoinOption::Bevel => style = style.line_join(LineJoin::Bevel),
        }

        match self.start_cap {
            CapOption::Butt => style = style.start_cap(LineCap::Butt),
            CapOption::Round => {
                let ratio = 0.015 * self.start_cap_value as f32;
                style = style.start_cap(LineCap::Round(ratio))
            }
            CapOption::Square => style = style.start_cap(LineCap::Square),
            CapOption::Arrow => {
                let points = vec![[-1.0, -2.0], [3.0, 0.0], [-1.0, 2.0]];
                style = style.start_cap(LineCap::Custom(Rc::from(points)))
            }
        }

        match self.end_cap {
            CapOption::Butt => style = style.end_cap(LineCap::Butt),
            CapOption::Round => {
                let ratio = 0.015 * self.end_cap_value as f32;
                style = style.end_cap(LineCap::Round(ratio))
            }
            CapOption::Square => style = style.end_cap(LineCap::Square),
            CapOption::Arrow => {
                let points = vec![[-1.0, -2.0], [3.0, 0.0], [-1.0, 2.0]];
                style = style.end_cap(LineCap::Custom(Rc::from(points)))
            }
        }

        let float_shapes = float_paths.stroke(style, self.is_closed);

        let scale = self.workspace.scale;
        let mut int_paths = Vec::with_capacity(float_shapes.len());
        for float_shape in float_shapes.iter() {
            for float_path in float_shape.iter() {
                let mut path = Vec::with_capacity(float_path.len());
                for p in float_path.iter() {
                    let x = (scale * p[0]) as i32;
                    let y = (scale * p[1]) as i32;
                    path.push(IntPoint::new(x, y));
                }
                int_paths.push(path);
            }
        }

        self.workspace.stroke_output = int_paths
    }

    pub(super) fn stroke_update_point(&mut self, update: PointEditUpdate) {
        self.workspace.points[update.index] = update.point.clone();
        let m_index = update.point.index;
        self.workspace.stroke_input[m_index.path_index][m_index.point_index] = update.point.pos;
        self.update_solution();
    }
}
