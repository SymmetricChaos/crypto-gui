use crate::cipher_id::CipherID;
use eframe::{egui::{CtxRef, SidePanel, CentralPanel, ScrollArea}, epi};

use crate::cipher_panel::{ControlPanel, DisplayPanel};


pub struct ClassicCrypto {
    control: ControlPanel,
    display: DisplayPanel,
    input: String,
    output: String,
    errors: String,
    active_cipher: CipherID,

}

impl Default for ClassicCrypto {
    fn default() -> Self {
        Self { 
            control: ControlPanel::default(),
            display: DisplayPanel::default(),
            input: String::new(),
            output: String::new(),
            errors: String::new(),
            active_cipher: CipherID::default(),
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

        SidePanel::right("display_panel").max_width(300.0).show(ctx, |ui| {
            self.display.ui(ui, &mut self.input, &mut self.output, &mut self.errors)
        });

        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                self.control.ui(ui, &mut self.input, &mut self.output, &mut self.errors, &mut self.active_cipher)
            });
        });
    }
}