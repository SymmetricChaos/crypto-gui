pub mod ciphers;
pub mod cipher_panels;
use cipher_panels::CaesarApp;

fn main() {
    let app = CaesarApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}