use codes::{tap_code::TapCode, text_utils::PresetAlphabet};

use crate::{
    cipher_panel::_generic_components::control_string,
    egui_aux::{mono, subheading},
};

use super::CodeFrame;

pub struct TapCodeFrame {
    code: TapCode,
    alphabet_string: String,
}

impl Default for TapCodeFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
            alphabet_string: String::from(PresetAlphabet::BasicLatinNoC),
        }
    }
}

impl CodeFrame for TapCodeFrame {
    fn ui(&mut self, ui: &mut egui::Ui, errors: &mut String) {
        ui.label(subheading("Common Latin Alphabets"));
        ui.horizontal(|ui| {
            if ui.button("No C").clicked() {
                self.code.assign_alphabet(PresetAlphabet::BasicLatinNoC)
            };
            if ui.button("No J").clicked() {
                self.code.assign_alphabet(PresetAlphabet::BasicLatinNoJ)
            };
            if ui.button("No Q").clicked() {
                self.code.assign_alphabet(PresetAlphabet::BasicLatinNoQ)
            };
        });

        ui.label(subheading("Alphabet"));
        if control_string(ui, &mut self.alphabet_string).changed() {
            match self.code.set_alphabet(&self.alphabet_string) {
                Ok(_) => (),
                Err(e) => *errors = e.to_string(),
            }
        }
        ui.add_space(16.0);

        ui.label("Grid");
        mono(ui, &self.code.show_grid(), None);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
