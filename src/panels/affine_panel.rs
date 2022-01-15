use eframe::{egui::{self, TextStyle}};
use crate::{ciphers::LATIN, math::prime_factors};
use crate::ciphers::Affine;
use super::{cipher_windows::View, Mode, run_cipher};


pub struct AffineWindow {
    plaintext: String,
    ciphertext: String,
    add_key: u32,
    mul_key: u32,
    alphabet: String,
    mode: Mode,
}

impl Default for AffineWindow {
    fn default() -> Self {
        Self {
            plaintext: String::new(),
            ciphertext: String::new(),
            alphabet: String::from(LATIN),
            add_key: 0,
            mul_key: 1,
            mode: Mode::Encrypt,
        }
    }
}


impl crate::panels::cipher_windows::View for AffineWindow {
    fn ui(&mut self, ui: &mut egui::Ui) {

        let Self{ plaintext, ciphertext, add_key, mul_key, alphabet, mode } = self;

        egui::SidePanel::left("affine_control_panel").show_inside(ui, |ui| {
            ui.add_space(16.0);
            ui.label("Alphabet");
            ui.add(egui::TextEdit::singleline(alphabet));
            ui.add_space(16.0);
            
            let alpha_range = 0u32..=((alphabet.chars().count()-1) as u32);

            ui.label("Additive Key");
            ui.add(egui::Slider::new(add_key, alpha_range.clone()));
            ui.add_space(16.0);

            ui.label("Multiplicative Key");
            ui.add(egui::Slider::new(mul_key, alpha_range));
            ui.add_space(16.0);

            ui.horizontal(|ui| {
                ui.selectable_value(mode, Mode::Encrypt, "Encrypt");
                ui.selectable_value(mode, Mode::Decrypt, "Decrypt");
            });
            ui.add_space(16.0);

            if ui.button("Clear").clicked() {
                *plaintext = String::new();
                *ciphertext = String::new();
            }
            
            let cipher = Affine::new(*add_key as usize, *mul_key as usize, &alphabet);
            run_cipher(mode, &cipher, plaintext, ciphertext);


        });

        egui::SidePanel::right("affine_display_panel")
            .default_width(500.0)
            .show_inside(ui, |ui| {

            ui.label("Description:\n");
            let alpha_len = alphabet.chars().count();
            ui.label(format!("Because the alphabet has {} characters the multiplicative key must not be divisible by the following numbers: {:?}",alpha_len,prime_factors(alpha_len)));

            ui.add_space(16.0);
            ui.separator();
            ui.add_space(16.0);

            ui.label("Plaintext");
            ui.add(egui::TextEdit::multiline(plaintext).hint_text("Plaintext Here").text_style(TextStyle::Monospace));
            ui.add_space(16.0);
            ui.label("Ciphertext");
            ui.add(egui::TextEdit::multiline(ciphertext).hint_text("Ciphertext Here").text_style(TextStyle::Monospace));
        });
        

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