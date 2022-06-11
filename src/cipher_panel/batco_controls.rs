use super::{generic_components::*, View};
use crate::{ciphers::tactical::Batco, egui_aux::mono, global_rng::global_rng_controls};
use eframe::egui::{Slider, Ui};
use rand::prelude::StdRng;

impl View for Batco {
    fn ui(&mut self, ui: &mut Ui, _rng: &mut StdRng, _errors: &mut String) {
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
        global_rng_controls(ui);
        ui.add_space(16.0);

        // ui.add_space(16.0);
        // ui.horizontal(|ui| {
        //     if ui.button("Randomize from Seed").clicked() {
        //         match self.randomize_seeded() {
        //             Ok(_) => (),
        //             Err(e) => *errors = e.to_string(),
        //         }
        //     }
        //     ui.text_edit_singleline(&mut self.seed_string);
        // });

        mono(ui, &self.show_code_page(), None);
    }
}
