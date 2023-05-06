use eframe::egui::Ui;

use crate::{
    cipher_panel::_generic_components::control_string,
    codes::TapCode,
    egui_aux::{mono, subheading},
    text_aux::PresetAlphabet::*,
};

use super::{View, ViewableCode};

impl ViewableCode for TapCode {}

impl View for TapCode {
    fn ui(&mut self, ui: &mut Ui, errors: &mut String) {
        ui.label(subheading("Common Latin Alphabets"));
        ui.horizontal(|ui| {
            if ui.button("No C").clicked() {
                self.assign_alphabet(BasicLatinNoC)
            };
            if ui.button("No J").clicked() {
                self.assign_alphabet(BasicLatinNoJ)
            };
            if ui.button("No Q").clicked() {
                self.assign_alphabet(BasicLatinNoQ)
            };
        });

        ui.label(subheading("Alphabet"));
        if control_string(ui, &mut self.alphabet_string).changed() {
            match self.set_alphabet() {
                Ok(_) => (),
                Err(e) => *errors = e.to_string(),
            }
        }
        ui.add_space(16.0);

        ui.label("Grid");
        mono(ui, &self.show_grid(), None);
    }
}
