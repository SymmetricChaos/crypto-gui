pub mod ciphers;
pub mod math_functions;
pub mod text_functions;
pub mod cipher_panel;
pub mod errors;
pub mod cipher_id;
mod app;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let app = app::ClassicCrypto::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}