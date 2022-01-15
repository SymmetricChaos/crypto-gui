use eframe::{egui::CtxRef, epi};
use crate::panels::cipher_windows::CipherWindows;

pub struct DemoApp {
    cipher_windows: CipherWindows,
}

impl Default for DemoApp {
    fn default() -> Self {
        Self { cipher_windows: CipherWindows::default() }
    }
}


impl epi::App for DemoApp {
    fn name(&self) -> &str {
        "Ciphers"
    }

    fn setup(
        &mut self,
        _ctx: &CtxRef,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
    }

    fn update(&mut self, ctx: &CtxRef, _frame: &epi::Frame) {
        self.cipher_windows.ui(ctx);
    }
}