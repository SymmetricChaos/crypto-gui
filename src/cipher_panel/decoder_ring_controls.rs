use eframe::egui::Slider;
use super::View;
use super::generic_components::*;
use crate::{ciphers::DecoderRing, text_functions::LATIN_UPPER};

pub struct DecoderRingControls {
    cipher: DecoderRing,
}

impl Default for DecoderRingControls {
    fn default() -> Self {
        Self { 
            cipher: DecoderRing::new(0, LATIN_UPPER),
        }
    }
}

impl View for DecoderRingControls {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, input: &mut String, output: &mut String) {
        ui.add_space(16.0);
        input_alphabet(ui, &mut self.cipher);
        ui.add_space(16.0);

        ui.label("Key");
        let alpha_range = 0..=((self.cipher.length()-1));
        ui.add(Slider::new(&mut self.cipher.index, alpha_range));
        ui.add_space(16.0);

        ui.horizontal(|ui| {
            if ui.button("Annie").clicked() {
                self.cipher.annie();
            }
            if ui.button("Midnight").clicked() {
                self.cipher.midnight();
            }
        });

        encrypt_decrypt(ui, &mut self.cipher, input, output);
        ui.add_space(16.0);
        randomize_button(ui, &mut self.cipher);
        ui.add_space(16.0);
        clear_button(ui, input, output);
    }
}
