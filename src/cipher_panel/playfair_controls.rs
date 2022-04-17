use eframe::egui::{TextEdit, TextStyle, Ui};
use eframe::egui::{RichText,Color32};
use rand::prelude::StdRng;

use super::View;
use super::generic_components::*;
use crate::ciphers::Playfair;
use crate::preset_alphabet::PresetAlphabet::*;

impl View for Playfair {
    fn ui(&mut self, ui: &mut Ui, rng: &mut StdRng, _errors: &mut String) {

        randomize_reset(ui, self, rng);
        ui.add_space(16.0);
        
        ui.label("Select Alphabet");
        ui.horizontal(|ui| {
            if ui.button("No Q").clicked() { self.set_alphabet(BasicLatinNoQ) };
            if ui.button("No J").clicked() { self.set_alphabet(BasicLatinNoJ) };
            if ui.button("Alphanumeric").clicked() { self.set_alphabet(BasicLatinWithDigits) };
            if ui.button("Base64").clicked() { self.set_alphabet(Base64) };
        });
        ui.add_space(10.0);
        ui.label(RichText::new(&self.alphabet).monospace().background_color(Color32::BLACK));
        ui.add_space(16.0);

        ui.label("Key Word");
        text_edit(ui, self.control_key());

        ui.label("Spacer Character\nInserted as padding where needed");
        ui.add(TextEdit::singleline(&mut self.control_spacer().to_string()).font(TextStyle::Monospace).desired_width(15.0));

        ui.label(RichText::new(format!("Grid\n{}",self)).monospace());
        ui.add_space(16.0);

        //(ui, self.grid_side_len(), self.grid_side_len(), self.get_input_alphabet())
    }
}
