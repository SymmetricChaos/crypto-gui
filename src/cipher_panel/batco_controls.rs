use super::{generic_components::*, View};
use crate::{
    ciphers::{Batco, Cipher},
    egui_aux::mono,
};
use eframe::egui::{Slider, Ui};
use rand::prelude::StdRng;

impl View for Batco {
    fn ui(&mut self, ui: &mut Ui, rng: &mut StdRng, errors: &mut String) {
        randomize_reset(ui, self, rng);
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

        if ui.button("Use Seed").clicked() {
            if self.seed.is_none() {
                self.seed = Some(0)
            } else {
                self.seed = None
            }
        }

        if self.seed.is_some() {
            if ui.text_edit_singleline(&mut self.seed_string).changed() {
                let r = self.seed_string_to_seed();
                if r.is_err() {
                    errors.push_str(&r.unwrap_err().to_string())
                } else {
                    self.randomize(rng);
                    errors.clear()
                }
            };
        }

        mono(ui, &self.show_code_page(), None);
    }
}
