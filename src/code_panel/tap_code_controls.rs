use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::other::tap_code::TapCode;
use rand::thread_rng;
use utils::{
    preset_alphabet::Alphabet,
    text_functions::{shuffled_str, unique_string},
};

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
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.subheading("Common Alphabets");
            ui.horizontal(|ui| {
                for alphabet in [
                    Alphabet::BasicLatinNoC,
                    Alphabet::BasicLatinNoJ,
                    Alphabet::BasicLatinNoQ,
                    Alphabet::Alphanumeric,
                ] {
                    if ui.button(alphabet.name()).clicked() {
                        self.alphabet_string = alphabet.into();
                        self.code.set_alphabet(&self.alphabet_string)
                    }
                }
            });
        });

        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.subheading("Alphabet");
            if ui.button("🎲").on_hover_text("shuffle").clicked() {
                self.alphabet_string = shuffled_str(&self.alphabet_string, &mut thread_rng())
            }
        });
        if ui.control_string(&mut self.alphabet_string).changed() {
            unique_string(&mut self.alphabet_string);
            while self.alphabet_string.chars().count() > 100 {
                self.alphabet_string.pop();
            }
            self.code.set_alphabet(&self.alphabet_string);
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
