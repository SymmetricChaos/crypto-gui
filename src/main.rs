pub mod ciphers;
pub mod math_functions;
pub mod text_functions;
pub mod cipher_panel;
mod app;

fn main() {
    let app = app::ClassicCrypto::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}