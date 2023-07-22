use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::other::tap_code::TapCode;
use utils::preset_alphabet::Alphabet;

pub struct TapCodeFrame {
    code: TapCode,
    alphabet_string: String,
}

impl Default for TapCodeFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
            alphabet_string: String::from(Alphabet::BasicLatinNoC),
        }
    }
}

impl CodeFrame for TapCodeFrame {
    fn ui(&mut self, ui: &mut egui::Ui, errors: &mut String) {
        ui.subheading("Common Alphabets");
        ui.horizontal(|ui| {
            if ui.button("No C").clicked() {
                self.code.assign_alphabet(Alphabet::BasicLatinNoC)
            };
            if ui.button("No J").clicked() {
                self.code.assign_alphabet(Alphabet::BasicLatinNoJ)
            };
            if ui.button("No Q").clicked() {
                self.code.assign_alphabet(Alphabet::BasicLatinNoQ)
            };
        });

        ui.subheading("Alphabet");
        if ui.control_string(&mut self.alphabet_string).changed() {
            match self.code.set_alphabet(&self.alphabet_string) {
                Ok(_) => (),
                Err(e) => *errors = e.to_string(),
            }
        }
        ui.add_space(16.0);

        ui.subheading("Grid");
        ui.mono(self.code.show_grid());
        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
