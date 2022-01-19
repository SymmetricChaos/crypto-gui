use eframe::egui;
use crate::codes::{Godel,Code};
use super::{cipher_windows::View, display_panel, general_controls, input_alphabet};


pub struct GodelWindow {
    input: String,
    output: String,
    cipher: Affine,
}

impl Default for GodelWindow {
    fn default() -> Self {
        Self {
            input: String::new(),
            output: String::new(),
            cipher: Godel::new(0, 1, LATIN),
        }
    }
}


impl crate::panels::cipher_windows::View for GodelWindow {
    fn ui(&mut self, ui: &mut egui::Ui) {

        let Self{ input, output, cipher } = self;

        egui::SidePanel::left("control_panel").show_inside(ui, |ui| {
            ui.add_space(16.0);
            input_alphabet(ui, cipher);
            ui.add_space(16.0);

            ui.label("Additive Key");
            let alpha_range = 0..=((cipher.length()-1));
            ui.add(egui::Slider::new(&mut cipher.add_key, alpha_range.clone()));
            ui.add_space(16.0);

            ui.label("Multiplicative Key");
            ui.label(format!("Must not be divisible by the following numbers: {:?}",prime_factors(cipher.length())));
            let alpha_range = 1..=((cipher.length()-1));
            ui.add(egui::Slider::new(&mut cipher.mul_key, alpha_range));
            ui.add_space(16.0);

            // Currently we call this every frame even though we only need to do so when the Multiplicative Key slider is changed
            cipher.set_inverse();

            general_controls(ui, cipher, input, output);

        });


        display_panel(ui, 
            "",
            input, 
            output, 
        );


    }
}




impl crate::panels::cipher_windows::CipherWindow for GodelWindow {
    fn name(&self) -> &'static str {
        "Affine Cipher"
    }

    fn show(&mut self, ctx: &egui::CtxRef, open: &mut bool) {
        let window = egui::Window::new("Godel Encoding")
            .default_width(600.0)
            .default_height(400.0)
            .vscroll(false)
            .open(open);
        window.show(ctx, |ui| self.ui(ui));
    }
}