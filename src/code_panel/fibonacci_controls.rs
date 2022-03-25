use eframe::egui::{Grid};
use super::View;
use crate::{codes::FibonacciCode, egui_aux::mono_strong};

const NUM_ROWS: usize = 3;

impl View for FibonacciCode {
    fn ui(&mut self, ui: &mut eframe::egui::Ui) {
        //ui.add(TextEdit::singleline(self.control_alphabet()));
        ui.label("ETAOINSHRDLCUMWFGYPBVKJXQZ");
        Grid::new("fib_code_grid").num_columns(NUM_ROWS).show(ui, |ui| {
            let mut ctr = 0;
            for (c, code) in self.chars_codes() {
                let pair = format!("{}  {}     ", c, code);
                mono_strong(ui,&pair, None);
                ctr += 1;
                if ctr % NUM_ROWS == 0 {
                    ui.end_row()
                }
            }
        });
    }
}
