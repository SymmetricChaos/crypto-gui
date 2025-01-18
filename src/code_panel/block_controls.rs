use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::other::block::BlockCode;
use egui::Slider;
use itertools::Itertools;
use utils::preset_alphabet::Alphabet;

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
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/codes/src/other/block.rs",
        );
        ui.add_space(8.0);

        ui.subheading("Alphabet");
        ui.label("Characters to be encoded.");
        if ui.control_string(&mut self.alphabet_string).changed() {
            self.code.alphabet = self.alphabet_string.chars().collect_vec()
        }
        ui.add_space(16.0);

        ui.subheading("Code Symbols");
        ui.label("The symbols to be used in the code.");
        if ui.control_string(&mut self.symbol_string).changed() {
            // Restrinct the symbols in the code to be
            //    not whitespace or the comma (which are both restricted)
            //    unique
            //    no more than five
            self.symbol_string = self
                .symbol_string
                .chars()
                .filter(|c| !c.is_whitespace() && *c != ',')
                .unique()
                .take(5)
                .collect();
            self.code.symbols = self.symbol_string.chars().collect_vec()
        }
        ui.add_space(16.0);

        ui.subheading("Code Width");
        ui.label("How many symbols appear in each code.");
        if ui.add(Slider::new(&mut self.code.width, 1..=8)).changed() {
            self.code.width = self.code.min_code_width();
        }
        ui.add_space(16.0);

        ui.subheading("Separated");
        ui.label("A fixed-width code can be read without inserting spaces or commas. With this set the output will be comma separated.");
        ui.checkbox(&mut self.code.spaced, "Use Separator");
        ui.add_space(8.0);

        ui.label(format!("There are {} codes.", self.code.total_codes()));

        ui.two_column_table("Value", "Code", self.code.chars_codes());
        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
