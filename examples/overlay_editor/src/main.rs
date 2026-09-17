#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    overlay_editor::run_desktop()
}

#[cfg(target_arch = "wasm32")]
fn main() {}
