use crate::ciphers::polyalphabetic::Bazeries;

use super::{View, ViewableCipher, _generic_components::*};
use eframe::egui::{self, Slider, Ui};

impl ViewableCipher for Bazeries {}

impl View for Bazeries {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Alphabet");
        if control_string(ui, &mut self.alphabet_string).changed() {
            self.set_alphabet()
        }
        ui.add_space(16.0);

        ui.label("Offset");
        let alpha_range = 0..=(self.alphabet_len());
        ui.add(Slider::new(&mut self.offset, alpha_range.clone()));
        ui.add_space(16.0);

        ui.label("Wheels");
        for wheel in &self.wheels {
            ui.add(egui::Label::new(egui::RichText::from(wheel).monospace()));
        }

        ui.horizontal(|ui| {
            if ui.button("+").clicked() {
                self.add_wheel()
            }
            if ui.button("-").clicked() {
                self.del_wheel()
            }
        });
    }
}
