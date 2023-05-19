use super::{View, ViewableCode};
use crate::{
    codes::{ecc::isbn::IsbnVariant, Code, Isbn},
    egui_aux::subheading,
};

pub struct IsbnFrame {
    pub code: Isbn,
    pub text: String,
}

impl Default for IsbnFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
            text: Default::default(),
        }
    }
}

impl ViewableCode for IsbnFrame {}

impl Code for IsbnFrame {
    fn encode(&self, text: &str) -> Result<String, crate::errors::Error> {
        self.code.encode(text)
    }

    fn decode(&self, text: &str) -> Result<String, crate::errors::Error> {
        self.code.decode(text)
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {}
}

impl View for IsbnFrame {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _errors: &mut String) {
        ui.group(|ui| {
            ui.label(subheading("Variant"));
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.code.variant, IsbnVariant::Ten, "ISBN-10");
                ui.selectable_value(&mut self.code.variant, IsbnVariant::Thirteen, "ISBN-13");
            });
        });

        match self.code.variant {
            IsbnVariant::Ten => ui.label("ISBN-10 numbers consist of 9 digits and a final check digit which may be X, representing a check value of 10."),
            IsbnVariant::Thirteen => ui.label("ISBN-13 numbers consist of 12 digits and a final check digit. The prefix value 987 is reserved for ISBN-10 numbers being re-coded at ISBN-13 numbers, the final check digit is also recalculated when doing this."),
        };

        ui.add_space(16.0);
        ui.label("Check Multiple ISBN codes at once. Put in codes separated by commas.");
        ui.text_edit_multiline(&mut self.text);
        if ui.button("Check").clicked() {
            self.text = self.code.check_csv_isbn(&self.text);
        }
    }
}
