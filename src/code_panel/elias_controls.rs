use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::{
    mathematical::{elias::EliasCode, elias_integers::EliasVariant},
    traits::{Code, IOMode},
};
use egui::TextEdit;
use utils::text_functions::unique_string;

pub struct EliasCodeFrame {
    code: EliasCode,
}

impl Default for EliasCodeFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for EliasCodeFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.subheading("Variant");
            ui.selectable_value(
                &mut self.code.integer_code.variant,
                EliasVariant::Delta,
                "Delta δ",
            );
            ui.selectable_value(
                &mut self.code.integer_code.variant,
                EliasVariant::Gamma,
                "Gamma γ",
            );
            ui.selectable_value(
                &mut self.code.integer_code.variant,
                EliasVariant::Omega,
                "Omega ω",
            );
        });

        ui.group(|ui| {
            ui.subheading("Mode");
            ui.selectable_value(&mut self.code.mode, IOMode::Integer, "Integer");
            ui.selectable_value(&mut self.code.mode, IOMode::Letter, "Letter");
            ui.selectable_value(&mut self.code.mode, IOMode::Word, "Word");
        });

        ui.add_space(16.0);

        match self.code.mode {
            IOMode::Letter => {
                ui.label("Provide an alphabet. Elias codes will be assigned to each character of the alphabet in ascending order. When decoding the '�' symbol appears when a code without a known meaning is encountered.");
                if ui.control_string(&mut self.code.maps.alphabet).changed() {
                    unique_string(&mut self.code.maps.alphabet);
                    self.code.maps.alphabet.retain(|x| x != '�');
                    self.code.set_letter_map();
                };
                ui.fill_code_columns(16, 5, Box::new(self.code.maps.chars_codes()));
            }
            IOMode::Word => {
                ui.label("Provide any number of words or phrases separated by commas. Elias codes will be assigned to each word or phrase in ascending order. When decoding the '�' symbol appears when a code without a known meaning is encountered.");
                if ui
                    .add(TextEdit::multiline(&mut self.code.maps.words_string))
                    .changed()
                {
                    self.code.set_word_map();
                };
                ui.fill_code_columns(16, 5, Box::new(self.code.maps.words_codes()));
            }
            IOMode::Integer => {
                ui.label("Get the Elias coding for any list of positive integers or decode any string of 0s and 1s into a list of positive integers. A sample list of encodings it provided below.");
                let pairs = (1..33).map(|n| (n.to_string(), self.code.integer_code.encode_u32(n)));
                ui.fill_code_columns(16, 5, Box::new(pairs));
            }
        }
        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn Code {
        &self.code
    }
}
