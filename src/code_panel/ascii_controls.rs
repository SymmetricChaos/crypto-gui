use super::{View, ViewableCode};
use crate::{
    codes::{
        ascii::AsciiMode::{EightBit, SevenBit},
        Ascii,
    },
    egui_aux::mono_button,
};

impl ViewableCode for Ascii {}

impl View for Ascii {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _errors: &mut String) {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.mode, EightBit, "8-Bit");
            ui.selectable_value(&mut self.mode, SevenBit, "7-Bit");
        });
        let nrows = 32;
        let ncols = 4;
        ui.columns(ncols, |columns| {
            let mut ctr = 0;
            let mut col = 0;
            for (c, code) in self.chars_codes() {
                let pair = format!("{}  {} ", c, code);
                // if mono_button(&mut columns[col], &pair).clicked() {
                //     input.push(c)
                // }
                ctr += 1;
                if ctr % nrows == 0 {
                    col += 1
                }
            }
        });
    }
}
