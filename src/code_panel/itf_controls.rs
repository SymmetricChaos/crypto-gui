use codes::{commercial::itf::Itf, traits::Code};

use crate::ui_elements::UiElements;

use super::CodeFrame;

pub struct ItfFrame {
    pub code: Itf,
    pub example: String,
}

impl Default for ItfFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
            example: String::from("123"),
        }
    }
}

impl CodeFrame for ItfFrame {
    fn ui(&mut self, ui: &mut eframe::egui::Ui) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/codes/src/commercial/itf.rs",
        );
        ui.add_space(8.0);

        ui.checkbox(&mut self.code.insert_zero, "Automatically Insert Zero");

        ui.subheading("Example");
        ui.text_edit_singleline(&mut self.example);
        match self.code.encode(&self.example) {
            Ok(bits) => ui.mono(bits),
            Err(e) => ui.error_text(e),
        };
        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
