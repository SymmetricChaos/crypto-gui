pub(crate) use crate::ui_elements::fill_code_columns;
use crate::ui_elements::subheading;

use super::CodeFrame;
use codes::{
    mathematical::{elias::EliasCode, elias_integers::EliasVariant},
    traits::{Code, IOMode},
};
use egui::TextEdit;

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
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.group(|ui| {
            ui.label(subheading("Variant"));
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
            ui.label(subheading("Mode"));
            ui.selectable_value(&mut self.code.mode, IOMode::Integer, "Integer");
            ui.selectable_value(&mut self.code.mode, IOMode::Letter, "Letter");
            ui.selectable_value(&mut self.code.mode, IOMode::Word, "Word");
        });

        ui.add_space(16.0);

        match self.code.mode {
            IOMode::Letter => {
                ui.label("Alphabetical Mode: Provide an alphabet. Elias codes will be assigned to each character of the alphabet in ascending order. When decoding the '�' symbol appears when a code without a known meaning is encountered.");
                if ui
                    .add(TextEdit::singleline(&mut self.code.maps.alphabet))
                    .changed()
                {
                    self.code.set_letter_map();
                };
                fill_code_columns(16, 5, ui, Box::new(self.code.maps.chars_codes()));
            }
            IOMode::Word => {
                ui.label("Word Mode: Provide any number of words or phrases separated by commas. Elias codes will be assigned to each word or phrase in ascending order. When decoding the '�' symbol appears when a code without a known meaning is encountered.");
                if ui
                    .add(TextEdit::singleline(&mut self.code.maps.words_string))
                    .changed()
                {
                    self.code.set_word_map();
                };
                fill_code_columns(16, 5, ui, Box::new(self.code.maps.words_codes()));
            }
            IOMode::Integer => {
                ui.label("Integer Mode: Get the Elias coding for any list of positive integers or decode any string of 0s and 1s into a list of positive integers. A sample list of encodings it provided below.");
                let pairs = (1..33).map(|n| (n.to_string(), self.code.integer_code.encode_u32(n)));
                fill_code_columns(16, 5, ui, Box::new(pairs));
            }
        }
    }

    fn code(&self) -> &dyn Code {
        &self.code
    }
}