use super::CodeFrame;
use codes::compression::run_length::RunLengthEncoding;

pub struct RleFrame {
    code: RunLengthEncoding,
}

impl Default for RleFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for RleFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/codes/src/compression/run_length.rs",
        );

        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
