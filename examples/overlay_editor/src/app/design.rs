use eframe::egui::{self, Color32};

pub(crate) struct Design;

impl Design {
    pub(crate) fn canvas_color() -> Color32 {
        Color32::from_rgb(21, 24, 29)
    }
    pub(crate) fn panel_color() -> Color32 {
        Color32::from_rgb(32, 36, 43)
    }
    pub(crate) fn border_color() -> Color32 {
        Color32::from_rgb(48, 54, 65)
    }
    pub(crate) fn solution_color() -> Color32 {
        Color32::from_rgb(101, 214, 160)
    }
    pub(crate) fn subject_color() -> Color32 {
        Color32::from_rgb(240, 128, 128)
    }
    pub(crate) fn clip_color() -> Color32 {
        Color32::from_rgb(103, 174, 247)
    }
    pub(crate) fn negative_color() -> Color32 {
        Color32::from_rgb(222, 228, 237)
    }
    pub(crate) fn accent_color() -> Color32 {
        Color32::from_rgb(167, 139, 250)
    }
    pub(crate) fn both_color() -> Color32 {
        Color32::from_rgb(101, 214, 160)
    }

    pub(crate) fn apply(ctx: &egui::Context) {
        let mut style = (*ctx.global_style()).clone();
        let mut visuals = egui::Visuals::dark();
        visuals.panel_fill = Self::panel_color();
        visuals.window_fill = Self::panel_color();
        visuals.extreme_bg_color = Self::canvas_color();
        visuals.faint_bg_color = Self::border_color();
        visuals.override_text_color = Some(Self::negative_color());
        visuals.weak_text_color = Some(Color32::from_rgb(145, 155, 173));
        visuals.selection.bg_fill = Color32::from_rgb(64, 52, 91);
        visuals.selection.stroke = egui::Stroke::new(1.0_f32, Self::accent_color());
        visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(1.0_f32, Self::border_color());
        visuals.widgets.inactive.bg_fill = Color32::from_rgb(42, 47, 57);
        visuals.widgets.inactive.weak_bg_fill = Color32::from_rgb(42, 47, 57);
        visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0_f32, Self::border_color());
        visuals.widgets.hovered.bg_fill = Color32::from_rgb(55, 59, 73);
        visuals.widgets.hovered.weak_bg_fill = Color32::from_rgb(55, 59, 73);
        visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.0_f32, Self::accent_color());
        visuals.widgets.active.bg_fill = Color32::from_rgb(64, 52, 91);
        visuals.widgets.active.weak_bg_fill = Color32::from_rgb(64, 52, 91);
        visuals.widgets.active.bg_stroke = egui::Stroke::new(1.0_f32, Self::accent_color());
        for widget in [
            &mut visuals.widgets.noninteractive,
            &mut visuals.widgets.inactive,
            &mut visuals.widgets.hovered,
            &mut visuals.widgets.active,
            &mut visuals.widgets.open,
        ] {
            widget.corner_radius = egui::CornerRadius::same(5);
        }
        style.visuals = visuals;
        style.spacing.item_spacing = egui::vec2(8.0, 8.0);
        style.spacing.interact_size = egui::vec2(64.0, 30.0);
        style.spacing.button_padding = egui::vec2(10.0, 5.0);
        style
            .text_styles
            .insert(egui::TextStyle::Body, egui::FontId::proportional(13.0));
        style
            .text_styles
            .insert(egui::TextStyle::Button, egui::FontId::proportional(13.0));
        style
            .text_styles
            .insert(egui::TextStyle::Heading, egui::FontId::proportional(16.0));
        ctx.set_global_style(style);
    }

    pub(crate) fn legend(ui: &mut egui::Ui, color: Color32, label: &str) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 5.0;
            let (rect, _) = ui.allocate_exact_size(egui::vec2(8.0, 8.0), egui::Sense::hover());
            ui.painter().circle_filled(rect.center(), 4.0, color);
            ui.colored_label(color, label);
        });
    }

    pub(crate) fn panel_frame(margin: i8) -> egui::Frame {
        egui::Frame::new()
            .fill(Self::panel_color())
            .inner_margin(margin)
    }
}

pub(crate) fn tests_panel(ui: &egui::Ui, id: &str) -> egui::Panel {
    egui::Panel::left(egui::Id::new(id))
        .exact_size((ui.available_width() * 0.15).min(120.0))
        .resizable(false)
        .frame(Design::panel_frame(10))
}

pub(crate) fn inspector(
    ui: &mut egui::Ui,
    id: &str,
    title: &str,
    content: impl FnOnce(&mut egui::Ui),
) {
    egui::Panel::right(egui::Id::new(id))
        .default_size(280.0)
        .size_range(240.0..=320.0)
        .frame(Design::panel_frame(16))
        .show_inside(ui, |ui| {
            ui.heading(title);
            ui.add_space(4.0);
            ui.separator();
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, content);
        });
}

pub(crate) fn controls(ui: &mut egui::Ui, id: &str, content: impl FnOnce(&mut egui::Ui)) {
    ui.push_id(id, |ui| {
        ui.spacing_mut().item_spacing = egui::vec2(8.0, 6.0);
        ui.spacing_mut().slider_width = (ui.available_width() - 96.0).max(40.0);
        content(ui);
    });
}

pub(crate) fn slider(ui: &mut egui::Ui, label: &str, slider: egui::Slider<'_>) -> bool {
    let label = ui.label(label);
    let changed = ui.add(slider).labelled_by(label.id).changed();
    ui.add_space(6.0);
    changed
}

pub(crate) fn checkbox(ui: &mut egui::Ui, label: &str, value: &mut bool) -> bool {
    let changed = ui.checkbox(value, label).changed();
    ui.add_space(6.0);
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
        .width(ui.available_width())
        .truncate()
        .selected_text(value.to_string())
        .show_ui(ui, |ui| {
            for &option in options {
                ui.selectable_value(value, option, option.to_string());
            }
        })
        .response
        .labelled_by(label_response.id);
    ui.add_space(6.0);
    *value != before
}
