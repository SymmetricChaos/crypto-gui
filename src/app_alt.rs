use eframe::{egui::CtxRef, epi};
use crate::panels::cipher_windows_alt::Ciphers;

pub struct DemoApp {
    ciphers: Ciphers,
}

impl Default for DemoApp {
    fn default() -> Self {
        Self { ciphers: Ciphers::default() }
    }
}


impl epi::App for DemoApp {
    fn name(&self) -> &str {
        "Classical Cryptography"
    }

    fn setup(
        &mut self,
        _ctx: &CtxRef,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
    }

    fn update(&mut self, ctx: &CtxRef, frame: &epi::Frame) {
        self.ciphers.ui(ctx);
        frame.set_window_size((1000.0,600.0).into());
        ctx.set_pixels_per_point(1.2);
    }
}