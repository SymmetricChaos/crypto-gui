use codes::text_standards::baudot::Baudot;

use crate::ui_elements::UiElements;

use super::CodeFrame;

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

        ui.label("The second International Telegraphy Alphabet (ITA2) was based on work by Baudot and Murray. It became a widely used standard that made ITA1 obsolete. This variant is still in minor usage and is the origin of the 'baud' as a five-bit unit of information.");
        ui.add_space(8.0);
        ui.checkbox(&mut self.code.spaced, "Print as Groups of Five");
        ui.add_space(8.0);

        ui.fill_code_columns(16, 4, self.code.codes_chars());
        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
