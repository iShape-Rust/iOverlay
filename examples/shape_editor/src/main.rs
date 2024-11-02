mod square;
mod fill_view;
mod data;
mod app;
mod util;

use crate::app::main::EditorApp;


fn main() -> iced::Result {
    iced::run("iOverlay", EditorApp::update, EditorApp::view)
}