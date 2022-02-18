use eframe::egui::TextEdit;
use eframe::egui::{RichText, Color32};

use super::View;
use super::generic_components::*;
use crate::ciphers::Cipher;
use crate::ciphers::ADFGVX;
use crate::text_functions::PresetAlphabet;


impl View for ADFGVX {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, input: &mut String, output: &mut String, errors: &mut String) {

        ui.label("Select Mode");
        ui.horizontal(|ui| {
            if ui.button("ADFGX").clicked() { self.set_alphabet(PresetAlphabet::BasicLatinNoJ) };
            if ui.button("ADFGVX").clicked() { self.set_alphabet(PresetAlphabet::BasicLatinWithDigits) };
        });

        ui.add_space(10.0);
        ui.label(RichText::new(self.get_input_alphabet().clone()).monospace().background_color(Color32::BLACK));
        ui.add_space(16.0);

        ui.label("Polybius Key Word");
        ui.add(TextEdit::singleline(self.polybius.set_key()));

        ui.label(RichText::new(format!("Grid\n{}",self.polybius)).monospace());
        ui.add_space(16.0);

        ui.label("Columnar Key Word");
        ui.add(TextEdit::singleline(self.columnar.set_key()));

        encrypt_decrypt(ui, self, input, output, errors);
        ui.add_space(16.0);
        randomize_button(ui, self);
    }
}
