use eframe::egui;
use crate::ciphers::M209;
use super::{cipher_windows::View, display_panel, general_controls};


pub struct M209Window {
    input: String,
    output: String,
    cipher: M209,
}

impl Default for M209Window {
    fn default() -> Self {
        Self {
            input: String::new(),
            output: String::new(),
            cipher: M209::default(),
        }
    }
}


impl crate::panels::cipher_windows::View for M209Window {
    fn ui(&mut self, ui: &mut egui::Ui) {

        let Self{ input, output, cipher } = self;

        egui::SidePanel::left("control_panel").show_inside(ui, |ui| {
            ui.add_space(16.0);
            ui.label("Alphabet");
            ui.label("ABDCEFGHIJKLMNOPQRSTUVWXYZ");
            ui.add_space(16.0);

            ui.label("Pins");
            //cipher.set_pins();
            ui.add_space(16.0);

            ui.label("Lugs");
            //cipher.set_lugs();
            ui.add_space(16.0);

            general_controls(ui, cipher, input, output);

            for rotor in cipher.rotors() {
                ui.label(rotor.to_string());
            }
            

        });


        display_panel(ui, 
            "The M209 Cipher",
            input, 
            output, 
        );
    }
}




impl crate::panels::cipher_windows::CipherWindow for M209Window {
    fn name(&self) -> &'static str {
        "M209 Cipher"
    }

    fn show(&mut self, ctx: &egui::CtxRef, open: &mut bool) {
        let window = egui::Window::new("M209 Cipher")
            .default_width(600.0)
            .default_height(400.0)
            .vscroll(false)
            .open(open);
        window.show(ctx, |ui| self.ui(ui));
    }
}