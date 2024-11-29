use super::CodeFrame;
use crate::ui_elements::UiElements;
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

        ui.checkbox(&mut self.code.spaced, "Use Spaces");
        ui.add_space(8.0);

        ui.label("Get the Fibonacci coding for any list of positive integers or decode any string of 0s and 1s into a list of positive integers. A sample list of encodings is provided below.");
        let pairs = (1..=64).map(|n| {
            (
                n.to_string(),
                self.code.integer_code.borrow_mut().encode_u32(n).to_owned(),
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
