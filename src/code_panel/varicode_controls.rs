use codes::text_standards::varicode::Varicode;

use crate::{code_panel::CodeFrame, ui_elements::UiElements};

pub struct VaricodeFrame {
    code: Varicode,
}

impl Default for VaricodeFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for VaricodeFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/codes/src/text_standards/varicode.rs",
        );
        ui.add_space(8.0);

        ui.add_space(8.0);
        ui.checkbox(&mut self.code.spaced, "Use Spaces");
        ui.add_space(16.0);
        ui.two_column_table("Character", "Code", self.code.chars_codes_display());
        ui.add_space(32.0)
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
