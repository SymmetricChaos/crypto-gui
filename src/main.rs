pub mod ciphers;
pub mod math_functions;
pub mod text_functions;
pub mod cipher_panel;

fn main() {
    let app = cipher_panel::app::ClassicCrypto::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}