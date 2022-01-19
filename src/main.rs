pub mod ciphers;
pub mod codes;
pub mod panels;
pub mod app;
pub mod math_functions;
pub mod text_functions;

fn main() {
    let app = app::DemoApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}