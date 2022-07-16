use eframe::egui::{RichText, TextEdit, TextStyle, Ui};

use crate::ciphers::playfair::Slidefair;

use super::{View, ViewableCipher, _generic_components::*};

impl ViewableCipher for Slidefair {}

impl View for Slidefair {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Alphabet");
        if control_string(ui, &mut self.alphabet_string).changed() {
            self.set_alphabet()
        }
        ui.add_space(16.0);

        ui.label("Key Word");
        if control_string(ui, &mut self.key_word).changed() {
            self.set_key()
        }
        ui.add_space(16.0);

        ui.label("Spacer Character\nInserted at end as padding if needed");
        ui.add(
            TextEdit::singleline(self.control_spacer())
                .font(TextStyle::Monospace)
                .desired_width(15.0),
        );

        ui.label("Grid");
        for row in self.rows() {
            ui.label(RichText::new(row).monospace());
        }

        ui.add_space(16.0);
    }
}
