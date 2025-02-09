mod data;
mod app;
mod draw;
mod point_editor;
mod sheet;
mod geom;

use std::fs;
use std::path::Path;
use iced::application;
use crate::app::main::EditorApp;
use crate::data::resource::AppResource;

fn main() -> iced::Result {
    let app_resource = AppResource::with_paths(
        "../../tests/boolean",
        "../../tests/string",
        "../../tests/stroke"
    );

    // let boolean_file_path = Path::new("../../tests_boolean.json");
    // let string_file_path = Path::new("../../tests_string.json");
    // let boolean_data = fs::read_to_string(boolean_file_path)
    //     .expect("Failed to read boolean JSON file");
    // let string_data = fs::read_to_string(string_file_path)
    //     .expect("Failed to read string JSON file");
    // let app_resource = AppResource::with_content(boolean_data, string_data);

    let app_initializer = || {
        let app = EditorApp::new(app_resource);
        (app, iced::Task::none())
    };

    application("iOverlay", EditorApp::update, EditorApp::view)
        .subscription(EditorApp::subscription)
        .run_with(app_initializer)
}