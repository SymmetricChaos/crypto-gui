use eframe::egui::Slider;
use super::View;
use super::generic_components::*;
use crate::ciphers::DecoderRing;

impl View for DecoderRing {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, input: &mut String, output: &mut String, errors: &mut String) {
        ui.add_space(16.0);
        input_alphabet(ui, self);
        ui.add_space(16.0);

        ui.label("Key");
        let alpha_range = 0..=((self.length()-1));
        ui.add(Slider::new(&mut self.index, alpha_range));
        ui.add_space(16.0);

        ui.horizontal(|ui| {
            if ui.button("Annie").clicked() {
                self.annie();
            }
            if ui.button("Midnight").clicked() {
                self.midnight();
            }
        });

        encrypt_decrypt(ui, self, input, output, errors);
        ui.add_space(16.0);
        randomize_button(ui, self);
    }
}
