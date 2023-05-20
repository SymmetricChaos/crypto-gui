use super::CodeFrame;
use crate::egui_aux::{error_text, subheading};
use codes::ecc::isbn::{is_valid_isbn_10, is_valid_isbn_13, Isbn, IsbnVariant};
use itertools::Itertools;

pub struct IsbnFrame {
    pub code: Isbn,
    pub text: String,
    pub example: String,
}

impl Default for IsbnFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
            text: Default::default(),
            example: String::from("1234567890128"),
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

        ui.text_edit_singleline(&mut self.example);
        match self.code.variant {
            IsbnVariant::Ten => match is_valid_isbn_10(&self.example) {
                Ok(_) => {
                    ui.label("<<<UPCOMING>>>");
                }
                Err(e) => {
                    ui.label(error_text(&e.inner()));
                }
            },
            IsbnVariant::Thirteen => match is_valid_isbn_13(&self.example) {
                Ok(_) => {
                    ui.horizontal(|ui| {
                        ui.label("Digits:  ");
                        ui.label(self.example.chars().filter(|c| *c != '-').join(" "));
                    });
                    ui.label("Weights:  1 3 1 3 1 3 1 3 1 3 1 3 1");
                    ui.horizontal(|ui| {
                        ui.label("Products: ");
                        ui.label(
                            self.example
                                .chars()
                                .filter(|c| *c != '-')
                                .map(|c| c.to_digit(10).unwrap())
                                .zip([1, 3].into_iter().cycle())
                                .map(|(a, b)| (a * b) % 10)
                                .join(" "),
                        );
                    });
                }
                Err(e) => {
                    ui.label(error_text(&e.inner()));
                }
            },
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
