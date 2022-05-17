use std::fmt::Display;

use crate::codes::Code;
use crate::egui_aux::mono_strong;
use eframe::egui::{self, Color32, RichText};

pub fn encode_decode(
    ui: &mut egui::Ui,
    code: &dyn Code,
    input: &mut String,
    output: &mut String,
    errors: &mut String,
) {
    ui.horizontal(|ui| {
        if ui
            .button(RichText::from("ENCODE").color(Color32::GOLD))
            .clicked()
        {
            errors.clear();
            match code.encode(input) {
                Ok(text) => *output = text,
                Err(e) => *errors = e.to_string(),
            }
        };
        if ui
            .button(RichText::from("DECODE").color(Color32::GOLD))
            .clicked()
        {
            errors.clear();
            match code.decode(input) {
                Ok(text) => *output = text,
                Err(e) => *errors = e.to_string(),
            }
        }
    });
}

pub fn fill_code_columns<T: Display, S: Display>(
    nrows: usize,
    ncols: usize,
    ui: &mut egui::Ui,
    iter: Box<dyn Iterator<Item = (T, S)> + '_>,
) {
    ui.columns(ncols, |columns| {
        let mut ctr = 0;
        let mut col = 0;
        for (c, code) in iter {
            let pair = format!("{}  {} ", c, code);
            mono_strong(&mut columns[col], &pair, Some(18.0));
            ctr += 1;
            if ctr % nrows == 0 {
                col += 1
            }
        }
    });
}

pub fn code_button_columns(
    nrows: usize,
    ncols: usize,
    ui: &mut egui::Ui,
    target: &mut String,
    space: &str,
    iter: Box<dyn Iterator<Item = (char, &str)> + '_>,
) {
    ui.columns(ncols, |columns| {
        let mut ctr = 0;
        let mut col = 0;
        for (c, code) in iter {
            let pair = format!("{}  {} ", c, code);
            if columns[col].button(&pair).clicked() {
                if !target.is_empty() {
                    target.push_str(space);
                }
                target.push_str(code)
            }
            ctr += 1;
            if ctr % nrows == 0 {
                col += 1
            }
        }
    });
}
