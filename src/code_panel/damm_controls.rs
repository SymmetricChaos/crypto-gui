use crate::ui_elements::UiElements;

use super::CodeFrame;
use codes::ecc::damm::{Damm, DAMM_TABLE};

pub struct DammFrame {
    pub code: Damm,
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
    fn ui(&mut self, ui: &mut eframe::egui::Ui) {
        ui.add_space(16.0);

        ui.label("The Cayley table chosen by Damm. Note that it forms a Latin square, no row or column includes any digit more than once.");
        egui::Grid::new("damm_grid")
            .num_columns(10)
            .min_col_width(5.0)
            .max_col_width(5.0)
            .striped(true)
            .show(ui, |ui| {
                ui.label(" ");
                for digit in 0..10 {
                    ui.mono_strong(digit);
                }
                ui.end_row();
                for (n, row) in DAMM_TABLE.iter().enumerate() {
                    ui.mono_strong(n);
                    for digit in row {
                        ui.mono(digit);
                    }
                    ui.end_row();
                }
            });

        ui.label("The algorithm starts with a check value of 0. Then for each digit in the number the current check digit is used to select a row and the digit is used to select a column. The resulting value becomes the new check digit. The final value is the check digit. When decoding the same process is used and if the final value is 0 the code is valid.\nBecause the principle diagonal is all zeroes a code may be prepended with any number of zeroes without changing its check digit.");
        ui.add_space(16.0);

        ui.label("Check the validity of Damm codes. Put in codes separated by commas.");
        ui.text_edit_multiline(&mut self.text);
        if ui.button("Check").clicked() {
            self.text = self.code.check_csv_damm(&self.text);
        }
        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
