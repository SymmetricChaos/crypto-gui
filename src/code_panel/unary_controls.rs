use super::CodeFrame;
use crate::ui_elements::UiElements;
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

impl UnaryCodeFrame {
    pub fn usize_to_unary(&self, n: usize) -> String {
        if self.code.invert {
            "0".repeat(n) + "1"
        } else {
            "1".repeat(n) + "0"
        }
    }
}

impl CodeFrame for UnaryCodeFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/codes/src/mathematical/unary.rs",
        );
        ui.add_space(8.0);

        ui.add_space(8.0);
        ui.checkbox(&mut self.code.spaced, "Use Spaces");
        ui.add_space(8.0);

        ui.subheading("Invert Bits");
        ui.label("The 0 and 1 bits can be switched to create an equivalent code.");
        ui.checkbox(&mut self.code.invert, "");
        ui.add_space(8.0);

        ui.label("Convert between numbers and their unary encodings. When decoding the '�' symbol appears when an invalid code is encountered.");
        ui.add_space(16.0);
        ui.two_column_table(
            "Integer",
            "Code",
            Box::new((0..6).into_iter().map(|n| (n, self.usize_to_unary(n)))),
        );

        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
