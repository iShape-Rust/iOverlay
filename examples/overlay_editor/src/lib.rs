mod app;
mod data;
mod draw;
mod geom;
mod point_editor;
mod sheet;
#[cfg(target_arch = "wasm32")]
pub mod web;

#[cfg(not(target_arch = "wasm32"))]
pub fn run_desktop() -> eframe::Result {
    let resources = data::resource::AppResource::with_paths(
        concat!(env!("CARGO_MANIFEST_DIR"), "/../tests/boolean"),
        concat!(env!("CARGO_MANIFEST_DIR"), "/../tests/string"),
        concat!(env!("CARGO_MANIFEST_DIR"), "/../tests/stroke"),
        concat!(env!("CARGO_MANIFEST_DIR"), "/../tests/variable_stroke"),
        concat!(env!("CARGO_MANIFEST_DIR"), "/../tests/outline"),
    );
    eframe::run_native(
        "iOverlay Editor",
        eframe::NativeOptions {
            viewport: eframe::egui::ViewportBuilder::default().with_inner_size([1280.0, 800.0]),
            centered: true,
            ..Default::default()
        },
        Box::new(move |cc| {
            cc.egui_ctx.set_visuals(eframe::egui::Visuals::dark());
            Ok(Box::new(app::main::EditorApp::with_resource(resources)))
        }),
    )
}
