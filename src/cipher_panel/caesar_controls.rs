use eframe::egui::Slider;
use super::View;
use super::generic_components::*;
use crate::ciphers::Caesar;

impl View for Caesar {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, input: &mut String, output: &mut String) {
        ui.add_space(16.0);
        input_alphabet(ui, self);
        ui.add_space(16.0);

        ui.label("Key");
        let alpha_range = 0..=((self.length()-1));
        ui.add(Slider::new(&mut self.shift, alpha_range));
        ui.add_space(16.0);

        encrypt_decrypt(ui, self, input, output);
        ui.add_space(16.0);
        randomize_button(ui, self);
        ui.add_space(16.0);
        clear_button(ui, input, output);
    }
}
