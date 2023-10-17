use cfg_if::cfg_if;

pub mod model;

cfg_if! { if #[cfg(feature = "hydrate")] {
    use leptos::*;
    use wasm_bindgen::prelude::wasm_bindgen;
    use crate::app::*;

    pub mod app;

    #[wasm_bindgen]
    pub fn hydrate() {
        leptos::mount_to_body(App);
    }
}}

cfg_if! { if #[cfg(feature = "ssr")] {
}}
