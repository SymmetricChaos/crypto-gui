pub mod ciphers;
pub mod panels;
pub mod app;

fn main() {
    let app = app::DemoApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}