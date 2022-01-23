use eframe::egui::Slider;
use eframe::egui::TextEdit;
use eframe::egui::TextStyle;
use super::View;
use super::generic_components::*;
use crate::ciphers::Cipher;
use crate::{ciphers::GeneralSubstitution, text_functions::LATIN_UPPER};

pub struct GeneralSubstitutionControls {
    cipher: GeneralSubstitution,
}

impl Default for GeneralSubstitutionControls {
    fn default() -> Self {
        Self { 
            cipher: GeneralSubstitution::new(LATIN_UPPER, LATIN_UPPER),
        }
    }
}

impl View for GeneralSubstitutionControls {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, input: &mut String, output: &mut String) {
        ui.add_space(16.0);
        ui.label("Plaintext Alphabet");
        ui.add(TextEdit::singleline(self.cipher.input_alphabet()).text_style(TextStyle::Monospace));
        ui.add_space(16.0);

        ui.label("Ciphertext Alphabet");
        ui.add(TextEdit::singleline(self.cipher.output_alphabet()).text_style(TextStyle::Monospace));
        ui.add_space(16.0);


        encrypt_decrypt(ui, &mut self.cipher, input, output);
        ui.add_space(16.0);
        randomize_button(ui, &mut self.cipher);
        ui.add_space(16.0);
        clear_button(ui, input, output);
    }
}
