#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

pub mod app;

pub mod grid;
pub mod math_functions;
pub mod rotors;

pub mod ids;

pub mod cipher_panel;
pub mod ciphers;

pub mod code_panel;
pub mod codes;

pub mod errors;
pub mod pages;

pub mod egui_aux;

pub mod text_aux;

pub mod global_rng;

pub mod tokenizer;

// // ----------------------------------------------------------------------------
// // When compiling for web:

// #[cfg(target_arch = "wasm32")]
// use eframe::wasm_bindgen::{self, prelude::*};

// /// This is the entry-point for all the web-assembly.
// /// This is called once from the HTML.
// /// It loads the app, installs some callbacks, then returns.
// /// You can add more callbacks like this if you want to call in to your code.
// #[cfg(target_arch = "wasm32")]
// #[wasm_bindgen]
// pub fn start(canvas_id: &str) -> Result<(), eframe::wasm_bindgen::JsValue> {
//     let app = app::ClassicCrypto::default();
//     eframe::start_web(
//         canvas_id,
//         Box::new(|cc| Box::new(app::ClassicCrypto::build_with_context(cc))),
//     )
// }
