use super::{View, ViewableCode};
use crate::codes::Baudot;

impl ViewableCode for Baudot {}

impl View for Baudot {
    fn ui(&mut self, _ui: &mut eframe::egui::Ui, _errors: &mut String) {
        // let nrows = 16;
        // let ncols = 2;
        // ui.columns(ncols, |columns| {
        //     let mut ctr = 0;
        //     let mut col = 0;
        //     for (c, code) in self.letters_codes() {
        //         let pair = format!("{}  {} ", c, code);
        //         // if mono_button(&mut columns[col], &pair).clicked() {
        //         //     input.push(c)
        //         // }
        //         ctr += 1;
        //         if ctr % nrows == 0 {
        //             col += 1
        //         }
        //     }
        // });
    }
}
