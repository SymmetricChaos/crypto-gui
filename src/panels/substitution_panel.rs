use eframe::egui::{self, TextStyle};
use crate::ciphers::{LATIN, Substitution, Cipher};
use super::{input_alphabet, display_panel, general_controls};
use super::cipher_windows::View;


pub struct SubstitutionWindow {
    input: String,
    output: String,
    cipher: Substitution,
}

impl Default for SubstitutionWindow {
    fn default() -> Self {
        Self {
            input: String::new(),
            output: String::new(),
            cipher: Substitution::new(LATIN, LATIN),
        }
    }
}


impl crate::panels::cipher_windows::View for SubstitutionWindow {
    fn ui(&mut self, ui: &mut egui::Ui) {

        let Self{ input, output, cipher } = self;

        egui::SidePanel::left("control_panel").show_inside(ui, |ui| {
            ui.add_space(16.0);
            input_alphabet(ui, cipher);
            ui.add_space(16.0);

            ui.label("Cipher Alphabet");
            ui.add(egui::TextEdit::singleline(cipher.output_alphabet()).text_style(TextStyle::Monospace));
            ui.add_space(16.0);

            general_controls(ui, cipher, input, output);

        });

        display_panel(ui, 
            "The most general kind of simple substitution cipher maps the input alphabet to any arbitrary output alphabet. If these two alphabets are the same there are n! (n factorial) possibile keys where n is the number of letters in the alphabet. There are infinitely many options if the alphabets do not have to match. However like all simple substitution cipher frequency analysis easily breaks any significant amount of text.",
            input, 
            output, 
        );
        

    }
}


impl crate::panels::cipher_windows::CipherWindow for SubstitutionWindow {
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