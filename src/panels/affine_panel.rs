use eframe::egui;
use crate::{ciphers::LATIN, math_functions::prime_factors};
use crate::ciphers::Affine;
use super::{decrypt_button, encrypt_button};
use super::{cipher_windows::View, display_panel, clear_button, input_alphabet, randomize_button};


pub struct AffineWindow {
    input: String,
    output: String,
    cipher: Affine,
}

impl Default for AffineWindow {
    fn default() -> Self {
        Self {
            input: String::new(),
            output: String::new(),
            cipher: Affine::new(0, 1, LATIN),
        }
    }
}


impl crate::panels::cipher_windows::View for AffineWindow {
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
            cipher.set_inverse();
            ui.add_space(16.0);

            ui.horizontal(|ui| {
                encrypt_button(ui, cipher, input, output);
                decrypt_button(ui, cipher, input, output);
            });
            ui.add_space(32.0);

            clear_button(ui, input, output);
            ui.add_space(16.0);

            randomize_button(ui, cipher);

        });


        display_panel(ui, 
            "The Affine Cipher is a slight improvement on the Caesar Cipher. Rather than just adding a value to each letter's position the value is also multiplied, thus increasing the number of possible keys. This introduces a slight difficulty since multiplication does not have an unique inverse modulo 'n' unless the the multiplier is coprime to 'n'.",
            input, 
            output, 
        );


    }
}




impl crate::panels::cipher_windows::CipherWindow for AffineWindow {
    fn name(&self) -> &'static str {
        "Affine Cipher"
    }

    fn show(&mut self, ctx: &egui::CtxRef, open: &mut bool) {
        let window = egui::Window::new("Affine Cipher")
            .default_width(600.0)
            .default_height(400.0)
            .vscroll(false)
            .open(open);
        window.show(ctx, |ui| self.ui(ui));
    }
}