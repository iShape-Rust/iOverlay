use crate::{app::main::EditorApp, data::resource::AppResource};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WebApp {
    runner: eframe::WebRunner,
}

#[wasm_bindgen]
impl WebApp {
    #[wasm_bindgen(constructor)]
    pub fn create() -> Self {
        console_error_panic_hook::set_once();
        let _ = console_log::init_with_level(log::Level::Debug);
        Self {
            runner: eframe::WebRunner::new(),
        }
    }

    /// Keeps the existing five JSON arguments. Await the returned Promise to report startup errors.
    pub async fn start(
        &self,
        boolean_data: String,
        string_data: String,
        stroke_data: String,
        variable_stroke_data: String,
        outline_data: String,
    ) -> Result<(), JsValue> {
        let document = web_sys::window()
            .and_then(|window| window.document())
            .ok_or_else(|| JsValue::from_str("Browser document is unavailable"))?;
        let canvas = match document.get_element_by_id("overlay-editor-canvas") {
            Some(element) => element.dyn_into::<web_sys::HtmlCanvasElement>()?,
            None => {
                let canvas = document
                    .create_element("canvas")?
                    .dyn_into::<web_sys::HtmlCanvasElement>()?;
                canvas.set_id("overlay-editor-canvas");
                canvas.style().set_property("width", "100vw")?;
                canvas.style().set_property("height", "100vh")?;
                canvas.style().set_property("display", "block")?;
                let body = document
                    .body()
                    .ok_or_else(|| JsValue::from_str("Browser body is unavailable"))?;
                body.style().set_property("margin", "0")?;
                body.append_child(&canvas)?;
                canvas
            }
        };
        let resource = AppResource::with_content(
            &boolean_data,
            &string_data,
            &stroke_data,
            &variable_stroke_data,
            &outline_data,
        );
        self.runner
            .start(
                canvas,
                eframe::WebOptions::default(),
                Box::new(move |cc| {
                    cc.egui_ctx.set_visuals(eframe::egui::Visuals::dark());
                    Ok(Box::new(EditorApp::with_resource(resource)))
                }),
            )
            .await
    }

    pub fn destroy(&self) {
        self.runner.destroy();
    }
}

impl Default for WebApp {
    fn default() -> Self {
        Self::create()
    }
}
