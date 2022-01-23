use eframe::{egui::{CtxRef, SidePanel, CentralPanel}, epi};

use crate::cipher_panel::{DisplayPanel, ControlPanel, View};


pub struct ClassicCrypto {
    display: DisplayPanel,
    control: ControlPanel,
    input: String,
    output: String,

}

impl Default for ClassicCrypto {
    fn default() -> Self {
        Self { 
            display: DisplayPanel::default(),
            control: ControlPanel::default(),
            input: String::new(),
            output: String::new(),
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
            self.display.ui(ui, &mut self.input, &mut self.output)
        });
        CentralPanel::default().show(ctx, |ui| {
            self.control.ui(ui, &mut self.input, &mut self.output)
        });
    }
}