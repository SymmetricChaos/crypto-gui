pub mod ciphers;
pub mod codes;
pub mod cipher_id;
pub mod code_id;

pub mod cipher_panel;
pub mod code_panel;

pub mod math_functions;

pub mod text_functions;
pub mod vecstring;
pub mod preset_alphabet;

pub mod errors;

pub mod grid;

pub mod egui_aux;

mod app;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let app = app::ClassicCrypto::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}