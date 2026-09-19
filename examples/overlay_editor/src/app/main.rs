use crate::app::boolean::content::BooleanMessage;
use crate::app::boolean::content::BooleanState;
use crate::app::outline::content::OutlineMessage;
use crate::app::outline::content::OutlineState;
use crate::app::string::content::StringMessage;
use crate::app::string::content::StringState;
use crate::app::stroke::content::StrokeMessage;
use crate::app::stroke::content::StrokeState;
use crate::app::variable_stroke::content::{VariableStrokeMessage, VariableStrokeState};
use eframe::egui;

use crate::data::resource::AppResource;

pub struct EditorApp {
    main_actions: Vec<MainAction>,
    pub(super) state: MainState,
    pub(super) app_resource: AppResource,
}

pub(super) struct MainState {
    selected_action: MainAction,
    pub(super) boolean: BooleanState,
    pub(super) string: StringState,
    pub(super) stroke: StrokeState,
    pub(super) variable_stroke: VariableStrokeState,
    pub(super) outline: OutlineState,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum MainAction {
    Boolean,
    String,
    Stroke,
    VariableStroke,
    Outline,
}

impl MainAction {
    fn title(&self) -> &str {
        match self {
            MainAction::Boolean => "Boolean",
            MainAction::String => "String",
            MainAction::Stroke => "Stroke",
            MainAction::VariableStroke => "Variable Stroke",
            MainAction::Outline => "Outline",
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum MainMessage {
    ActionSelected(MainAction),
}

#[derive(Debug, Clone)]
pub(crate) enum AppMessage {
    Main(MainMessage),
    Bool(BooleanMessage),
    String(StringMessage),
    Stroke(StrokeMessage),
    VariableStroke(VariableStrokeMessage),
    Outline(OutlineMessage),
    NextTest,
    PrevTest,
}

impl EditorApp {
    pub fn with_resource(mut app_resource: AppResource) -> Self {
        Self {
            main_actions: vec![
                MainAction::Boolean,
                MainAction::String,
                MainAction::Stroke,
                MainAction::VariableStroke,
                MainAction::Outline,
            ],
            state: MainState {
                selected_action: MainAction::Boolean,
                boolean: BooleanState::new(&mut app_resource.boolean),
                string: StringState::new(&mut app_resource.string),
                stroke: StrokeState::new(&mut app_resource.stroke),
                variable_stroke: VariableStrokeState::new(&mut app_resource.variable_stroke),
                outline: OutlineState::new(&mut app_resource.outline),
            },
            app_resource,
        }
    }
}

impl EditorApp {
    pub(crate) fn update(&mut self, message: AppMessage) {
        match message {
            AppMessage::Main(msg) => self.update_main(msg),
            AppMessage::Bool(msg) => self.boolean_update(msg),
            AppMessage::String(msg) => self.string_update(msg),
            AppMessage::Stroke(msg) => self.stroke_update(msg),
            AppMessage::VariableStroke(msg) => self.variable_stroke_update(msg),
            AppMessage::Outline(msg) => self.outline_update(msg),
            AppMessage::NextTest => match self.state.selected_action {
                MainAction::Boolean => self.boolean_next_test(),
                MainAction::String => self.string_next_test(),
                MainAction::Stroke => self.stroke_next_test(),
                MainAction::VariableStroke => self.variable_stroke_next_test(),
                MainAction::Outline => self.outline_next_test(),
            },
            AppMessage::PrevTest => match self.state.selected_action {
                MainAction::Boolean => self.boolean_prev_test(),
                MainAction::String => self.string_prev_test(),
                MainAction::Stroke => self.stroke_prev_test(),
                MainAction::VariableStroke => self.variable_stroke_prev_test(),
                MainAction::Outline => self.outline_prev_test(),
            },
        }
    }

    fn update_main(&mut self, message: MainMessage) {
        match message {
            MainMessage::ActionSelected(action) => {
                self.state.selected_action = action;
                match self.state.selected_action {
                    MainAction::Boolean => self.boolean_init(),
                    MainAction::String => self.string_init(),
                    MainAction::Stroke => self.stroke_init(),
                    MainAction::VariableStroke => self.variable_stroke_init(),
                    MainAction::Outline => self.outline_init(),
                }
            }
        }
    }

    pub(crate) fn view(&mut self, ui: &mut egui::Ui) {
        if !ui.ctx().egui_wants_keyboard_input() {
            if ui.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                self.update(AppMessage::NextTest);
            }
            if ui.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                self.update(AppMessage::PrevTest);
            }
        }
        egui::Panel::left("navigation")
            .exact_size(150.0)
            .resizable(false)
            .show_inside(ui, |ui| {
                for action in self.main_actions.clone() {
                    let response =
                        ui.selectable_label(self.state.selected_action == action, action.title());
                    if response.clicked() {
                        response.surrender_focus();
                        self.update(AppMessage::Main(MainMessage::ActionSelected(action)));
                    }
                }
            });
        match self.state.selected_action {
            MainAction::Boolean => self.boolean_content(ui),
            MainAction::String => self.string_content(ui),
            MainAction::Stroke => self.stroke_content(ui),
            MainAction::VariableStroke => self.variable_stroke_content(ui),
            MainAction::Outline => self.outline_content(ui),
        }
    }
}

impl eframe::App for EditorApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.view(ui);
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use eframe::egui::{self, RawInput, Rect, Vec2};

    fn editor() -> EditorApp {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../tests");
        let path = |name| root.join(name).to_str().unwrap().to_owned();
        EditorApp::with_resource(AppResource::with_paths(
            &path("boolean"),
            &path("string"),
            &path("stroke"),
            &path("variable_stroke"),
            &path("outline"),
        ))
    }

    fn render(app: &mut EditorApp, ctx: &egui::Context) {
        let output = ctx.run_ui(
            RawInput {
                screen_rect: Some(Rect::from_min_size(
                    egui::Pos2::ZERO,
                    Vec2::new(1280.0, 800.0),
                )),
                ..Default::default()
            },
            |ui| app.view(ui),
        );
        let primitives = ctx.tessellate(output.shapes, output.pixels_per_point);
        assert!(!primitives.is_empty());
        for primitive in primitives {
            if let egui::epaint::Primitive::Mesh(mesh) = primitive.primitive {
                assert!(mesh.is_valid());
                assert!(
                    mesh.vertices.iter().all(|vertex| vertex.pos.is_finite()),
                    "invalid mesh in {:?}, tests: {}, {}, {}, {}, {}; bad vertices: {:?}",
                    app.state.selected_action,
                    app.state.boolean.test,
                    app.state.string.test,
                    app.state.stroke.test,
                    app.state.variable_stroke.test,
                    app.state.outline.test,
                    mesh.vertices
                        .iter()
                        .filter(|v| !v.pos.is_finite())
                        .take(4)
                        .collect::<Vec<_>>()
                );
            }
        }
    }

    #[test]
    fn all_fixture_workspaces_render() {
        let mut app = editor();
        let ctx = egui::Context::default();
        for action in app.main_actions.clone() {
            app.update(AppMessage::Main(MainMessage::ActionSelected(
                action.clone(),
            )));
            let count = match action {
                MainAction::Boolean => app.app_resource.boolean.count,
                MainAction::String => app.app_resource.string.count,
                MainAction::Stroke => app.app_resource.stroke.count,
                MainAction::VariableStroke => app.app_resource.variable_stroke.count,
                MainAction::Outline => app.app_resource.outline.count,
            };
            assert!(count > 0);
            for index in 0..count {
                let message = match action {
                    MainAction::Boolean => AppMessage::Bool(BooleanMessage::TestSelected(index)),
                    MainAction::String => AppMessage::String(StringMessage::TestSelected(index)),
                    MainAction::Stroke => AppMessage::Stroke(StrokeMessage::TestSelected(index)),
                    MainAction::VariableStroke => {
                        AppMessage::VariableStroke(VariableStrokeMessage::TestSelected(index))
                    }
                    MainAction::Outline => AppMessage::Outline(OutlineMessage::TestSelected(index)),
                };
                app.update(message);
                render(&mut app, &ctx);
            }
        }
    }

    #[test]
    fn boolean_and_string_render_modes() {
        use crate::app::{
            boolean::control::ModeOption as BooleanMode, string::control::ModeOption as StringMode,
        };
        let mut app = editor();
        let ctx = egui::Context::default();
        for mode in [
            BooleanMode::Edit,
            BooleanMode::Debug,
            BooleanMode::Subject,
            BooleanMode::Clip,
            BooleanMode::Intersect,
            BooleanMode::Union,
            BooleanMode::Difference,
            BooleanMode::InverseDifference,
            BooleanMode::Xor,
        ] {
            app.update(AppMessage::Bool(BooleanMessage::ModeSelected(mode)));
            render(&mut app, &ctx);
        }
        app.update(AppMessage::Main(MainMessage::ActionSelected(
            MainAction::String,
        )));
        for mode in [
            StringMode::Edit,
            StringMode::Debug,
            StringMode::Slice,
            StringMode::ClipDirect,
            StringMode::ClipInvert,
        ] {
            app.update(AppMessage::String(StringMessage::ModeSelected(mode)));
            render(&mut app, &ctx);
        }
    }
    #[test]
    fn arrow_navigation_works_after_canvas_and_sidebar_clicks() {
        let mut app = editor();
        let ctx = egui::Context::default();
        let mut frame = |events| {
            let _ = ctx.run_ui(
                RawInput {
                    screen_rect: Some(Rect::from_min_size(
                        egui::Pos2::ZERO,
                        Vec2::new(1280.0, 800.0),
                    )),
                    events,
                    ..Default::default()
                },
                |ui| app.view(ui),
            );
            app.state.boolean.test
        };
        frame(vec![]);
        for pos in [egui::pos2(1100.0, 700.0), egui::pos2(170.0, 13.0)] {
            frame(vec![egui::Event::PointerMoved(pos)]);
            frame(vec![egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: Default::default(),
            }]);
            let before = frame(vec![egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: Default::default(),
            }]);
            let after = frame(vec![egui::Event::Key {
                key: egui::Key::ArrowDown,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: Default::default(),
            }]);
            assert_eq!(after, before + 1);
            frame(vec![egui::Event::Key {
                key: egui::Key::ArrowDown,
                physical_key: None,
                pressed: false,
                repeat: false,
                modifiers: Default::default(),
            }]);
        }
    }
}
