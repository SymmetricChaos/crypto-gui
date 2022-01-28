use eframe::egui::Color32;
use eframe::egui::RichText;
use eframe::egui::TextEdit;
use eframe::egui::TextStyle;

use super::View;
use super::generic_components::*;
use crate::ciphers::{Cipher,Beaufort,PolyalphabeticMode};


impl View for Beaufort {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, input: &mut String, output: &mut String) {
        ui.add_space(16.0);
        input_alphabet(ui, self);
        ui.add_space(16.0);

        ui.label("Key Word");
        ui.add(TextEdit::singleline(&mut self.key_word).text_style(TextStyle::Monospace));

        ui.label("Mode");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.mode, PolyalphabeticMode::Cyclic, "Standard");
            ui.selectable_value(&mut self.mode, PolyalphabeticMode::Autokey, "Autokey");
        });

        if ui.button(RichText::from("ENCRYPT / DECRYPT").color(Color32::GOLD)).clicked() {
            match self.encrypt(input) {
                Ok(text) => *output = text,
                Err(e) => *output = e.to_string(),
            }
        };
        ui.add_space(16.0);
        randomize_button(ui, self);
        ui.add_space(16.0);
        clear_button(ui, input, output);
    }
}
