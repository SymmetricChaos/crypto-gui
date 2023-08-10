use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::other::block::BlockCode;
use egui::Slider;
use itertools::Itertools;
use utils::{preset_alphabet::Alphabet, text_functions::unique_string};

pub struct BlockCodeFrame {
    code: BlockCode,
    alphabet_string: String,
    symbol_string: String,
}

impl Default for BlockCodeFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
            alphabet_string: String::from(Alphabet::BasicLatin),
            symbol_string: String::from("01"),
        }
    }
}

impl CodeFrame for BlockCodeFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.subheading("Alphabet");
        ui.label("Characters to be encoded.");
        if ui.control_string(&mut self.alphabet_string).changed() {
            self.code.alphabet = self.alphabet_string.chars().collect_vec()
        }
        ui.add_space(16.0);

        ui.subheading("Code Symbols");
        ui.label("The symbols to be used in the code.");
        if ui.control_string(&mut self.symbol_string).changed() {
            unique_string(&mut self.symbol_string);
            while self.symbol_string.chars().count() > 5 {
                self.symbol_string.pop();
            }
            self.code.symbols = self.symbol_string.chars().collect_vec()
        }
        ui.add_space(16.0);

        ui.subheading("Code Width");
        ui.label("How many symbols appear in each code.");
        if ui.add(Slider::new(&mut self.code.width, 1..=8)).changed() {
            self.code.width = self.code.min_code_width();
        }
        ui.add_space(16.0);

        ui.label(format!("There are {} codes.", self.code.total_codes()));

        ui.two_column_table("Code", "Value", self.code.chars_codes());
        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
