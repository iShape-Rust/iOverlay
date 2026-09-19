use crate::app::design::{checkbox, controls, select, slider};
use crate::app::main::{AppMessage, EditorApp};
use crate::app::stroke::content::StrokeMessage;
use eframe::egui;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum CapOption {
    #[default]
    Butt,
    Round,
    Square,
    Arrow,
}

impl CapOption {
    const ALL: [CapOption; 4] = [
        CapOption::Butt,
        CapOption::Round,
        CapOption::Square,
        CapOption::Arrow,
    ];
}

impl std::fmt::Display for CapOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CapOption::Butt => "Butt",
                CapOption::Round => "Round",
                CapOption::Square => "Square",
                CapOption::Arrow => "Arrow",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum JoinOption {
    #[default]
    Miter,
    Round,
    Bevel,
}

impl JoinOption {
    const ALL: [JoinOption; 3] = [JoinOption::Miter, JoinOption::Round, JoinOption::Bevel];
}

impl std::fmt::Display for JoinOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                JoinOption::Miter => "Miter",
                JoinOption::Round => "Round",
                JoinOption::Bevel => "Bevel",
            }
        )
    }
}

impl EditorApp {
    pub(crate) fn stroke_control(&mut self, ui: &mut egui::Ui) {
        controls(ui, "stroke_controls", |ui| {
            let mut width = self.state.stroke.width;
            if slider(
                ui,
                "Stroke Width",
                egui::Slider::new(&mut width, 0.1..=10.0).step_by(0.01),
            ) {
                self.update(AppMessage::Stroke(StrokeMessage::WidthValueUpdated(width)));
            }
            let mut start_cap = self.state.stroke.start_cap;
            if select(ui, "Start Cap", &mut start_cap, &CapOption::ALL) {
                self.update(AppMessage::Stroke(StrokeMessage::StartCapSelected(
                    start_cap,
                )));
            }
            if self.state.stroke.start_cap == CapOption::Round {
                let mut start_cap_value = self.state.stroke.start_cap_value;
                if slider(
                    ui,
                    "Start Cap Detail",
                    egui::Slider::new(&mut start_cap_value, 1..=100),
                ) {
                    self.update(AppMessage::Stroke(StrokeMessage::StartCapValueUpdated(
                        start_cap_value,
                    )));
                }
            }
            let mut end_cap = self.state.stroke.end_cap;
            if select(ui, "End Cap", &mut end_cap, &CapOption::ALL) {
                self.update(AppMessage::Stroke(StrokeMessage::EndCapSelected(end_cap)));
            }
            if self.state.stroke.end_cap == CapOption::Round {
                let mut end_cap_value = self.state.stroke.end_cap_value;
                if slider(
                    ui,
                    "End Cap Detail",
                    egui::Slider::new(&mut end_cap_value, 1..=100),
                ) {
                    self.update(AppMessage::Stroke(StrokeMessage::EndCapValueUpdated(
                        end_cap_value,
                    )));
                }
            }
            let mut join = self.state.stroke.join;
            if select(ui, "Line Join", &mut join, &JoinOption::ALL) {
                self.update(AppMessage::Stroke(StrokeMessage::JoinSelected(join)));
            }
            if self.state.stroke.join != JoinOption::Bevel {
                let mut join_value = self.state.stroke.join_value;
                if slider(
                    ui,
                    "Join Detail",
                    egui::Slider::new(&mut join_value, 1..=100),
                ) {
                    self.update(AppMessage::Stroke(StrokeMessage::JoinValueUpdated(
                        join_value,
                    )));
                }
            }
            let mut is_closed = self.state.stroke.is_closed;
            if checkbox(ui, "Is Closed", &mut is_closed) {
                self.update(AppMessage::Stroke(StrokeMessage::IsClosedUpdated(
                    is_closed,
                )));
            }
        });
    }
}
