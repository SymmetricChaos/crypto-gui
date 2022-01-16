use eframe::{egui::{self, TextStyle}};
use rand::prelude::ThreadRng;
use crate::{ciphers::LATIN, math::shuffle_str};
use crate::ciphers::Substitution;
use super::{cipher_windows::View, Mode, run_cipher};


pub struct SubstitutionWindow {
    plaintext: String,
    ciphertext: String,
    alphabet1: String,
    alphabet2: String,
    mode: Mode,
}

impl Default for SubstitutionWindow {
    fn default() -> Self {
        Self {
            plaintext: String::new(),
            ciphertext: String::new(),
            alphabet1: LATIN.to_string(),
            alphabet2: LATIN.to_string(),
            mode: Mode::Encrypt,
        }
    }
}


impl crate::panels::cipher_windows::View for SubstitutionWindow {
    fn ui(&mut self, ui: &mut egui::Ui) {

        let Self{ plaintext, ciphertext, alphabet1, alphabet2, mode } = self;

        egui::SidePanel::left("subs_control_panel").show_inside(ui, |ui| {
            ui.add_space(16.0);
            ui.label("Plaintext Alphabet");
            ui.add(egui::TextEdit::singleline(alphabet1));
            ui.add_space(16.0);

            ui.add_space(16.0);
            ui.label("Ciphertext Alphabet");
            ui.add(egui::TextEdit::singleline(alphabet2));
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

            let rng = &mut ThreadRng::default();
            if ui.button("Randomize").clicked() {
                *alphabet2 = shuffle_str(alphabet1, rng)
            }
            
            let cipher = Substitution::new(alphabet1, alphabet2);
            run_cipher(mode, &cipher, plaintext, ciphertext);

        });

        egui::SidePanel::right("subs_display_panel")
            .default_width(500.0)
            .show_inside(ui, |ui| {

            ui.label("Description:\nThe most general kind of simple substitution cipher maps the input alphabet to any arbitrary output alphabet. If these two alphabets are the same there are n! (n factorial) possibile keys where n is the number of letters in the alphabet. There are infinitely many options if the alphabets do not have to match. However like all simple substitution cipher frequency analysis easily breaks any significant amount of text.");

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


impl crate::panels::cipher_windows::CipherFrame for SubstitutionWindow {
    fn name(&self) -> &'static str {
        "Substitution Cipher"
    }

    fn show(&mut self, ctx: &egui::CtxRef, open: &mut bool) {
        let window = egui::Window::new("Substitution Cipher")
            .default_width(600.0)
            .default_height(400.0)
            .vscroll(false)
            .open(open);
        window.show(ctx, |ui| self.ui(ui));
    }
}