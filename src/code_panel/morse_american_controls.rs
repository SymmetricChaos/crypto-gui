use super::View;
use crate::{codes::MorseAmerican, egui_aux::mono_button};

impl View for MorseAmerican {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, input: &mut String, _output: &mut String, _errors: &mut String) {

        let nrows = 20;
        let ncols = 3;
        ui.columns(ncols, |columns| {

            let mut ctr = 0;
            let mut col = 0;
            for (c, code) in self.chars_codes() {
                let pair = format!("{}  {} ", c, code);
                if mono_button(&mut columns[col], &pair).clicked() {
                    input.push(c)
                }
                ctr += 1;
                if ctr % nrows == 0 {
                    col += 1
                }
            }
        });
        
    }
}