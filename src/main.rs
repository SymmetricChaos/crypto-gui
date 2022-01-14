pub mod ciphers;
pub mod caesar_panel;
use caesar_panel::CaesarApp;

fn main() {
    let app = CaesarApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}