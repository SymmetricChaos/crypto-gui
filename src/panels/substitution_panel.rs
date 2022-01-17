use eframe::egui::{self, TextStyle};
use crate::ciphers::LATIN;
use crate::ciphers::{Substitution, Cipher};
use super::{input_alphabet, mode_selector, clear_button, randomize_button};
use super::{cipher_windows::View, Mode, run_button};


pub struct SubstitutionWindow {
    plaintext: String,
    ciphertext: String,
    cipher: Substitution,
    mode: Mode,
}

impl Default for SubstitutionWindow {
    fn default() -> Self {
        Self {
            plaintext: String::new(),
            ciphertext: String::new(),
            cipher: Substitution::new(LATIN, LATIN),
            mode: Mode::Encrypt,
        }
    }
}


impl crate::panels::cipher_windows::View for SubstitutionWindow {
    fn ui(&mut self, ui: &mut egui::Ui) {


        let Self{ plaintext, ciphertext, cipher, mode } = self;

        egui::SidePanel::left("control_panel").show_inside(ui, |ui| {
            ui.add_space(16.0);
            input_alphabet(ui, cipher);
            ui.add_space(16.0);

            ui.label("Cipher Alphabet");
            ui.add(egui::TextEdit::singleline(cipher.output_alphabet()).text_style(TextStyle::Monospace));
            ui.add_space(16.0);

            mode_selector(ui, mode);
            ui.add_space(16.0);

            run_button(ui, mode, cipher, plaintext, ciphertext);
            ui.add_space(32.0);

            clear_button(ui, plaintext, ciphertext);
            ui.add_space(16.0);

            randomize_button(ui, cipher);

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