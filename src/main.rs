pub mod ciphers;
pub mod panels;
pub mod app;
pub mod math;

fn main() {
    let app = app::DemoApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}