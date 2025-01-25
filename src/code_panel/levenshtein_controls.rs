use super::CodeFrame;
use crate::ui_elements::{prefix_code_sep, signed, UiElements};
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

        ui.subheading("Signed");
        ui.label("The Levenshtein codes can be extended to all integers by assigning negative integer to odd values and all others to even values.");
        ui.checkbox(&mut self.code.signed, "Use Signed");
        ui.add_space(8.0);

        prefix_code_sep(ui, &mut self.code.spaced);

        signed(ui, &mut self.code.signed);

        // invert_bits(ui, &mut self.code.invert);

        ui.label("A sample list of encodings:");
        ui.add_space(4.0);
        if self.code.signed {
            ui.two_column_table(
                "Integer",
                "Code",
                Box::new((-5..=5).into_iter().map(|n| (n, self.code.encode_i32(n)))),
            );
        } else {
            ui.two_column_table(
                "Integer",
                "Code",
                Box::new((0..=9).into_iter().map(|n| (n, self.code.encode_u32(n)))),
            );
        }

        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn Code {
        &self.code
    }
}
