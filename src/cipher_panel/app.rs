use eframe::{egui::{CtxRef, SidePanel, CentralPanel}, epi};

use super::View;

pub struct ClassicCrypto {
    display: super::DisplayPanel,
    control: super::ControlPanel,

}

impl Default for ClassicCrypto {
    fn default() -> Self {
        Self { 
            display: super::DisplayPanel::default(),
            control: super::ControlPanel::default(),
        }
    }
}


impl epi::App for ClassicCrypto {
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
        frame.set_window_size((1000.0,550.0).into());
        ctx.set_pixels_per_point(1.2);
        SidePanel::right("display_panel").show(ctx, |ui| {
            self.display.ui(ui)
        });
        CentralPanel::default().show(ctx, |ui| {
            self.control.ui(ui)
        });
    }
}