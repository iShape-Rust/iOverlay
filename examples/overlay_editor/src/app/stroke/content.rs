use crate::app::design;
use crate::app::main::{AppMessage, EditorApp};
use crate::app::stroke::control::{CapOption, JoinOption};
use crate::app::stroke::workspace::WorkspaceState;
use crate::data::stroke::StrokeResource;
use crate::geom::camera::Camera;
use crate::point_editor::point::PathsToEditorPoints;
use crate::point_editor::widget::PointEditUpdate;
use i_triangle::i_overlay::mesh::stroke::offset::StrokeOffset;
use i_triangle::i_overlay::mesh::style::{LineCap, LineJoin, StrokeStyle};
use i_triangle::i_overlay::i_float::int::point::IntPoint;
use i_triangle::i_overlay::i_float::int::rect::IntRect;
use iced::widget::{scrollable, Button, Column, Container, Row, Space, Text};
use iced::{Alignment, Length, Padding, Size, Vector};
use std::collections::HashMap;

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
    pub(crate) size: Size,
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
    WorkspaceSized(Size),
    WorkspaceZoomed(Camera),
    WorkspaceDragged(Vector<f32>),
}

impl EditorApp {
    fn stroke_sidebar(&self) -> Column<'_, AppMessage> {
        let count = self.app_resource.stroke.count;
        let mut column = Column::new().push(
            Space::new()
                .width(Length::Fill)
                .height(Length::Fixed(2.0)),
        );
        for index in 0..count {
            let is_selected = self.state.stroke.test == index;
            column = column.push(
                Container::new(
                    Button::new(
                        Text::new(format!("test_{}", index))
                            .style(if is_selected {
                                design::style_sidebar_text_selected
                            } else {
                                design::style_sidebar_text
                            })
                            .size(14),
                    )
                    .width(Length::Fill)
                    .on_press(AppMessage::Stroke(StrokeMessage::TestSelected(index)))
                    .style(if is_selected {
                        design::style_sidebar_button_selected
                    } else {
                        design::style_sidebar_button
                    }),
                )
                .padding(self.design.action_padding()),
            );
        }

        column
    }

    pub(crate) fn stroke_content(&self) -> Row<'_, AppMessage> {
        Row::new()
            .push(
                scrollable(
                    Container::new(self.stroke_sidebar())
                        .width(Length::Fixed(160.0))
                        .height(Length::Shrink)
                        .align_x(Alignment::Start)
                        .padding(Padding::new(0.0).right(8))
                        .style(design::style_sidebar_background),
                )
                .direction(scrollable::Direction::Vertical(
                    scrollable::Scrollbar::new()
                        .width(4)
                        .margin(0)
                        .scroller_width(4)
                        .anchor(scrollable::Anchor::Start),
                )),
            )
            .push(self.stroke_workspace())
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
            StrokeMessage::WorkspaceZoomed(zoom) => self.stroke_update_zoom(zoom),
            StrokeMessage::WorkspaceDragged(drag) => self.stroke_update_drag(drag),
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
        let next_test = self.state.stroke.test + 1;
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

    fn stroke_update_size(&mut self, size: Size) {
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
            size: Size::ZERO,
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
            },
            JoinOption::Round => {
                let ratio = 0.015 * self.join_value as f32;
                style = style.line_join(LineJoin::Round(ratio))
            }
            JoinOption::Bevel => style = style.line_join(LineJoin::Bevel),
        }

        match self.start_cap {
            CapOption::Butt => {
                style = style.start_cap(LineCap::Butt)
            },
            CapOption::Round => {
                let ratio = 0.015 * self.start_cap_value as f32;
                style = style.start_cap(LineCap::Round(ratio))
            }
            CapOption::Square => style = style.start_cap(LineCap::Square),
            CapOption::Arrow => {
                let points = vec![
                    [-1.0, -2.0],
                    [ 3.0,  0.0],
                    [-1.0,  2.0],
                ];
                style = style.start_cap(LineCap::Custom(points))
            }
        }

        match self.end_cap {
            CapOption::Butt => {
                style = style.end_cap(LineCap::Butt)
            },
            CapOption::Round => {
                let ratio = 0.015 * self.end_cap_value as f32;
                style = style.end_cap(LineCap::Round(ratio))
            }
            CapOption::Square => style = style.end_cap(LineCap::Square),
            CapOption::Arrow => {
                let points = vec![
                    [-1.0, -2.0],
                    [ 3.0,  0.0],
                    [-1.0,  2.0],
                ];
                style = style.end_cap(LineCap::Custom(points))
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
