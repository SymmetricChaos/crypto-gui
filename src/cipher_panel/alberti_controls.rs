use eframe::egui::{RichText, Slider, TextEdit, TextStyle};
use super::View;
use super::generic_components::*;
use crate::ciphers::{Alberti, Cipher};


impl View for Alberti {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, input: &mut String, output: &mut String, errors: &mut String) {
        ui.add_space(16.0);
        ui.label("Fixed Alphabet");
        ui.add(TextEdit::singleline(self.get_mut_input_alphabet()).font(TextStyle::Monospace));
        ui.add_space(16.0);

        ui.label("Moving Alphabet");
        ui.add(TextEdit::singleline(self.get_mut_output_alphabet()).font(TextStyle::Monospace));
        ui.add_space(16.0);

        ui.label(RichText::new(self.to_string()).monospace());

        ui.label("Index");
        let alpha_range = 0..=((self.alphabet_len()-1));
        ui.add(Slider::new(&mut self.start_index, alpha_range.clone()));
        ui.add_space(16.0);

        encrypt_decrypt(ui, self, input, output, errors);
        ui.add_space(16.0);
        randomize_button(ui, self);
    }
}
