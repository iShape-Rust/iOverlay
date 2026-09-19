use eframe::egui::{self, Color32};

pub(crate) struct Design;

impl Design {
    pub(crate) fn solution_color() -> Color32 {
        Color32::from_rgb(32, 199, 32)
    }
    pub(crate) fn subject_color() -> Color32 {
        Color32::from_rgb(255, 51, 51)
    }
    pub(crate) fn clip_color() -> Color32 {
        Color32::from_rgb(26, 142, 255)
    }
    pub(crate) fn negative_color() -> Color32 {
        Color32::from_rgb(224, 224, 224)
    }
    pub(crate) fn accent_color() -> Color32 {
        Color32::from_rgb(255, 140, 0)
    }
    pub(crate) fn both_color() -> Color32 {
        Color32::from_rgb(76, 217, 100)
    }
}

const CONTROL_WIDTH: f32 = 232.0;

pub(crate) fn controls(ui: &mut egui::Ui, id: &str, content: impl FnOnce(&mut egui::Ui)) {
    ui.scope(|ui| {
        ui.spacing_mut().item_spacing = egui::vec2(8.0, 6.0);
        ui.spacing_mut().interact_size = egui::vec2(64.0, 26.0);
        ui.spacing_mut().slider_width = 160.0;
        egui::Grid::new(id)
            .num_columns(2)
            .min_col_width(130.0)
            .min_row_height(26.0)
            .spacing([16.0, 6.0])
            .show(ui, content);
    });
}

pub(crate) fn slider(ui: &mut egui::Ui, label: &str, slider: egui::Slider<'_>) -> bool {
    let label = ui.label(label);
    let changed = ui.add(slider).labelled_by(label.id).changed();
    ui.end_row();
    changed
}

pub(crate) fn checkbox(ui: &mut egui::Ui, label: &str, value: &mut bool) -> bool {
    let label = ui.label(label);
    let changed = ui.checkbox(value, "").labelled_by(label.id).changed();
    ui.end_row();
    changed
}

pub(crate) fn select<T: Copy + PartialEq + std::fmt::Display>(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut T,
    options: &[T],
) -> bool {
    let before = *value;
    let label_response = ui.label(label);
    egui::ComboBox::from_id_salt(label)
        .width(CONTROL_WIDTH)
        .selected_text(value.to_string())
        .show_ui(ui, |ui| {
            for &option in options {
                ui.selectable_value(value, option, option.to_string());
            }
        })
        .response
        .labelled_by(label_response.id);
    ui.end_row();
    *value != before
}
