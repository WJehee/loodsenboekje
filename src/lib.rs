use cfg_if::cfg_if;

pub mod app;
// pub mod model;
// mod web;
// mod error;

cfg_if! { if #[cfg(feature = "hydrate")] {
    use leptos::*;
    use wasm_bindgen::prelude::wasm_bindgen;
    use crate::app::*;

    #[wasm_bindgen]
    pub fn hydrate() {
        leptos::mount_to_body(App);
    }
}}
