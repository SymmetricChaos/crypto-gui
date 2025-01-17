use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::{mathematical::levenshtein::LevenshteinCode, traits::Code};

pub struct LevenshteinCodeFrame {
    code: LevenshteinCode,
}

impl Default for LevenshteinCodeFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
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

        ui.checkbox(&mut self.code.spaced, "Use Spaces");
        ui.add_space(8.0);

        ui.label("A sample list of encodings:");
        let pairs = (0..32).map(|n| (n.to_string(), self.code.integer_code.encode_u32(n)));
        ui.add_space(16.0);
        ui.two_column_table("Integer", "Code", Box::new(pairs));

        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn Code {
        &self.code
    }
}
