use codes::commercial::isbn::{Isbn, IsbnVariant};

use super::CodeFrame;
use crate::egui_aux::{mono, subheading};

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

impl CodeFrame for IsbnFrame {
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

        ui.add_space(8.0);

        match self.code.variant {
            IsbnVariant::Ten => ui.label(mono(
                " 0  3  0  6  4  0  6  1  5  2 (digits)
10  9  8  7  6  5  4  3  2  1 (weights)
 0 27  0 42 24  0 24  3 10  2 (weighted values)

these values sum to 132, which is a multiple of 11, so the code is valid
                ",
            )),
            IsbnVariant::Thirteen => ui.label(mono(
                "9  7  8  0  3  0  6  4  0  6  1  5  7 (digits)
1  3  1  3  1  3  1  3  1  3  1  3  1 (weights)
9 21  8  0  3  0  6 12  0 18  1 15  7 (weighted values)

these values sum to 100, which is a multiple of 10, so the code is valid",
            )),
        };

        ui.add_space(16.0);
        ui.label("Check the validity of ISBN codes. Put in codes separated by commas.");
        ui.text_edit_multiline(&mut self.text);
        if ui.button("Check").clicked() {
            self.text = self.code.check_csv_isbn(&self.text);
        }
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
