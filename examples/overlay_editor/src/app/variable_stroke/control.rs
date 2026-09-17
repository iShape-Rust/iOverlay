use crate::app::design::{controls, slider};
use crate::app::main::{AppMessage, EditorApp};
use crate::app::variable_stroke::content::VariableStrokeMessage;
use eframe::egui;

impl EditorApp {
    pub(crate) fn variable_stroke_control(&mut self, ui: &mut egui::Ui) {
        controls(ui, "variable_stroke_controls", |ui| {
            let mut width_scale = self.state.variable_stroke.width_scale;
            if slider(
                ui,
                "Width Scale",
                egui::Slider::new(&mut width_scale, 0.1..=3.0).step_by(0.01),
            ) {
                self.update(AppMessage::VariableStroke(
                    VariableStrokeMessage::WidthScaleUpdated(width_scale),
                ));
            }
            let mut round_angle = self.state.variable_stroke.round_angle;
            if slider(
                ui,
                "Round Detail",
                egui::Slider::new(&mut round_angle, 1..=50),
            ) {
                self.update(AppMessage::VariableStroke(
                    VariableStrokeMessage::RoundAngleUpdated(round_angle),
                ));
            }
        });
    }
}
