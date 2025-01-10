use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::binary_to_text::intel_hex::IntelHex;
use egui::DragValue;

pub struct IntelHexFrame {
    code: IntelHex,
}

impl Default for IntelHexFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for IntelHexFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/codes/src/binary_to_text/intel_hex.rs",
        );
        ui.add_space(8.0);
        ui.binary_to_text_input_mode(&mut self.code.mode);

        ui.add_space(8.0);
        ui.subheading("Starting Address");
        ui.add(DragValue::new(&mut self.code.address));

        ui.add_space(8.0);
        ui.subheading("Maximum Line Length");
        ui.add(DragValue::new(&mut self.code.line_length).range(1..=255));

        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
