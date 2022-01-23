use eframe::egui::TextEdit;
use eframe::egui::TextStyle;
use super::View;
use super::generic_components::*;
use crate::ciphers::Cipher;
use crate::ciphers::GeneralSubstitution;


impl View for GeneralSubstitution {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, input: &mut String, output: &mut String) {
        ui.add_space(16.0);
        ui.label("Plaintext Alphabet");
        ui.add(TextEdit::singleline(self.input_alphabet()).text_style(TextStyle::Monospace));
        ui.add_space(16.0);

        ui.label("Ciphertext Alphabet");
        ui.add(TextEdit::singleline(self.output_alphabet()).text_style(TextStyle::Monospace));
        ui.add_space(16.0);


        encrypt_decrypt(ui, self, input, output);
        ui.add_space(16.0);
        randomize_button(ui, self);
        ui.add_space(16.0);
        clear_button(ui, input, output);
    }
}
