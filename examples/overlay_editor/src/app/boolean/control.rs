use crate::app::boolean::content::BooleanMessage;
use crate::app::design::{controls, select};
use crate::app::main::{AppMessage, EditorApp};
use crate::app::{fill_option::FillOption, solver_option::SolverOption};
use eframe::egui;
use i_triangle::i_overlay::core::overlay_rule::OverlayRule;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum ModeOption {
    #[default]
    Edit,
    Debug,
    Subject,
    Clip,
    Intersect,
    Union,
    Difference,
    InverseDifference,
    Xor,
}

impl ModeOption {
    const ALL: [ModeOption; 9] = [
        ModeOption::Edit,
        ModeOption::Debug,
        ModeOption::Subject,
        ModeOption::Clip,
        ModeOption::Intersect,
        ModeOption::Union,
        ModeOption::Difference,
        ModeOption::InverseDifference,
        ModeOption::Xor,
    ];

    pub(crate) fn overlay_rule(&self) -> Option<OverlayRule> {
        match self {
            ModeOption::Subject => Some(OverlayRule::Subject),
            ModeOption::Clip => Some(OverlayRule::Clip),
            ModeOption::Intersect => Some(OverlayRule::Intersect),
            ModeOption::Union => Some(OverlayRule::Union),
            ModeOption::Difference => Some(OverlayRule::Difference),
            ModeOption::InverseDifference => Some(OverlayRule::InverseDifference),
            ModeOption::Xor => Some(OverlayRule::Xor),
            _ => None,
        }
    }
}

impl std::fmt::Display for ModeOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ModeOption::Edit => "Edit",
                ModeOption::Debug => "Debug",
                ModeOption::Subject => "Subject",
                ModeOption::Clip => "Clip",
                ModeOption::Intersect => "Intersect",
                ModeOption::Union => "Union",
                ModeOption::Difference => "Difference",
                ModeOption::InverseDifference => "InverseDifference",
                ModeOption::Xor => "Xor",
            }
        )
    }
}

impl EditorApp {
    pub(crate) fn boolean_control(&mut self, ui: &mut egui::Ui) {
        controls(ui, "boolean_controls", |ui| {
            let mut solver = self.state.boolean.solver;
            if select(ui, "Solver", &mut solver, &SolverOption::ALL) {
                self.update(AppMessage::Bool(BooleanMessage::SolverSelected(solver)));
            }
            let mut fill = self.state.boolean.fill;
            if select(ui, "Fill Rule", &mut fill, &FillOption::ALL) {
                self.update(AppMessage::Bool(BooleanMessage::FillSelected(fill)));
            }
            let mut mode = self.state.boolean.mode;
            if select(ui, "Mode", &mut mode, &ModeOption::ALL) {
                self.update(AppMessage::Bool(BooleanMessage::ModeSelected(mode)));
            }
        });
    }
}
