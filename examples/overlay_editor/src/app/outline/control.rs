use crate::app::design::{controls, select, slider};
use crate::app::main::{AppMessage, EditorApp};
use crate::app::outline::content::OutlineMessage;
use eframe::egui;

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
    pub(crate) fn outline_control(&mut self, ui: &mut egui::Ui) {
        controls(ui, "outline_controls", |ui| {
            let mut outer_offset = self.state.outline.outer_offset;
            if slider(
                ui,
                "Outer Offset",
                egui::Slider::new(&mut outer_offset, -50.0..=50.0).step_by(0.01),
            ) {
                self.update(AppMessage::Outline(
                    OutlineMessage::OuterOffsetValueUpdated(outer_offset),
                ));
            }
            let mut inner_offset = self.state.outline.inner_offset;
            if slider(
                ui,
                "Inner Offset",
                egui::Slider::new(&mut inner_offset, -50.0..=50.0).step_by(0.01),
            ) {
                self.update(AppMessage::Outline(
                    OutlineMessage::InnerOffsetValueUpdated(inner_offset),
                ));
            }
            let mut join = self.state.outline.join;
            if select(ui, "Line Join", &mut join, &JoinOption::ALL) {
                self.update(AppMessage::Outline(OutlineMessage::JoinSelected(join)));
            }
            if self.state.outline.join != JoinOption::Bevel {
                let mut join_value = self.state.outline.join_value;
                if slider(
                    ui,
                    "Join Detail",
                    egui::Slider::new(&mut join_value, 1..=100),
                ) {
                    self.update(AppMessage::Outline(OutlineMessage::JoinValueUpdated(
                        join_value,
                    )));
                }
            }
        });
    }
}
