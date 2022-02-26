use eframe::egui::{RichText, Slider, TextEdit, TextStyle};
use super::View;
use super::generic_components::*;
use crate::ciphers::{Alberti};


impl View for Alberti {
    fn ui(&mut self, ui: &mut eframe::egui::Ui) {
        ui.add_space(16.0);
        ui.label("Fixed Alphabet");
        ui.add(TextEdit::singleline(self.control_fixed_alphabet()).font(TextStyle::Monospace));
        ui.add_space(16.0);

        ui.label("Moving Alphabet");
        ui.add(TextEdit::singleline(self.control_moving_alphabet()).font(TextStyle::Monospace));
        ui.add_space(16.0);

        ui.label(RichText::new(self.to_string()).monospace());

        ui.label("Index");
        let alpha_range = 0..=((self.alphabet_len()-1));
        ui.add(Slider::new(&mut self.start_index, alpha_range.clone()));
        ui.add_space(16.0);

        randomize_button(ui, self);
    }
}
