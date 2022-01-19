use crate::math_functions::prime_factors;
use eframe::egui;
use eframe::egui::Response;
use crate::ciphers::LATIN;
use crate::ciphers::Affine;
use super::decrypt_button;
use super::encrypt_button;
use super::randomize_button;
use super::{clear_button, input_alphabet};


pub struct AffineWidget {
    input: String,
    output: String,
    cipher: Affine,
}

impl Default for AffineWidget {
    fn default() -> Self {
        Self {
            input: String::new(),
            output: String::new(),
            cipher: Affine::new(0, 1, LATIN),
        }
    }
}


impl egui::Widget for &mut AffineWidget {
    fn ui(self, ui: &mut egui::Ui) -> Response {

        let cipher = &mut self.cipher;
        let input = &mut self.input;
        let output = &mut self.output;

        egui::CentralPanel::default().show_inside(ui, |ui| {
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

            ui.horizontal(|ui| {
                encrypt_button(ui, cipher, input, output);
                decrypt_button(ui, cipher, input, output);
            });
            ui.add_space(32.0);

            clear_button(ui, input, output);
            ui.add_space(16.0);

            randomize_button(ui, cipher);

        }).response
    }
}