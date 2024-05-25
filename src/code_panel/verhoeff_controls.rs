use super::CodeFrame;
use codes::ecc::verhoeff::VerhoeffAlgorithm;

pub struct VerhoeffFrame {
    pub code: VerhoeffAlgorithm,
    pub text: String,
}

impl Default for VerhoeffFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
            text: Default::default(),
        }
    }
}

impl CodeFrame for VerhoeffFrame {
    fn ui(&mut self, ui: &mut eframe::egui::Ui) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/codes/src/ecc/verhoeff.rs",
        );
        ui.add_space(8.0);

        ui.label("Check the validity of Verhoeff codes. Put in codes separated by commas.");
        ui.text_edit_multiline(&mut self.text);
        if ui.button("Check").clicked() {
            self.text = self.code.check_csv_verhoeff(&self.text);
        }
        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
