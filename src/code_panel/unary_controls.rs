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

        ui.subheading("Separated");
        ui.label("A prefix code can be read without inserting spaces or commas. With this set the output will be comma separated.");
        ui.checkbox(&mut self.code.spaced, "Use Separator");
        ui.add_space(8.0);

        ui.subheading("Invert Bits");
        ui.label("The 0 and 1 bits can be inverted.");
        ui.checkbox(&mut self.code.invert, "Use Inverted");
        ui.add_space(8.0);

        ui.label("A sample list of encodings:");
        ui.add_space(4.0);
        ui.two_column_table(
            "Integer",
            "Code",
            Box::new((0..=9).into_iter().map(|n| (n, self.code.encode_usize(n)))),
        );

        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
