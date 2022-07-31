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

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Classic Cryptography",
        native_options,
        Box::new(|cc| Box::new(app::ClassicCrypto::build_with_context(cc))),
    );
}
