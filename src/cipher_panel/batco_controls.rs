use super::{View, ViewableCipher, _generic_components::*};
use crate::{ciphers::tactical::Batco, egui_aux::mono};
use eframe::egui::{Slider, Ui};

impl ViewableCipher for Batco {}

impl View for Batco {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Message Key");

        ui.horizontal(|ui| {
            ui.monospace(format!("{}", self.message_number_to_char()));
            ui.add(
                Slider::new(&mut self.message_number, 0..=5)
                    .clamp_to_range(true)
                    .show_value(false),
            );
        });

        ui.horizontal(|ui| {
            mono(ui, &format!("{}", self.message_letter_to_char()), None);
            ui.add(
                Slider::new(&mut self.message_letter, 0..=25)
                    .clamp_to_range(true)
                    .show_value(false),
            );
        });

        ui.add_space(16.0);

        mono(ui, &self.show_code_page(), None);
    }
}
