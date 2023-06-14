use super::CodeFrame;
use codes::ecc::{damm::DAMM_TABLE, verhoeff::VerhoeffAlgorithm};

pub struct DammFrame {
    pub code: VerhoeffAlgorithm,
    pub text: String,
}

impl Default for DammFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
            text: Default::default(),
        }
    }
}

impl CodeFrame for DammFrame {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _errors: &mut String) {
        ui.add_space(16.0);
        ui.label("Check the validity of Damm codes. Put in codes separated by commas.");
        ui.text_edit_multiline(&mut self.text);
        if ui.button("Check").clicked() {
            self.text = self.code.check_csv_verhoeff(&self.text);
        }

        // ui.label(format!("{:?}", DAMM_TABLE[0]));
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
