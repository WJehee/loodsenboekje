pub mod app;
pub mod components;
pub mod auth;
pub mod errors;
pub mod model;

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}

