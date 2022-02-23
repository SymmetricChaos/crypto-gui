use eframe::egui::{RichText, TextEdit};

use super::View;
use super::generic_components::*;
use crate::ciphers::ADFGVX;
use crate::text_types::{PresetAlphabet::*};


impl View for ADFGVX {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, input: &mut String, output: &mut String, errors: &mut String) {

        ui.label("Select Mode");
        ui.horizontal(|ui| {
            if ui.button("ADFGX").clicked() { self.set_alphabet(BasicLatinNoJ) };
            if ui.button("ADFGVX").clicked() { self.set_alphabet(BasicLatinWithDigits) };
        });

        ui.label("Polybius Key Word");
        ui.add(TextEdit::singleline(self.polybius.control_key()));

        ui.label(RichText::new(format!("Grid\n{}",self.polybius)).monospace());
        ui.add_space(16.0);

        ui.label("Columnar Key Word");
        ui.add(TextEdit::singleline(self.columnar.control_key()));

        encrypt_decrypt(ui, self, input, output, errors);
        ui.add_space(16.0);
        randomize_button(ui, self);
    }
}
