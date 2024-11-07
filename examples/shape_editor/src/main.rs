mod square;
mod data;
mod app;
mod shape;
mod point_editor;
mod sheet;
mod geom;

use iced::application;
use crate::app::main::EditorApp;


fn main() -> iced::Result {
    application("iOverlay", EditorApp::update, EditorApp::view)
        .subscription(EditorApp::subscription)
        .run()
}