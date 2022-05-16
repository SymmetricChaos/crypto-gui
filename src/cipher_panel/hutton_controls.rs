use super::{generic_components::*, View};
use crate::{ciphers::{Hutton, hutton::HuttonVer}, egui_aux::mono};
use eframe::egui::Ui;
use rand::prelude::StdRng;

impl View for Hutton {
    fn ui(&mut self, ui: &mut Ui, rng: &mut StdRng, _errors: &mut String) {
        randomize_reset(ui, self, rng);
        ui.add_space(16.0);

        ui.label("Alphabet");
        if control_string(ui, &mut self.alphabet_string).changed() {
            self.set_alphabet()
        }

        ui.add_space(16.0);
        ui.label("Select Version");
        ui.horizontal(|ui| {
            if ui.button("V1").clicked() {
                self.version = HuttonVer::V1
            };
            if ui.button("V2").clicked() {
                self.version = HuttonVer::V2
            };
        });

        ui.add_space(16.0);
        ui.label("Password");
        if control_string(ui, &mut self.password_string).changed() {
            self.set_password()
        }

        ui.add_space(16.0);
        ui.label("Key Word");
        if control_string(ui, &mut self.key_string).changed() {
            self.set_key()
        }

        ui.add_space(8.0);
        mono(ui, &self.keyed_alphabet(), None);
    }
}
