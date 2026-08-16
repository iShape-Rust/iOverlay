mod data;
mod draw;

#[path = "../../overlay_editor/src/geom/mod.rs"]
#[allow(dead_code)]
mod geom;
#[path = "../../overlay_editor/src/point_editor/mod.rs"]
#[allow(dead_code)]
mod point_editor;
#[path = "../../overlay_editor/src/sheet/mod.rs"]
mod sheet;

use crate::data::FixtureResource;
use crate::draw::LinesWidget;
use crate::geom::camera::Camera;
use crate::point_editor::point::{EditorPoint, MultiIndex};
use crate::point_editor::widget::{PointEditUpdate, PointsEditorWidget};
use crate::sheet::widget::SheetWidget;
use i_overlay::mesh::variable_stroke::{
    StrokeVertex, VariableStrokeDebug, VariableStrokeDebugEdgeKind, VariableStrokeStyle,
};
use i_triangle::i_overlay::i_float::int::point::IntPoint;
use i_triangle::i_overlay::i_float::int::rect::IntRect;
use i_triangle::i_overlay::i_shape::int::path::IntPaths;
use iced::keyboard::Key;
use iced::keyboard::key::Named;
use iced::widget::{
    Button, Column, Container, Row, Stack, Text, button, checkbox, container, scrollable, slider,
};
use iced::{Color, Element, Length, Size, Subscription, Task, Vector, application, keyboard};
use std::collections::HashMap;
use std::f32::consts::PI;
use std::path::PathBuf;

fn main() -> iced::Result {
    application(EditorApp::new, EditorApp::update, EditorApp::view)
        .title("iOverlay · Variable Stroke Debug")
        .resizable(true)
        .centered()
        .subscription(EditorApp::subscription)
        .run()
}

#[derive(Debug, Clone)]
struct VariablePoint {
    pos: IntPoint<i32>,
    width: f32,
}

#[derive(Debug, Clone, Copy)]
enum Layer {
    Input,
    RadiusGuides,
    RawEdges,
    Sections,
    Joins,
    Caps,
    Closures,
    Direction,
    FinalContour,
}

#[derive(Debug, Clone)]
enum Message {
    FixtureSelected(usize),
    VertexSelected(usize),
    WidthChanged(f32),
    RoundAngleChanged(f32),
    LayerToggled(Layer, bool),
    PointEdited(PointEditUpdate),
    WorkspaceSized(Size),
    WorkspaceZoomed(Camera),
    WorkspaceDragged(Vector<f32>),
    NextFixture,
    PreviousFixture,
}

struct LayerVisibility {
    input: bool,
    radius_guides: bool,
    raw_edges: bool,
    sections: bool,
    joins: bool,
    caps: bool,
    closures: bool,
    direction: bool,
    final_contour: bool,
}

impl Default for LayerVisibility {
    fn default() -> Self {
        Self {
            input: true,
            radius_guides: true,
            raw_edges: true,
            sections: true,
            joins: true,
            caps: true,
            closures: true,
            direction: true,
            final_contour: true,
        }
    }
}

#[derive(Default)]
struct EdgeCounts {
    sections: usize,
    joins: usize,
    caps: usize,
    closures: usize,
}

struct EditorApp {
    resource: FixtureResource,
    fixture_index: usize,
    fixture_name: String,
    scale: f32,
    variable_paths: Vec<Vec<VariablePoint>>,
    centerlines: IntPaths<i32>,
    editor_points: Vec<EditorPoint>,
    selected_vertex: usize,
    round_angle: f32,
    final_contours: IntPaths<i32>,
    radius_guides: IntPaths<i32>,
    section_edges: IntPaths<i32>,
    join_edges: IntPaths<i32>,
    cap_edges: IntPaths<i32>,
    closing_edges: IntPaths<i32>,
    counts: EdgeCounts,
    layers: LayerVisibility,
    camera: Camera,
    viewport_size: Size,
    cameras: HashMap<usize, Camera>,
    error: Option<String>,
}

impl EditorApp {
    fn new() -> (Self, Task<Message>) {
        let fixture_folder =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../tests/variable_stroke");
        let resource = FixtureResource::new(fixture_folder);
        let mut app = Self {
            resource,
            fixture_index: usize::MAX,
            fixture_name: String::new(),
            scale: 1.0,
            variable_paths: vec![],
            centerlines: vec![],
            editor_points: vec![],
            selected_vertex: 0,
            round_angle: 0.18,
            final_contours: vec![],
            radius_guides: vec![],
            section_edges: vec![],
            join_edges: vec![],
            cap_edges: vec![],
            closing_edges: vec![],
            counts: EdgeCounts::default(),
            layers: LayerVisibility::default(),
            camera: Camera::empty(),
            viewport_size: Size::ZERO,
            cameras: HashMap::new(),
            error: None,
        };
        if app.resource.len() > 0 {
            app.load_fixture(0);
        } else {
            app.error = Some("No variable_stroke JSON fixtures found".to_owned());
        }
        (app, Task::none())
    }

    fn subscription(&self) -> Subscription<Message> {
        keyboard::listen().filter_map(|event| match event {
            keyboard::Event::KeyPressed { key, .. } => match key {
                Key::Named(Named::ArrowDown) => Some(Message::NextFixture),
                Key::Named(Named::ArrowUp) => Some(Message::PreviousFixture),
                _ => None,
            },
            _ => None,
        })
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::FixtureSelected(index) => self.load_fixture(index),
            Message::VertexSelected(index) => self.selected_vertex = index,
            Message::WidthChanged(width) => {
                if let Some(index) = self
                    .editor_points
                    .get(self.selected_vertex)
                    .map(|p| p.index.clone())
                {
                    self.variable_paths[index.path_index][index.point_index].width = width;
                    self.rebuild();
                }
            }
            Message::RoundAngleChanged(angle) => {
                self.round_angle = angle;
                self.rebuild();
            }
            Message::LayerToggled(layer, visible) => match layer {
                Layer::Input => self.layers.input = visible,
                Layer::RadiusGuides => self.layers.radius_guides = visible,
                Layer::RawEdges => self.layers.raw_edges = visible,
                Layer::Sections => self.layers.sections = visible,
                Layer::Joins => self.layers.joins = visible,
                Layer::Caps => self.layers.caps = visible,
                Layer::Closures => self.layers.closures = visible,
                Layer::Direction => self.layers.direction = visible,
                Layer::FinalContour => self.layers.final_contour = visible,
            },
            Message::PointEdited(update) => {
                self.selected_vertex = update.index;
                let index = update.point.index.clone();
                self.editor_points[update.index] = update.point.clone();
                self.variable_paths[index.path_index][index.point_index].pos = update.point.pos;
                self.refresh_centerlines();
                self.rebuild();
            }
            Message::WorkspaceSized(size) => {
                self.viewport_size = size;
                if self.camera.is_empty() {
                    self.frame_input();
                } else {
                    self.camera.size = size;
                }
            }
            Message::WorkspaceZoomed(camera) => self.camera = camera,
            Message::WorkspaceDragged(position) => self.camera.pos = position,
            Message::NextFixture => {
                if self.fixture_index + 1 < self.resource.len() {
                    self.load_fixture(self.fixture_index + 1);
                }
            }
            Message::PreviousFixture => {
                if self.fixture_index > 0 && self.fixture_index != usize::MAX {
                    self.load_fixture(self.fixture_index - 1);
                }
            }
        }
        Task::none()
    }

    fn load_fixture(&mut self, index: usize) {
        match self.resource.load(index) {
            Ok(fixture) => {
                if self.fixture_index != usize::MAX {
                    self.cameras.insert(self.fixture_index, self.camera);
                }
                self.scale = fixture.scale;
                self.variable_paths = fixture
                    .stroke
                    .into_iter()
                    .map(|path| {
                        path.into_iter()
                            .map(|vertex| VariablePoint {
                                pos: self.to_int(vertex.point),
                                width: vertex.width,
                            })
                            .collect()
                    })
                    .collect();
                self.fixture_index = index;
                self.fixture_name = self.resource.name(index);
                self.selected_vertex = 0;
                self.refresh_centerlines();
                self.refresh_editor_points();
                self.camera = self
                    .cameras
                    .get(&index)
                    .copied()
                    .unwrap_or_else(Camera::empty);
                if self.camera.is_empty() && self.viewport_size.width > 0.0 {
                    self.frame_input();
                }
                self.error = None;
                self.rebuild();
            }
            Err(error) => self.error = Some(error),
        }
    }

    fn refresh_centerlines(&mut self) {
        self.centerlines = self
            .variable_paths
            .iter()
            .map(|path| path.iter().map(|vertex| vertex.pos).collect())
            .collect();
    }

    fn refresh_editor_points(&mut self) {
        self.editor_points.clear();
        for (path_index, path) in self.variable_paths.iter().enumerate() {
            for (point_index, vertex) in path.iter().enumerate() {
                self.editor_points.push(EditorPoint {
                    pos: vertex.pos,
                    index: MultiIndex {
                        point_index,
                        path_index,
                        group_index: 0,
                    },
                });
            }
        }
    }

    fn frame_input(&mut self) {
        if let Some(rect) = IntRect::with_iter(self.editor_points.iter().map(|point| &point.pos)) {
            self.camera = Camera::new(rect, self.viewport_size);
        }
    }

    fn rebuild(&mut self) {
        let inverse_scale = 1.0 / self.scale;
        let paths: Vec<Vec<_>> = self
            .variable_paths
            .iter()
            .map(|path| {
                path.iter()
                    .map(|vertex| {
                        StrokeVertex::new(
                            [
                                inverse_scale * vertex.pos.x as f32,
                                inverse_scale * vertex.pos.y as f32,
                            ],
                            vertex.width,
                        )
                    })
                    .collect()
            })
            .collect();
        let style = VariableStrokeStyle::new().round_angle(self.round_angle);
        let debug = paths.variable_stroke_debug(style);

        self.final_contours = debug
            .shapes
            .into_iter()
            .flatten()
            .map(|contour| {
                contour
                    .into_iter()
                    .map(|point| self.to_int(point))
                    .collect()
            })
            .collect();
        self.section_edges.clear();
        self.join_edges.clear();
        self.cap_edges.clear();
        self.closing_edges.clear();
        self.counts = EdgeCounts::default();

        for edge in debug.edges {
            let path = vec![self.to_int(edge.a), self.to_int(edge.b)];
            match edge.kind {
                VariableStrokeDebugEdgeKind::SectionBoundary => {
                    self.counts.sections += 1;
                    self.section_edges.push(path);
                }
                VariableStrokeDebugEdgeKind::JoinArc => {
                    self.counts.joins += 1;
                    self.join_edges.push(path);
                }
                VariableStrokeDebugEdgeKind::CapArc | VariableStrokeDebugEdgeKind::CircleArc => {
                    self.counts.caps += 1;
                    self.cap_edges.push(path);
                }
                VariableStrokeDebugEdgeKind::JoinClosure
                | VariableStrokeDebugEdgeKind::CapClosure => {
                    self.counts.closures += 1;
                    self.closing_edges.push(path);
                }
            }
        }
        self.radius_guides = radius_guides(&paths, self.scale);
    }

    fn to_int(&self, point: [f32; 2]) -> IntPoint<i32> {
        IntPoint::new(
            (self.scale * point[0]).round() as i32,
            (self.scale * point[1]).round() as i32,
        )
    }

    fn selected_width(&self) -> f32 {
        self.editor_points
            .get(self.selected_vertex)
            .map(|point| self.variable_paths[point.index.path_index][point.index.point_index].width)
            .unwrap_or(0.0)
    }

    fn view(&self) -> Element<'_, Message> {
        Row::new()
            .push(
                Container::new(scrollable(self.controls()).height(Length::Fill))
                    .width(Length::Fixed(285.0))
                    .height(Length::Fill)
                    .padding(12),
            )
            .push(self.workspace())
            .height(Length::Fill)
            .into()
    }

    fn controls(&self) -> Column<'_, Message> {
        let mut fixtures = Column::new().spacing(2);
        for index in 0..self.resource.len() {
            let selected = index == self.fixture_index;
            let button = Button::new(Text::new(self.resource.name(index)).size(14))
                .width(Length::Fill)
                .on_press(Message::FixtureSelected(index));
            fixtures = fixtures.push(if selected {
                button.style(button::primary)
            } else {
                button.style(button::text)
            });
        }

        let mut vertices = Column::new().spacing(2);
        for (index, point) in self.editor_points.iter().enumerate() {
            let vertex = &self.variable_paths[point.index.path_index][point.index.point_index];
            let label = format!(
                "path {} · v{}   width {:.2}",
                point.index.path_index, point.index.point_index, vertex.width
            );
            let button = Button::new(Text::new(label).size(13))
                .width(Length::Fill)
                .on_press(Message::VertexSelected(index));
            vertices = vertices.push(if index == self.selected_vertex {
                button.style(button::primary)
            } else {
                button.style(button::text)
            });
        }

        let total_edges =
            self.counts.sections + self.counts.joins + self.counts.caps + self.counts.closures;
        let stats = format!(
            "raw edges: {total_edges}\nsections {} · joins {}\ncaps {} · closing {}",
            self.counts.sections, self.counts.joins, self.counts.caps, self.counts.closures
        );

        let mut column = Column::new()
            .spacing(8)
            .push(Text::new("Variable Stroke Debug").size(22))
            .push(Text::new(format!("{}  ·  fixture {}/{}", self.fixture_name, self.fixture_index.saturating_add(1), self.resource.len())).size(13))
            .push(Text::new(stats).size(13))
            .push(Text::new("Fixtures").size(16))
            .push(fixtures)
            .push(Text::new("StrokeVertex width").size(16))
            .push(vertices)
            .push(Text::new(format!("selected width: {:.2}", self.selected_width())).size(13))
            .push(slider(0.0..=300.0, self.selected_width(), Message::WidthChanged).step(0.1_f32))
            .push(Text::new(format!("round_angle: {:.4} rad", self.round_angle)).size(13))
            .push(slider(0.01 * PI..=0.25 * PI, self.round_angle, Message::RoundAngleChanged).step(0.005_f32))
            .push(Text::new("Layers").size(16))
            .push(layer_checkbox("Input centerline / vertices", self.layers.input, Layer::Input))
            .push(layer_checkbox("Vertex radius guides", self.layers.radius_guides, Layer::RadiusGuides))
            .push(layer_checkbox("Raw added edges (master)", self.layers.raw_edges, Layer::RawEdges))
            .push(layer_checkbox("  Cyan · section boundaries", self.layers.sections, Layer::Sections))
            .push(layer_checkbox("  Orange · join arcs", self.layers.joins, Layer::Joins))
            .push(layer_checkbox("  Magenta · cap arcs", self.layers.caps, Layer::Caps))
            .push(layer_checkbox("  Yellow · closing edges", self.layers.closures, Layer::Closures))
            .push(layer_checkbox("Edge direction arrows", self.layers.direction, Layer::Direction))
            .push(layer_checkbox("Green · final contour", self.layers.final_contour, Layer::FinalContour))
            .push(Text::new("Drag diamonds to edit positions. Drag empty canvas to pan; wheel/trackpad zooms. ↑/↓ changes fixture.").size(12));

        if let Some(error) = &self.error {
            column = column.push(
                Text::new(error)
                    .color(Color::from_rgb8(255, 80, 80))
                    .size(13),
            );
        }
        column
    }

    fn workspace(&self) -> Container<'_, Message> {
        let mut stack = Stack::new().push(
            Container::new(SheetWidget::new(
                self.camera,
                Color::from_rgb8(130, 130, 130).scale_alpha(0.35),
                Message::WorkspaceSized,
                Message::WorkspaceZoomed,
                Message::WorkspaceDragged,
            ))
            .width(Length::Fill)
            .height(Length::Fill),
        );

        if self.camera.is_not_empty() {
            if self.layers.final_contour && !self.final_contours.is_empty() {
                stack = stack.push(full_layer(LinesWidget::new(
                    &self.final_contours,
                    self.camera,
                    Color::from_rgb8(45, 210, 90),
                    2.5,
                    false,
                    true,
                )));
            }
            if self.layers.input && self.layers.radius_guides {
                stack = stack.push(full_layer(LinesWidget::new(
                    &self.radius_guides,
                    self.camera,
                    Color::from_rgb8(170, 170, 180).scale_alpha(0.65),
                    1.0,
                    false,
                    false,
                )));
            }
            if self.layers.raw_edges {
                if self.layers.sections {
                    stack = stack.push(full_layer(LinesWidget::new(
                        &self.section_edges,
                        self.camera,
                        Color::from_rgb8(45, 200, 255),
                        1.7,
                        self.layers.direction,
                        false,
                    )));
                }
                if self.layers.joins {
                    stack = stack.push(full_layer(LinesWidget::new(
                        &self.join_edges,
                        self.camera,
                        Color::from_rgb8(255, 145, 40),
                        2.0,
                        self.layers.direction,
                        false,
                    )));
                }
                if self.layers.caps {
                    stack = stack.push(full_layer(LinesWidget::new(
                        &self.cap_edges,
                        self.camera,
                        Color::from_rgb8(220, 80, 255),
                        2.0,
                        self.layers.direction,
                        false,
                    )));
                }
                if self.layers.closures {
                    stack = stack.push(full_layer(LinesWidget::new(
                        &self.closing_edges,
                        self.camera,
                        Color::from_rgb8(255, 220, 40),
                        2.2,
                        self.layers.direction,
                        false,
                    )));
                }
            }
            if self.layers.input {
                stack = stack.push(full_layer(LinesWidget::new(
                    &self.centerlines,
                    self.camera,
                    Color::from_rgb8(255, 70, 70),
                    1.5,
                    false,
                    false,
                )));
                stack = stack.push(full_layer(
                    PointsEditorWidget::new(&self.editor_points, self.camera, Message::PointEdited)
                        .set_drag_color(Color::from_rgb8(255, 145, 40))
                        .set_hover_color(Color::WHITE),
                ));
            }
        }

        Container::new(stack)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(container::dark)
    }
}

fn layer_checkbox(label: &str, value: bool, layer: Layer) -> iced::widget::Checkbox<'_, Message> {
    checkbox(value)
        .label(label)
        .on_toggle(move |visible| Message::LayerToggled(layer, visible))
}

fn full_layer<'a>(widget: impl Into<Element<'a, Message>>) -> Container<'a, Message> {
    Container::new(widget)
        .width(Length::Fill)
        .height(Length::Fill)
}

fn radius_guides(paths: &[Vec<StrokeVertex<[f32; 2]>>], scale: f32) -> IntPaths<i32> {
    const STEPS: usize = 48;
    let mut guides = Vec::new();
    for path in paths {
        for vertex in path {
            let radius = 0.5 * vertex.width.max(0.0);
            if radius <= 0.0 {
                continue;
            }
            let mut circle = Vec::with_capacity(STEPS + 1);
            for step in 0..=STEPS {
                let angle = 2.0 * PI * step as f32 / STEPS as f32;
                circle.push(IntPoint::new(
                    (scale * (vertex.point[0] + radius * angle.cos())).round() as i32,
                    (scale * (vertex.point[1] + radius * angle.sin())).round() as i32,
                ));
            }
            guides.push(circle);
        }
    }
    guides
}
