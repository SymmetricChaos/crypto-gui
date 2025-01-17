use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::mathematical::factoradic::{encode_u64, Factoradic};

pub struct FactoradicFrame {
    code: Factoradic,
}

impl Default for FactoradicFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for FactoradicFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/codes/src/mathematical/factoradic.rs",
        );
        ui.add_space(8.0);

        ui.subheading("Example Calculation");
        ui.mono(
            "factoradic number     3:4:2:2:1:0\n\ndigits                3   4  2  2  1  0\nplace values       ×120 ×24 ×6 ×2 ×1 ×1\ndigit values        360  96 12  4  1  0\n\ntheir sum is 473",
        );

        ui.add_space(16.0);

        ui.subheading("Small Example Values");
        ui.two_column_table(
            "Integer",
            "Code",
            Box::new((0..10).into_iter().map(|n| (n, encode_u64(n)))),
        );

        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
