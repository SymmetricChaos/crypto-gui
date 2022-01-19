pub mod ciphers;
pub mod panels;
pub mod app;
pub mod math_functions;
pub mod text_functions;
pub mod app_alt;

fn main() {
    let app = app_alt::DemoApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}