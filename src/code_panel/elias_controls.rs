use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::{
    letter_word_code::IOMode,
    mathematical::{elias::EliasCode, elias_integers::EliasVariant},
    traits::Code,
};
use egui::TextEdit;
use strum::IntoEnumIterator;
use utils::text_functions::unique_string;

pub struct EliasCodeFrame {
    code: EliasCode,
    words_string: String,
}

impl Default for EliasCodeFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
            words_string: String::new(),
        }
    }
}

impl CodeFrame for EliasCodeFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/codes/src/mathematical/elias.rs",
        );
        ui.add_space(8.0);

        ui.group(|ui| {
            ui.subheading("Variant");
            ui.selectable_value(
                &mut self.code.integer_code.borrow_mut().variant,
                EliasVariant::Delta,
                "Delta δ",
            );
            ui.selectable_value(
                &mut self.code.integer_code.borrow_mut().variant,
                EliasVariant::Gamma,
                "Gamma γ",
            );
            ui.selectable_value(
                &mut self.code.integer_code.borrow_mut().variant,
                EliasVariant::Omega,
                "Omega ω",
            );
        });
        ui.add_space(8.0);
        ui.checkbox(&mut self.code.spaced, "Use Spaces");
        ui.add_space(8.0);

        ui.group(|ui| {
            ui.subheading("Mode");
            for variant in IOMode::iter() {
                ui.selectable_value(&mut self.code.mode, variant, variant.to_string());
            }
        });
        ui.add_space(16.0);

        match self.code.mode {
            IOMode::Letter => {
                ui.label("Provide an alphabet. Elias codes will be assigned to each character of the alphabet in ascending order. When decoding the '�' symbol appears when a code without a known meaning is encountered.");
                if ui.control_string(&mut self.code.maps.alphabet).changed() {
                    unique_string(&mut self.code.maps.alphabet);
                    self.code.maps.alphabet.retain(|x| x != '�');
                };
                ui.add_space(16.0);
                ui.two_column_table(
                    "Character",
                    "Code",
                    Box::new(self.code.maps.alphabet.chars().zip(self.code.values())),
                );
            }
            IOMode::Word => {
                ui.label("Provide any number of words or phrases separated by commas. Elias codes will be assigned to each word or phrase in ascending order. When decoding the '�' symbol appears when a code without a known meaning is encountered.");
                if ui
                    .add(TextEdit::multiline(&mut self.words_string))
                    .changed()
                {
                    self.code.maps.set_words(&self.words_string);
                };
                ui.add_space(16.0);
                ui.two_column_table(
                    "Character",
                    "Word",
                    Box::new(self.code.maps.words.iter().zip(self.code.values())),
                );
            }
            IOMode::Integer => {
                ui.label("Get the Elias coding for any list of positive integers or decode any string of 0s and 1s into a list of positive integers. A sample list of encodings is provided below.");
                ui.add_space(16.0);
                ui.two_column_table(
                    "Integer",
                    "Code",
                    Box::new(
                        (1..33).map(|n| (n.to_string(), self.code.encode(&n.to_string()).unwrap())),
                    ),
                );
            }
        }

        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn Code {
        &self.code
    }
}
