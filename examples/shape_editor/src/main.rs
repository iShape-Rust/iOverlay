mod square;
mod fill_view;
mod data;
mod app;


use crate::app::main::EditorApp;
use iced::widget::{Row, Container, Column, Text, Button, button};
use iced::{Alignment, Color, Element, Length, Padding};
use crate::fill_view::FillView;


fn main() -> iced::Result {
    iced::run("iOverlay", EditorApp::update, EditorApp::view)
}