use eframe::egui::TextEdit;
use eframe::egui::{RichText, Color32};

use super::View;
use super::generic_components::*;
use crate::ciphers::Cipher;
use crate::ciphers::Polybius;
use crate::text_types::{PresetAlphabet::*};


impl View for Polybius {
    fn ui(&mut self, ui: &mut eframe::egui::Ui) {

        randomize_button(ui, self);
        ui.add_space(16.0);

        ui.label("Select Alphabet");
        ui.horizontal(|ui| {
            if ui.button("No Q").clicked() { self.set_alphabet(BasicLatinNoQ) };
            if ui.button("No J").clicked() { self.set_alphabet(BasicLatinNoJ) };
            if ui.button("Alphanumeric").clicked() { self.set_alphabet(BasicLatinWithDigits) };
            if ui.button("Base64").clicked() { self.set_alphabet(Base64) };
        });

        ui.add_space(10.0);
        ui.label(RichText::new(self.get_mut_input_alphabet().clone()).monospace().background_color(Color32::BLACK));
        ui.add_space(16.0);

        ui.label("Key Word");
        ui.add(TextEdit::singleline(self.control_key()));

        ui.label(RichText::new(format!("Grid\n{}",self)).monospace());
        ui.add_space(16.0);
    }
}
