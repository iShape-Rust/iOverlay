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



#[cfg(not(target_arch = "wasm32"))]
fn run_desktop() -> iced::Result {

    let app_initializer = move || {
        let app_resource = AppResource::with_paths(
            "../tests/boolean",
            "../tests/string",
            "../tests/stroke",
            "../tests/outline"
        );
        let app = EditorApp::with_resource(app_resource);
        (app, iced::Task::none())
    };

    application(app_initializer, EditorApp::update, EditorApp::view)
        .resizable(true)
        .centered()
        .title("iOverlay Editor")
        .subscription(EditorApp::subscription)
        .run()
}

