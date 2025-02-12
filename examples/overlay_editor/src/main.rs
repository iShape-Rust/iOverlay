mod data;
mod app;
mod draw;
mod point_editor;
mod sheet;
mod geom;

use iced::application;
use crate::app::main::EditorApp;
use crate::data::resource::AppResource;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> iced::Result {
    run_desktop()
}

#[cfg(target_arch = "wasm32")]
fn main() {
    run_wasm();
}

#[cfg(not(target_arch = "wasm32"))]
fn run_desktop() -> iced::Result {

    let app_resource = AppResource::with_paths(
        "../../tests/boolean",
        "../../tests/string",
        "../../tests/stroke",
        "../../tests/outline"
    );

    let app_initializer = || {
        let app = EditorApp::new(app_resource);
        (app, iced::Task::none())
    };

    application("iOverlay", EditorApp::update, EditorApp::view)
        .subscription(EditorApp::subscription)
        .run_with(app_initializer)
}

#[cfg(target_arch = "wasm32")]
fn run_wasm() {
    use std::fs;
    use std::path::Path;

    let boolean_file_path = Path::new("../../tests_boolean.json");
    let string_file_path = Path::new("../../tests_string.json");
    let stroke_file_path = Path::new("../../tests_stroke.json");
    let outline_file_path = Path::new("../../tests_outline.json");

    let boolean_data = fs::read_to_string(boolean_file_path).expect("Failed to read boolean JSON file");
    let string_data = fs::read_to_string(string_file_path).expect("Failed to read string JSON file");
    let stroke_data = fs::read_to_string(stroke_file_path).expect("Failed to read string JSON file");
    let outline_data = fs::read_to_string(outline_file_path).expect("Failed to read string JSON file");

    let app_resource = AppResource::with_content(boolean_data, string_data, stroke_data, outline_data);

    let app_initializer = || {
        let app = EditorApp::new(app_resource);
        (app, iced::Task::none())
    };

    application("iOverlay", EditorApp::update, EditorApp::view)
        .subscription(EditorApp::subscription)
        .run_with(app_initializer)
}
