use crate::app::design;
use crate::app::main::{AppMessage, EditorApp};
use crate::app::variable_stroke::workspace::WorkspaceState;
use crate::geom::camera::Camera;
use crate::point_editor::point::PathsToEditorPoints;
use crate::point_editor::widget::PointEditUpdate;
use i_triangle::i_overlay::i_float::int::point::IntPoint;
use i_triangle::i_overlay::i_float::int::rect::IntRect;
use i_triangle::i_overlay::mesh::variable_stroke::offset::VariableStrokeOffset;
use i_triangle::i_overlay::mesh::variable_stroke::{StrokeVertex, VariableStrokeStyle};
use iced::widget::{scrollable, Button, Column, Container, Row, Space, Text};
use iced::{Alignment, Length, Padding, Size, Vector};
use std::collections::HashMap;
use std::fmt::Write;

#[derive(Debug, Clone)]
pub(crate) struct VariableStrokePoint {
    pub(crate) pos: IntPoint<i32>,
    pub(crate) width: f32,
}

#[derive(Clone)]
struct VariableStrokeExample {
    title: &'static str,
    scale: f32,
    is_closed: bool,
    paths: Vec<Vec<([f32; 2], f32)>>,
}

pub(crate) struct VariableStrokeState {
    pub(crate) test: usize,
    pub(crate) width_scale: f32,
    pub(crate) round_angle: u8,
    pub(crate) is_closed: bool,
    pub(crate) workspace: WorkspaceState,
    pub(crate) size: Size,
    pub(crate) cameras: HashMap<usize, Camera>,
    examples: Vec<VariableStrokeExample>,
}

#[derive(Debug, Clone)]
pub(crate) enum VariableStrokeMessage {
    TestSelected(usize),
    WidthScaleUpdated(f32),
    RoundAngleUpdated(u8),
    IsClosedUpdated(bool),
    PointEdited(PointEditUpdate),
    WorkspaceSized(Size),
    WorkspaceZoomed(Camera),
    WorkspaceDragged(Vector<f32>),
}

impl EditorApp {
    fn variable_stroke_sidebar(&self) -> Column<'_, AppMessage> {
        let mut column =
            Column::new().push(Space::new().width(Length::Fill).height(Length::Fixed(2.0)));
        for (index, example) in self.state.variable_stroke.examples.iter().enumerate() {
            let is_selected = self.state.variable_stroke.test == index;
            column = column.push(
                Container::new(
                    Button::new(
                        Text::new(example.title)
                            .style(if is_selected {
                                design::style_sidebar_text_selected
                            } else {
                                design::style_sidebar_text
                            })
                            .size(14),
                    )
                    .width(Length::Fill)
                    .on_press(AppMessage::VariableStroke(
                        VariableStrokeMessage::TestSelected(index),
                    ))
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

    pub(crate) fn variable_stroke_content(&self) -> Row<'_, AppMessage> {
        Row::new()
            .push(
                scrollable(
                    Container::new(self.variable_stroke_sidebar())
                        .width(Length::Fixed(180.0))
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
            .push(self.variable_stroke_workspace())
    }

    pub(crate) fn variable_stroke_update(&mut self, message: VariableStrokeMessage) {
        match message {
            VariableStrokeMessage::TestSelected(index) => self.variable_stroke_set_test(index),
            VariableStrokeMessage::WidthScaleUpdated(value) => {
                self.variable_stroke_update_width_scale(value)
            }
            VariableStrokeMessage::RoundAngleUpdated(value) => {
                self.variable_stroke_update_round_angle(value)
            }
            VariableStrokeMessage::IsClosedUpdated(is_closed) => {
                self.variable_stroke_update_is_closed(is_closed)
            }
            VariableStrokeMessage::PointEdited(update) => self.variable_stroke_update_point(update),
            VariableStrokeMessage::WorkspaceSized(size) => self.variable_stroke_update_size(size),
            VariableStrokeMessage::WorkspaceZoomed(zoom) => self.variable_stroke_update_zoom(zoom),
            VariableStrokeMessage::WorkspaceDragged(drag) => self.variable_stroke_update_drag(drag),
        }
    }

    fn variable_stroke_set_test(&mut self, index: usize) {
        self.state.variable_stroke.set_test(index);
        self.state.variable_stroke.update_solution();
    }

    pub(crate) fn variable_stroke_init(&mut self) {
        self.variable_stroke_set_test(self.state.variable_stroke.test);
    }

    pub(crate) fn variable_stroke_next_test(&mut self) {
        let next_test = self.state.variable_stroke.test + 1;
        if next_test < self.state.variable_stroke.examples.len() {
            self.variable_stroke_set_test(next_test);
        }
    }

    pub(crate) fn variable_stroke_prev_test(&mut self) {
        let test = self.state.variable_stroke.test;
        if test >= 1 {
            self.variable_stroke_set_test(test - 1);
        }
    }

    fn variable_stroke_update_size(&mut self, size: Size) {
        self.state.variable_stroke.size = size;
        let points = &self.state.variable_stroke.workspace.points;
        if self.state.variable_stroke.workspace.camera.is_empty() && !points.is_empty() {
            let rect = IntRect::with_iter(points.iter().map(|p| &p.pos))
                .unwrap_or(IntRect::new(-10_000, 10_000, -10_000, 10_000));
            let camera = Camera::new(rect, size);
            self.state.variable_stroke.workspace.camera = camera;
        } else {
            self.state.variable_stroke.workspace.camera.size = size;
        }
    }

    fn variable_stroke_update_width_scale(&mut self, width_scale: f32) {
        self.state.variable_stroke.width_scale = width_scale;
        self.state.variable_stroke.update_solution();
    }

    fn variable_stroke_update_round_angle(&mut self, value: u8) {
        self.state.variable_stroke.round_angle = value;
        self.state.variable_stroke.update_solution();
    }

    fn variable_stroke_update_is_closed(&mut self, is_closed: bool) {
        self.state.variable_stroke.is_closed = is_closed;
        self.state.variable_stroke.update_solution();
    }
}

impl VariableStrokeState {
    pub(crate) fn new() -> Self {
        let examples = examples();
        let mut state = VariableStrokeState {
            test: usize::MAX,
            width_scale: 1.0,
            round_angle: 12,
            is_closed: false,
            workspace: Default::default(),
            cameras: HashMap::with_capacity(examples.len()),
            size: Size::ZERO,
            examples,
        };

        state.set_test(0);
        state.update_solution();
        state
    }

    fn set_test(&mut self, index: usize) {
        let Some(example) = self.examples.get(index).cloned() else {
            return;
        };

        self.workspace.scale = example.scale;
        self.is_closed = example.is_closed;
        self.cameras.insert(self.test, self.workspace.camera);

        let editor_points = &mut self.workspace.points;
        editor_points.clear();

        let mut variable_input = Vec::with_capacity(example.paths.len());
        let mut centerline_input = Vec::with_capacity(example.paths.len());
        for path in example.paths.iter() {
            let mut variable_path = Vec::with_capacity(path.len());
            let mut centerline_path = Vec::with_capacity(path.len());
            for &(p, width) in path.iter() {
                let x = (example.scale * p[0]) as i32;
                let y = (example.scale * p[1]) as i32;
                let pos = IntPoint::new(x, y);
                variable_path.push(VariableStrokePoint { pos, width });
                centerline_path.push(pos);
            }
            variable_input.push(variable_path);
            centerline_input.push(centerline_path);
        }

        self.workspace.variable_input = variable_input;
        self.workspace.centerline_input = centerline_input;
        self.workspace
            .centerline_input
            .feed_edit_points(0, editor_points);

        let mut camera = *self.cameras.get(&index).unwrap_or(&Camera::empty());
        if camera.is_empty() && self.size.width > 0.001 {
            let rect = IntRect::with_iter(editor_points.iter().map(|p| &p.pos))
                .unwrap_or(IntRect::new(-10_000, 10_000, -10_000, 10_000));
            camera = Camera::new(rect, self.size);
        }

        self.workspace.camera = camera;
        self.test = index;
    }

    fn update_solution(&mut self) {
        let scale = 1.0 / self.workspace.scale;
        let mut float_paths = Vec::with_capacity(self.workspace.variable_input.len());
        for path in self.workspace.variable_input.iter() {
            let mut float_path = Vec::with_capacity(path.len());
            for p in path.iter() {
                let x = scale * p.pos.x as f32;
                let y = scale * p.pos.y as f32;
                let width = self.width_scale * p.width;
                float_path.push(StrokeVertex::new([x, y], width));
            }
            float_paths.push(float_path);
        }

        let round_angle = 0.015 * self.round_angle as f32;
        self.print_repro(&float_paths, round_angle);

        let style = VariableStrokeStyle::new().round_angle(round_angle);

        let float_shapes = float_paths.variable_stroke(style, self.is_closed);

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

    fn print_repro(&self, paths: &[Vec<StrokeVertex<[f32; 2]>>], round_angle: f32) {
        let title = self
            .examples
            .get(self.test)
            .map(|example| example.title)
            .unwrap_or("Custom");
        let mut dump = String::new();
        let _ = writeln!(
            dump,
            "\n// Dynamic Width repro: test={} title={title:?} width_scale={:?}",
            self.test, self.width_scale
        );
        let _ = writeln!(dump, "let paths = vec![");
        for path in paths {
            let _ = writeln!(dump, "    vec![");
            for vertex in path {
                let _ = writeln!(
                    dump,
                    "        StrokeVertex::new([{:?}_f32, {:?}_f32], {:?}_f32),",
                    vertex.point[0], vertex.point[1], vertex.width
                );
            }
            let _ = writeln!(dump, "    ],");
        }
        let _ = writeln!(dump, "];");
        let _ = writeln!(
            dump,
            "let style = VariableStrokeStyle::new().round_angle({round_angle:?}_f32);"
        );
        let _ = writeln!(
            dump,
            "let result = paths.variable_stroke(style, {});",
            self.is_closed
        );

        #[cfg(not(target_arch = "wasm32"))]
        println!("{dump}");

        #[cfg(target_arch = "wasm32")]
        log::info!("{dump}");
    }

    pub(super) fn variable_stroke_update_point(&mut self, update: PointEditUpdate) {
        self.workspace.points[update.index] = update.point.clone();
        let m_index = update.point.index;
        self.workspace.centerline_input[m_index.path_index][m_index.point_index] = update.point.pos;
        self.workspace.variable_input[m_index.path_index][m_index.point_index].pos =
            update.point.pos;
        self.update_solution();
    }
}

fn examples() -> Vec<VariableStrokeExample> {
    vec![
        VariableStrokeExample {
            title: "Equal width",
            scale: 100.0,
            is_closed: false,
            paths: vec![vec![
                ([0.0, 0.0], 12.0),
                ([45.0, -15.0], 12.0),
                ([85.0, 20.0], 12.0),
                ([130.0, 0.0], 12.0),
            ]],
        },
        VariableStrokeExample {
            title: "Open taper",
            scale: 100.0,
            is_closed: false,
            paths: vec![vec![
                ([0.0, 0.0], 2.0),
                ([35.0, -10.0], 6.0),
                ([70.0, 0.0], 14.0),
                ([105.0, 25.0], 5.0),
                ([140.0, 15.0], 10.0),
            ]],
        },
        VariableStrokeExample {
            title: "Closed loop",
            scale: 100.0,
            is_closed: true,
            paths: vec![vec![
                ([0.0, 0.0], 8.0),
                ([40.0, -25.0], 18.0),
                ([85.0, 5.0], 6.0),
                ([55.0, 50.0], 22.0),
                ([5.0, 40.0], 10.0),
            ]],
        },
        VariableStrokeExample {
            title: "Sharp joins",
            scale: 100.0,
            is_closed: false,
            paths: vec![vec![
                ([0.0, 0.0], 6.0),
                ([35.0, 35.0], 18.0),
                ([70.0, -20.0], 4.0),
                ([105.0, 35.0], 20.0),
                ([140.0, 0.0], 8.0),
            ]],
        },
        VariableStrokeExample {
            title: "Two paths",
            scale: 100.0,
            is_closed: false,
            paths: vec![
                vec![
                    ([0.0, 0.0], 5.0),
                    ([35.0, -25.0], 15.0),
                    ([75.0, -5.0], 7.0),
                    ([110.0, -30.0], 18.0),
                ],
                vec![
                    ([0.0, 35.0], 18.0),
                    ([35.0, 15.0], 6.0),
                    ([75.0, 45.0], 14.0),
                    ([110.0, 20.0], 4.0),
                ],
            ],
        },
        VariableStrokeExample {
            title: "Coverage at limit",
            scale: 100.0,
            is_closed: false,
            paths: vec![vec![([0.0, 0.0], 4.0), ([20.0, 0.0], 44.0)]],
        },
        VariableStrokeExample {
            title: "Coverage beyond limit",
            scale: 100.0,
            is_closed: false,
            paths: vec![vec![
                ([0.0, 0.0], 6.0),
                ([30.0, 0.0], 80.0),
                ([65.0, 20.0], 12.0),
                ([105.0, 0.0], 24.0),
            ]],
        },
        VariableStrokeExample {
            title: "Reverse coverage",
            scale: 100.0,
            is_closed: false,
            paths: vec![vec![
                ([0.0, 0.0], 80.0),
                ([30.0, 0.0], 6.0),
                ([65.0, -20.0], 18.0),
                ([105.0, 0.0], 8.0),
            ]],
        },
        VariableStrokeExample {
            title: "Zero tip",
            scale: 100.0,
            is_closed: false,
            paths: vec![vec![
                ([0.0, 0.0], 0.0),
                ([45.0, -10.0], 18.0),
                ([90.0, 15.0], 4.0),
                ([135.0, 0.0], 0.0),
            ]],
        },
        VariableStrokeExample {
            title: "U-turn break",
            scale: 100.0,
            is_closed: false,
            paths: vec![vec![
                ([0.0, 0.0], 8.0),
                ([60.0, 0.0], 20.0),
                ([5.0, 0.0], 10.0),
                ([65.0, 20.0], 16.0),
            ]],
        },
    ]
}
