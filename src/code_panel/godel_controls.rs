use crate::ui_elements::UiElements;

use super::CodeFrame;

use codes::{mathematical::godel::Godel, traits::IOMode};
use egui::TextEdit;
use utils::text_functions::unique_string;

pub struct GodelFrame {
    code: Godel,
}

impl Default for GodelFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for GodelFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.subheading("Mode");
            ui.selectable_value(&mut self.code.mode, IOMode::Letter, "Letter");
            ui.selectable_value(&mut self.code.mode, IOMode::Word, "Word");
        });
        ui.add_space(8.0);

        match self.code.mode {
            IOMode::Letter => {
                ui.label("Provide an alphabet. Codes will be assigned to each character of the alphabet in ascending order. When decoding the '�' symbol appears when a code without a known meaning is assigned.");
                if ui.control_string(&mut self.code.maps.alphabet).changed() {
                    unique_string(&mut self.code.maps.alphabet);
                    self.code.maps.alphabet.retain(|x| x != '�');
                    self.code.set_letter_map();
                };
                ui.add_space(16.0);
                ui.two_column_table("Code", "Character", Box::new(self.code.maps.codes_chars()));
            }
            IOMode::Word => {
                ui.label("Provide any number of words or phrases separated by commas. Codes will be assigned to each word or phrase in ascending order. When decoding the '�' symbol appears when a code without a known meaning is assigned.");
                if ui
                    .add(TextEdit::multiline(&mut self.code.maps.words_string))
                    .changed()
                {
                    self.code.set_word_map();
                };
                ui.add_space(16.0);
                ui.two_column_table("Code", "Word", Box::new(self.code.maps.codes_words()));
            }
            IOMode::Integer => {
                ui.label("<<<ERROR INTEGER MODE IS NOT DEFINED FOR GODEL CODE>>>");
            }
        }
        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
