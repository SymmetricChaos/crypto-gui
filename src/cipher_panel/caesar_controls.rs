use eframe::egui::Slider;
use super::View;
use super::generic_components::*;
use crate::{ciphers::Caesar, text_functions::LATIN};

pub struct CaesarControls {
    cipher: Caesar,
    input: String,
    output: String,
}

impl Default for CaesarControls {
    fn default() -> Self {
        Self { 
            cipher: Caesar::new(0, LATIN),
            input: String::new(),
            output: String::new(),
        }
    }
}

impl View for CaesarControls {
    fn ui(&mut self, ui: &mut eframe::egui::Ui) {
        ui.add_space(16.0);
        input_alphabet(ui, &mut self.cipher);
        ui.add_space(16.0);

        ui.label("Key");
        let alpha_range = 0..=((self.cipher.length()-1));
        ui.add(Slider::new(&mut self.cipher.shift, alpha_range));
        ui.add_space(16.0);

        encrypt_button(ui, &mut self.cipher, &mut self.input, &mut self.output);
        decrypt_button(ui, &mut self.cipher, &mut self.input, &mut self.output);
        randomize_button(ui, &mut self.cipher);
    }
}
