use crate::app::design;
use crate::app::main::{AppMessage, EditorApp};
use crate::app::outline::control::JoinOption;
use crate::app::outline::workspace::WorkspaceState;
use crate::data::outline::OutlineResource;
use crate::geom::camera::Camera;
use crate::point_editor::point::PathsToEditorPoints;
use crate::point_editor::widget::PointEditUpdate;
use i_triangle::i_overlay::mesh::style::{LineJoin, OutlineStyle};
use i_triangle::i_overlay::i_float::int::point::IntPoint;
use i_triangle::i_overlay::i_float::int::rect::IntRect;
use iced::widget::{scrollable, Button, Column, Container, Row, Space, Text};
use iced::{Alignment, Length, Padding, Size, Vector};
use std::collections::HashMap;

pub(crate) struct OutlineState {
    pub(crate) test: usize,
    pub(crate) offset: f32,
    pub(crate) join: JoinOption,
    pub(crate) join_value: u8,
    pub(crate) workspace: WorkspaceState,
    pub(crate) size: Size,
    pub(crate) cameras: HashMap<usize, Camera>,
}

#[derive(Debug, Clone)]
pub(crate) enum OutlineMessage {
    TestSelected(usize),
    OffsetValueUpdated(f32),
    JoinSelected(JoinOption),
    JoinValueUpdated(u8),
    PointEdited(PointEditUpdate),
    WorkspaceSized(Size),
    WorkspaceZoomed(Camera),
    WorkspaceDragged(Vector<f32>),
}

impl EditorApp {
    fn outline_sidebar(&self) -> Column<AppMessage> {
        let count = self.app_resource.outline.count;
        let mut column = Column::new().push(Space::new(Length::Fill, Length::Fixed(2.0)));
        for index in 0..count {
            let is_selected = self.state.outline.test == index;
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
                    .on_press(AppMessage::Outline(OutlineMessage::TestSelected(index)))
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

    pub(crate) fn outline_content(&self) -> Row<AppMessage> {
        Row::new()
            .push(
                scrollable(
                    Container::new(self.outline_sidebar())
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
            .push(self.outline_workspace())
    }

    pub(crate) fn outline_update(&mut self, message: OutlineMessage) {
        match message {
            OutlineMessage::TestSelected(index) => self.outline_set_test(index),
            OutlineMessage::OffsetValueUpdated(value) => self.outline_update_offset(value),
            OutlineMessage::JoinSelected(join) => self.outline_update_join(join),
            OutlineMessage::JoinValueUpdated(value) => self.outline_update_join_value(value),
            OutlineMessage::PointEdited(update) => self.outline_update_point(update),
            OutlineMessage::WorkspaceSized(size) => self.outline_update_size(size),
            OutlineMessage::WorkspaceZoomed(zoom) => self.outline_update_zoom(zoom),
            OutlineMessage::WorkspaceDragged(drag) => self.outline_update_drag(drag),
        }
    }

    fn outline_set_test(&mut self, index: usize) {
        self.state
            .outline
            .set_test(index, &mut self.app_resource.outline);
        self.state.outline.update_solution();
    }

    pub(crate) fn outline_init(&mut self) {
        self.outline_set_test(self.state.outline.test);
    }

    pub(crate) fn outline_next_test(&mut self) {
        let next_test = self.state.outline.test + 1;
        if next_test < self.app_resource.outline.count {
            self.outline_set_test(next_test);
        }
    }

    pub(crate) fn outline_prev_test(&mut self) {
        let test = self.state.outline.test;
        if test >= 1 {
            self.outline_set_test(test - 1);
        }
    }

    fn outline_update_size(&mut self, size: Size) {
        self.state.outline.size = size;
        let points = &self.state.outline.workspace.points;
        if self.state.outline.workspace.camera.is_empty() && !points.is_empty() {
            let rect = IntRect::with_iter(points.iter().map(|p| &p.pos))
                .unwrap_or(IntRect::new(-10_000, 10_000, -10_000, 10_000));
            let camera = Camera::new(rect, size);
            self.state.outline.workspace.camera = camera;
        } else {
            self.state.outline.workspace.camera.size = size;
        }
    }

    fn outline_update_offset(&mut self, width: f32) {
        self.state.outline.offset = width;
        self.state.outline.update_solution();
    }

    fn outline_update_join(&mut self, join: JoinOption) {
        self.state.outline.join = join;
        self.state.outline.update_solution();
    }

    fn outline_update_join_value(&mut self, value: u8) {
        self.state.outline.join_value = value;
        self.state.outline.update_solution();
    }
}

impl OutlineState {
    pub(crate) fn new(resource: &mut OutlineResource) -> Self {
        let mut state = OutlineState {
            test: usize::MAX,
            offset: 1.0,
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

    fn set_test(&mut self, index: usize, resource: &mut OutlineResource) {
        if let Some(test) = resource.load(index) {
            self.workspace.scale = test.scale;
            let editor_points = &mut self.workspace.points;
            if editor_points.is_empty() {
                let count = test.outline.iter().fold(0, |acc, path| acc + path.len());
                editor_points.reserve(count)
            } else {
                editor_points.clear();
            }

            let mut outline_input = Vec::with_capacity(test.outline.len());
            for path in test.outline.iter() {
                let mut int_path = Vec::with_capacity(path.len());
                for p in path.iter() {
                    let x = (test.scale * p[0]) as i32;
                    let y = (test.scale * p[1]) as i32;
                    int_path.push(IntPoint::new(x, y));
                }
                outline_input.push(int_path);
            }

            self.workspace.outline_input = outline_input;
            self.workspace
                .outline_input
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
        let outline_input = &self.workspace.outline_input;
        let mut float_paths = Vec::with_capacity(outline_input.len());
        let scale = 1.0 / self.workspace.scale;
        for path in outline_input.iter() {
            let mut float_path = Vec::with_capacity(path.len());
            for p in path.iter() {
                let x = scale * p.x as f32;
                let y = scale * p.y as f32;
                float_path.push([x, y]);
            }
            float_paths.push(float_path);
        }

        let mut style = OutlineStyle::new(self.offset);
        match self.join {
            JoinOption::Miter => {
                let ratio = 0.03 * self.join_value as f32;
                style = style.line_join(LineJoin::Miter(ratio))
            },
            JoinOption::Round => {
                let ratio = 0.015 * self.join_value as f32;
                style = style.line_join(LineJoin::Round(ratio))
            }
            JoinOption::Bevel => style = style.line_join(LineJoin::Bevel),
        }

        /*
        let float_shapes = float_paths.outline(style, self.is_closed);

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

        self.workspace.outline_output = int_paths
        */
    }

    pub(super) fn outline_update_point(&mut self, update: PointEditUpdate) {
        self.workspace.points[update.index] = update.point.clone();
        let m_index = update.point.index;
        self.workspace.outline_input[m_index.path_index][m_index.point_index] = update.point.pos;
        self.update_solution();
    }
}
