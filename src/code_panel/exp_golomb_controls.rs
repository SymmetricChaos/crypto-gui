use super::CodeFrame;
use crate::ui_elements::{invert_bits, prefix_code_sep, signed, UiElements};
use codes::mathematical::exp_golomb::{i32_to_exp_golomb, u32_to_exp_golomb, ExpGolomb};

pub struct ExpGolombFrame {
    code: ExpGolomb,
}

impl Default for ExpGolombFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl ExpGolombFrame {}

impl CodeFrame for ExpGolombFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/codes/src/mathematical/exp_golomb.rs",
        );
        ui.add_space(8.0);

        prefix_code_sep(ui, &mut self.code.spaced);

        signed(ui, &mut self.code.signed);

        invert_bits(ui, &mut self.code.invert);

        ui.label("A sample list of encodings:");
        if self.code.signed {
            ui.two_column_table(
                "Code",
                "Integer",
                Box::new((-8..=8).into_iter().map(|n| (n, i32_to_exp_golomb(n)))),
            );
        } else {
            ui.two_column_table(
                "Code",
                "Integer",
                Box::new((0..=16).into_iter().map(|n| (n, u32_to_exp_golomb(n)))),
            );
        }

        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
