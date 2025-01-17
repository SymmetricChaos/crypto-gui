use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::mathematical::symmetric_unary::SymmetricUnaryCode;

pub struct SymUnaryCodeFrame {
    code: SymmetricUnaryCode,
}

impl Default for SymUnaryCodeFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl SymUnaryCodeFrame {
    pub fn usize_to_unary(&self, n: usize) -> String {
        if self.code.invert {
            if n == 0 {
                return String::from("0");
            } else {
                format!("1{}1", "0".repeat(n - 1))
            }
        } else {
            if n == 0 {
                return String::from("1");
            } else {
                format!("0{}0", "1".repeat(n - 1))
            }
        }
    }
}

impl CodeFrame for SymUnaryCodeFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/codes/src/mathematical/symmetric_unary.rs",
        );
        ui.add_space(8.0);

        ui.add_space(8.0);
        ui.checkbox(&mut self.code.spaced, "Use Spaces");
        ui.add_space(8.0);

        ui.subheading("Invert Bits");
        ui.label("The 0 and 1 bits can be switched to create an equivalent code.");
        ui.checkbox(&mut self.code.invert, "");
        ui.add_space(8.0);

        ui.label("Convert between numbers and their symmetric unary encodings.");
        ui.add_space(16.0);
        ui.two_column_table(
            "Code",
            "Integer",
            Box::new((0..6).into_iter().map(|n| (n, self.usize_to_unary(n)))),
        );

        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
