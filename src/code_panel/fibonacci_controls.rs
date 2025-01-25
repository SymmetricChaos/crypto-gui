use super::CodeFrame;
use crate::ui_elements::{invert_bits, prefix_code_sep, UiElements};
use codes::mathematical::fibonacci::FibonacciCode;

pub struct FibonacciCodeFrame {
    code: FibonacciCode,
}

impl Default for FibonacciCodeFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for FibonacciCodeFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/codes/src/mathematical/fibonacci.rs",
        );
        ui.add_space(8.0);

        prefix_code_sep(ui, &mut self.code.spaced);

        invert_bits(ui, &mut self.code.invert);

        ui.label("A sample list of encodings:");
        let pairs = (1..=32).map(|n| {
            (
                n.to_string(),
                self.code
                    .integer_code
                    .borrow_mut()
                    .encode_u32(n)
                    .unwrap()
                    .to_owned(),
            )
        });
        ui.add_space(16.0);
        ui.two_column_table("Integer", "Code", Box::new(pairs));

        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
