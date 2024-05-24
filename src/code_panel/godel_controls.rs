use crate::ui_elements::UiElements;

use super::CodeFrame;

use codes::{letter_word_code::IOMode, mathematical::godel::Godel};
use egui::TextEdit;
use utils::text_functions::unique_string;

pub struct GodelFrame {
    code: Godel,
    words_string: String,
}

impl Default for GodelFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
            words_string: String::from("0, s, +, ×, =, (, ), implies, not, forall, exists, and, or, x1, P1, x2, P2, x3, P3, x4, P4, x5, P5")
        }
    }
}

impl CodeFrame for GodelFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/codes/src/mathematical/godel.rs",
        );
        ui.add_space(8.0);

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
                };
                ui.add_space(16.0);
                // ui.two_column_table("Code", "Character", Box::new(self.code.maps.codes_chars()));
            }
            IOMode::Word => {
                ui.label("Provide any number of words or phrases separated by commas. Codes will be assigned to each word or phrase in ascending order. When decoding the '�' symbol appears when a code without a known meaning is assigned.");
                if ui
                    .add(TextEdit::multiline(&mut self.words_string))
                    .changed()
                {
                    self.code.maps.set_words(&self.words_string);
                };
                ui.add_space(16.0);
                // ui.two_column_table("Code", "Word", Box::new(self.code.maps.codes_words()));
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
