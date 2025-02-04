use super::CodeFrame;
use crate::ui_elements::{invert_bits, prefix_code_sep, signed, UiElements};
use codes::mathematical::unary::UnaryCode;

pub struct UnaryCodeFrame {
    code: UnaryCode,
}

impl Default for UnaryCodeFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl UnaryCodeFrame {}

impl CodeFrame for UnaryCodeFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/codes/src/mathematical/unary.rs",
        );
        ui.add_space(8.0);

        ui.subheading("Symmetric");
        ui.label("The symmetric unary code is a variation that can be read in either direction.");
        ui.checkbox(&mut self.code.symmetric, "Use Symmetric");
        ui.add_space(8.0);

        signed(ui, &mut self.code.signed);

        prefix_code_sep(ui, &mut self.code.spaced);

        invert_bits(ui, &mut self.code.invert);

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

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
