pub mod cipher_id;
pub mod ciphers;
pub mod code_id;
pub mod codes;

pub mod cipher_panel;
pub mod code_panel;

pub mod pages;

pub mod math_functions;

pub mod text_aux;

pub mod errors;

pub mod grid;

pub mod egui_aux;

pub mod global_rng;

pub mod app;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Classic Cryptography",
        native_options,
        Box::new(|cc| Box::new(app::ClassicCrypto::build_with_context(cc))),
    );
}
