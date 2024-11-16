use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::mathematical::primorial::{encode_u64, Primorial};

pub struct PrimorialFrame {
    code: Primorial,
}

impl Default for PrimorialFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for PrimorialFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/codes/src/mathematical/primorial.rs",
        );
        ui.add_space(8.0);

        ui.subheading("Example Calculation");
        ui.mono(
            "primorial number     10:0:4:1:1\n\ndigits                10   0  4  1  1\nplace values       ×210 ×30 ×6 ×2 ×1\ndigit values        2100  0 24  2  1\n\ntheir sum is 2127",
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
