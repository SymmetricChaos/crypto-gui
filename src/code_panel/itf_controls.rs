use codes::{commercial::itf::Itf, traits::Code};

use crate::egui_aux::{error_text, mono};

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
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _errors: &mut String) {
        ui.checkbox(&mut self.code.insert_zero, "Automatically Insert Zero");

        ui.text_edit_singleline(&mut self.example);
        match self.code.encode(&self.example) {
            Ok(bits) => ui.label(mono(bits)),
            Err(e) => ui.label(error_text(e.inner())),
        };
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}