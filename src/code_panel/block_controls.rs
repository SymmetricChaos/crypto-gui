use super::{generic_components::fill_code_columns, CodeFrame};
use crate::egui_aux::subheading;
use codes::block::BlockCode;
use egui::TextEdit;
use itertools::Itertools;
use utils::preset_alphabet::PresetAlphabet;

pub struct BlockCodeFrame {
    code: BlockCode,
    alphabet_string: String,
    symbol_string: String,
}

impl Default for BlockCodeFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
            alphabet_string: String::from(PresetAlphabet::BasicLatin),
            symbol_string: String::from("01"),
        }
    }
}

impl CodeFrame for BlockCodeFrame {
    fn ui(&mut self, ui: &mut egui::Ui, errors: &mut String) {
        ui.label(subheading("Alphabet"));
        if ui
            .add(TextEdit::singleline(&mut self.alphabet_string))
            .changed()
        {
            self.code.alphabet = self.alphabet_string.chars().collect_vec()
        };
        ui.add_space(16.0);

        ui.label(subheading("Symbols"));
        if ui
            .add(TextEdit::singleline(&mut self.symbol_string))
            .changed()
        {
            self.code.symbols = self.symbol_string.chars().collect_vec()
        };
        ui.add_space(16.0);

        ui.horizontal(|ui| {
            if ui.small_button("-").clicked() {
                if let Err(e) = self.code.decrease_width() {
                    *errors = e.to_string();
                } else {
                    errors.clear()
                }
            }
            ui.label(self.code.width.to_string());
            if ui.small_button("+").clicked() {
                if let Err(e) = self.code.increase_width() {
                    *errors = e.to_string();
                } else {
                    errors.clear()
                }
            }
        });
        ui.add_space(16.0);
        fill_code_columns(24, 6, ui, self.code.chars_codes());
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
