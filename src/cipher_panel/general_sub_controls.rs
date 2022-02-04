use eframe::egui::TextEdit;
use eframe::egui::TextStyle;
use super::View;
use super::generic_components::*;
use crate::ciphers::Cipher;
use crate::ciphers::GeneralSubstitution;


impl View for GeneralSubstitution {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, input: &mut String, output: &mut String, errors: &mut String) {
        ui.add_space(16.0);
        ui.label("Plaintext Alphabet");
        ui.add(TextEdit::singleline(self.get_mut_input_alphabet()).text_style(TextStyle::Monospace));
        ui.add_space(16.0);

        ui.label("Ciphertext Alphabet");
        ui.add(TextEdit::singleline(self.get_mut_output_alphabet()).text_style(TextStyle::Monospace));
        ui.add_space(16.0);


        encrypt_decrypt(ui, self, input, output, errors);
        ui.add_space(16.0);
        randomize_button(ui, self);
    }
}
