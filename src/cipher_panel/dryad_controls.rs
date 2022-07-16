use super::{View, ViewableCipher, _generic_components::*};
use crate::{ciphers::tactical::Dryad, egui_aux::mono};
use eframe::egui::{Slider, Ui};

impl ViewableCipher for Dryad {}

impl View for Dryad {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Message Key");
        ui.horizontal(|ui| {
            mono(ui, &format!("{}", self.message_key_to_char()), None);
            ui.add(
                Slider::new(&mut self.message_key, 0..=24)
                    .clamp_to_range(true)
                    .show_value(false),
            );
        });

        ui.add_space(16.0);

        mono(ui, &self.show_code_page(), None);
    }
}
