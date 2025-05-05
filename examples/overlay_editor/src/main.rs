mod data;
mod app;
mod draw;
mod point_editor;
mod sheet;
mod geom;

use iced::{application, Font};
use crate::app::main::EditorApp;
use crate::data::resource::AppResource;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> iced::Result {
    run_desktop()
}

const ROBOTO_REGULAR: &[u8] = include_bytes!("assets/fonts/Roboto-Regular.ttf");

#[cfg(not(target_arch = "wasm32"))]
fn run_desktop() -> iced::Result {
    let _ = iced::font::load(ROBOTO_REGULAR);
    let custom_font = Font {
        family: iced::font::Family::Name("Roboto"),
        ..Font::default()
    };

    let app_initializer = move || {
        let app_resource = AppResource::with_paths(
            "../tests/boolean",
            "../tests/string",
            "../tests/stroke",
            "../tests/outline"
        );
        let app = EditorApp::with_resource(app_resource, custom_font);
        (app, iced::Task::none())
    };

    application(app_initializer, EditorApp::update, EditorApp::view)
        .resizable(true)
        .centered()
        .default_font(custom_font)
        .title("iOverlay Editor")
        .run()
}

