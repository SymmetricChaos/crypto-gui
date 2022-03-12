use eframe::egui::{self, RichText, Color32};
use crate::codes::Code;

pub fn encode_decode(ui: &mut egui::Ui, code: &dyn Code, input: &mut String, output: &mut String, errors: &mut String) {
    ui.horizontal(|ui| {
        if ui.button(RichText::from("ENCODE").color(Color32::GOLD)).clicked() {
            errors.clear();
            match code.encode(input) {
                Ok(text) => *output = text,
                Err(e) => *errors = e.to_string(),
            }
        };
        if ui.button(RichText::from("DECODE").color(Color32::GOLD)).clicked() {
            errors.clear();
            match code.decode(input) {
                Ok(text) => *output = text,
                Err(e) => *errors = e.to_string(),
            }
        }
    });
}