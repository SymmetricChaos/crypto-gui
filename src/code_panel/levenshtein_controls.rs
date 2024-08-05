use super::CodeFrame;
use crate::ui_elements::{integer_letter_code_controls, integer_word_code_controls, UiElements};
use codes::{letter_word_code::IOMode, mathematical::levenshtein::LevenshteinCode, traits::Code};
use strum::IntoEnumIterator;

pub struct LevenshteinCodeFrame {
    code: LevenshteinCode,
    words_string: String,
}

impl Default for LevenshteinCodeFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
            words_string: String::new(),
        }
    }
}

impl CodeFrame for LevenshteinCodeFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/codes/src/mathematical/levenshtein.rs",
        );
        ui.add_space(8.0);

        ui.group(|ui| {
            ui.subheading("Mode");
            for variant in IOMode::iter() {
                ui.selectable_value(&mut self.code.mode, variant, variant.to_string());
            }
        });
        ui.add_space(8.0);
        ui.checkbox(&mut self.code.spaced, "Use Spaces");
        ui.add_space(8.0);

        match self.code.mode {
            IOMode::Letter => {
                integer_letter_code_controls(ui, &mut self.code.maps.alphabet);
                // ui.two_column_table("Character", "Code", Box::new(self.code.maps.chars_codes()));
            }
            IOMode::Word => {
                integer_word_code_controls(ui, &mut self.words_string, &mut self.code.maps);
                // ui.two_column_table("Word", "Code", Box::new(self.code.maps.words_codes()));
            }
            IOMode::Integer => {
                ui.label("Get the Levenshtein coding for any list of non-negative integers or decode any string of 0s and 1s into a list of non-negative integers. A sample list of encodings it provided below.");
                let pairs = (0..32).map(|n| (n.to_string(), self.code.integer_code.encode_u32(n)));
                ui.add_space(16.0);
                ui.two_column_table("Integer", "Code", Box::new(pairs));
            }
        }

        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn Code {
        &self.code
    }
}
