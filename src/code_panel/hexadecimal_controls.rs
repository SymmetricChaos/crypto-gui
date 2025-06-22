use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::binary_to_text::hexadecimal::Hexadecimal;

pub struct Base16Frame {
    code: Hexadecimal,
}

impl Default for Base16Frame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for Base16Frame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/codes/src/binary_to_text/base16.rs",
        );
        ui.add_space(8.0);
        ui.binary_to_text_input_mode(&mut self.code.mode);

        ui.add_space(8.0);
        ui.checkbox(&mut self.code.upper, "Uppercase");

        if self.code.upper {
            ui.fill_code_columns(8, 4, Box::new((0..16).zip("0123456789ABCDEF".chars())));
        } else {
            ui.fill_code_columns(8, 4, Box::new((0..16).zip("0123456789abcdef".chars())));
        }

        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
