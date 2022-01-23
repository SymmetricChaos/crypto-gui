use eframe::{egui::{CtxRef, SidePanel, CentralPanel, TextStyle, TextEdit, ScrollArea}, epi};

use crate::cipher_panel::{ControlPanel, CipherID};


pub struct ClassicCrypto {
    control: ControlPanel,
    input: String,
    output: String,
    active_cipher: CipherID,

}

impl Default for ClassicCrypto {
    fn default() -> Self {
        Self { 
            control: ControlPanel::default(),
            input: String::new(),
            output: String::new(),
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
            ui.label(format!{"Description:\n{}",self.active_cipher.description()});

            ui.add_space(16.0);
            ui.separator();
            ui.add_space(16.0);
    
            ui.label("INPUT TEXT");
            ui.add(TextEdit::multiline(&mut self.input).text_style(TextStyle::Monospace));
            ui.add_space(16.0);
            ui.label("OUTPUT TEXT");
            ui.add(TextEdit::multiline(&mut self.output).text_style(TextStyle::Monospace));
        });

        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                self.control.ui(ui, &mut self.input, &mut self.output, &mut self.active_cipher)
            });
        });
    }
}