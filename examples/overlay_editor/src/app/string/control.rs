use crate::app::design::{controls, select};
use crate::app::main::{AppMessage, EditorApp};
use crate::app::string::content::StringMessage;
use crate::app::{fill_option::FillOption, solver_option::SolverOption};
use eframe::egui;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum ModeOption {
    #[default]
    Edit,
    Debug,
    Slice,
    ClipDirect,
    ClipInvert,
}

impl ModeOption {
    const ALL: [ModeOption; 5] = [
        ModeOption::Edit,
        ModeOption::Debug,
        ModeOption::Slice,
        ModeOption::ClipDirect,
        ModeOption::ClipInvert,
    ];
}

impl std::fmt::Display for ModeOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ModeOption::Edit => "Edit",
                ModeOption::Debug => "Debug",
                ModeOption::Slice => "Slice",
                ModeOption::ClipDirect => "ClipDirect",
                ModeOption::ClipInvert => "ClipInvert",
            }
        )
    }
}

impl EditorApp {
    pub(crate) fn string_control(&mut self, ui: &mut egui::Ui) {
        controls(ui, "string_controls", |ui| {
            let mut solver = self.state.string.solver;
            if select(ui, "Solver", &mut solver, &SolverOption::ALL) {
                self.update(AppMessage::String(StringMessage::SolverSelected(solver)));
            }
            let mut fill = self.state.string.fill;
            if select(ui, "Fill Rule", &mut fill, &FillOption::ALL) {
                self.update(AppMessage::String(StringMessage::FillSelected(fill)));
            }
            let mut mode = self.state.string.mode;
            if select(ui, "Mode", &mut mode, &ModeOption::ALL) {
                self.update(AppMessage::String(StringMessage::ModeSelected(mode)));
            }
        });
    }
}
