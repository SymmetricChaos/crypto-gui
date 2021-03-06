#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds

pub mod app;
pub mod cipher_panel;
pub mod ciphers;
pub mod grid;
pub mod math_functions;

pub mod cipher_id;
pub mod code_id;
pub mod code_panel;
pub mod codes;
pub mod errors;
pub mod pages;

pub mod egui_aux;

pub mod rotors;

pub mod text_aux;

pub mod global_rng;

// ----------------------------------------------------------------------------
// When compiling for web:

#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};

/// This is the entry-point for all the web-assembly.
/// This is called once from the HTML.
/// It loads the app, installs some callbacks, then returns.
/// You can add more callbacks like this if you want to call in to your code.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), eframe::wasm_bindgen::JsValue> {
    let app = app::ClassicCrypto::default();
    eframe::start_web(
        canvas_id,
        Box::new(|cc| Box::new(app::ClassicCrypto::build_with_context(cc))),
    )
}
