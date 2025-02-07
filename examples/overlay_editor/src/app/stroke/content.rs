use std::collections::HashMap;
use iced::Size;
use crate::app::fill_option::FillOption;
use crate::app::solver_option::SolverOption;
use crate::geom::camera::Camera;

pub(crate) struct StringState {
    pub(crate) test: usize,
    pub(crate) fill: FillOption,
    pub(crate) mode: crate::app::string::control::ModeOption,
    pub(crate) solver: SolverOption,
    pub(crate) workspace: crate::app::string::workspace::WorkspaceState,
    pub(crate) size: Size,
    pub(crate) cameras: HashMap<usize, Camera>,
}