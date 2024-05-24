use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::text_standards::linotype::Linotype;

pub struct LinotypeFrame {
    code: Linotype,
}

impl Default for LinotypeFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for LinotypeFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/codes/src/text_standards/linotype.rs",
        );
        ui.add_space(8.0);

        ui.label("Basic Linotype characters. The machines could be expanded with additional magazines of characters.");
        ui.fill_code_columns(32, 4, self.code.chars_codes());
        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
