use eframe::egui;
use crate::{ciphers::LATIN, math::prime_factors};
use crate::ciphers::Affine;
use super::{cipher_windows::View, Mode, display_panel, run_button, clear_button, mode_selector, input_alphabet, randomize_button};


pub struct AffineWindow {
    plaintext: String,
    ciphertext: String,
    cipher: Affine,
    mode: Mode,
}

impl Default for AffineWindow {
    fn default() -> Self {
        Self {
            plaintext: String::new(),
            ciphertext: String::new(),
            cipher: Affine::new(0, 1, LATIN),
            mode: Mode::Encrypt,
        }
    }
}


impl crate::panels::cipher_windows::View for AffineWindow {
    fn ui(&mut self, ui: &mut egui::Ui) {

        let Self{ plaintext, ciphertext, cipher, mode } = self;

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
            ui.add(egui::Slider::new(&mut cipher.mul_key, alpha_range));
            ui.add_space(16.0);

            mode_selector(ui, mode);
            ui.add_space(16.0);

            run_button(ui, mode, cipher, plaintext, ciphertext);
            ui.add_space(32.0);

            clear_button(ui, plaintext, ciphertext);
            ui.add_space(16.0);

            randomize_button(ui, cipher);

        });


        display_panel(ui, 
            "The Caesar Cipher is one of the oldest and simplest forms of cryptography. The key is any positive whole number. Each letter of the plaintext is shifted that many positions in the alphabet, wrapping around at the end.",
            plaintext, 
            ciphertext, 
        );


    }
}




impl crate::panels::cipher_windows::CipherFrame for AffineWindow {
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