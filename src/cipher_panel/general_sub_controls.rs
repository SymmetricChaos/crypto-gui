use eframe::egui::TextEdit;
use eframe::egui::TextStyle;
use super::View;
use super::generic_components::*;
use crate::ciphers::GeneralSubstitution;


impl View for GeneralSubstitution {
    fn ui(&mut self, ui: &mut eframe::egui::Ui) {
        ui.add_space(16.0);
        ui.label("Plaintext Alphabet");
        ui.add(TextEdit::singleline(self.control_alphabet1()).font(TextStyle::Monospace));
        ui.add_space(16.0);

        ui.label("Ciphertext Alphabet");
        ui.add(TextEdit::singleline(self.control_alphabet2()).font(TextStyle::Monospace));
        ui.add_space(16.0);

        randomize_button(ui, self);
    }
}
