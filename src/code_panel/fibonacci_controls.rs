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

        ui.subheading("Separated");
        ui.label("A prefix code can be read without inserting spaces or commas. With this set the output will be comma separated.");
        ui.checkbox(&mut self.code.spaced, "Use Separator");
        ui.add_space(8.0);

        ui.subheading("Invert Bits");
        ui.label("The 0 and 1 bits can be inverted.");
        ui.checkbox(&mut self.code.invert, "Use Inverted");
        ui.add_space(8.0);

        ui.label("A sample list of encodings:");
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
