use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::text_standards::baudot::Baudot;

pub struct BaudotFrame {
    code: Baudot,
}

impl Default for BaudotFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for BaudotFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/codes/src/text_standards/baudot.rs",
        );
        ui.add_space(8.0);

        ui.label("An alternate decoding scheme replaces all figures and control characters with letters and numbers, making it easier to print.");
        ui.checkbox(&mut self.code.alt_decode, "Alternate Decoding");
        ui.add_space(8.0);

        ui.checkbox(&mut self.code.spaced, "Print Bits as Groups of Five");
        ui.add_space(8.0);

        ui.fill_code_columns(16, 4, self.code.codes_chars());
        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
