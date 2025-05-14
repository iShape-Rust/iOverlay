use std::panic;
use std::sync::Once;
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub struct WebApp {}

static INIT_LOGGER: Once = Once::new();

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl WebApp {
    #[wasm_bindgen(constructor)]
    pub fn create() -> Self { Self {} }

    #[wasm_bindgen]
    pub fn start(
        &mut self,
        boolean_data: String,
        string_data: String,
        stroke_data: String,
        outline_data: String,
    ) {
        use log::info;
        use iced::application;

        use crate::app::main::EditorApp;
        use crate::data::resource::AppResource;

        panic::set_hook(Box::new(console_error_panic_hook::hook));
        INIT_LOGGER.call_once(|| {
            console_log::init_with_level(log::Level::Debug).expect("error initializing log");
        });

        info!("wasm start");

        let app_initializer = move || {
            info!("wasm init");
            let app_resource = AppResource::with_content(
                &boolean_data,
                &string_data,
                &stroke_data,
                &outline_data,
            );
            let app = EditorApp::with_resource(app_resource);

            (app, iced::Task::none())
        };

        application(app_initializer, EditorApp::update, EditorApp::view)
            .resizable(true)
            .default_font(custom_font)
            .centered()
            .title("iOverlay Editor")
            .run()
            .unwrap();

        info!("wasm app run");
    }
}