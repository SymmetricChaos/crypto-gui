use eframe::egui::{Color32, RichText, TextEdit, TextStyle, Ui};

use super::{View, ViewableCipher, _generic_components::*};
use crate::{ciphers::playfair::Playfair, text_aux::PresetAlphabet::*};

impl ViewableCipher for Playfair {}

impl View for Playfair {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Select Alphabet");
        ui.horizontal(|ui| {
            if ui.button("No Q").clicked() {
                self.assign_alphabet(BasicLatinNoQ)
            };
            if ui.button("No J").clicked() {
                self.assign_alphabet(BasicLatinNoJ)
            };
            if ui.button("Alphanumeric").clicked() {
                self.assign_alphabet(BasicLatinWithDigits)
            };
            if ui.button("Base64").clicked() {
                self.assign_alphabet(Base64)
            };
        });
        ui.add_space(10.0);
        ui.label(
            RichText::new(&self.alphabet)
                .monospace()
                .background_color(Color32::BLACK),
        );
        ui.add_space(16.0);

        ui.label("Key Word");
        text_edit(ui, self.control_key());

        ui.label("Spacer Character\nInserted as padding where needed");
        ui.add(
            TextEdit::singleline(&mut self.control_spacer().to_string())
                .font(TextStyle::Monospace)
                .desired_width(15.0),
        );

        ui.label(RichText::new(format!("Grid\n{}", self)).monospace());
        ui.add_space(16.0);

        //(ui, self.grid_side_len(), self.grid_side_len(), self.get_input_alphabet())
    }
}
