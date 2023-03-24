use super::{View, ViewableCode};
use crate::codes::{
    unicode::{DisplayMode, UnicodeEncoding},
    Unicode,
};

impl ViewableCode for Unicode {}

impl View for Unicode {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _errors: &mut String) {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.encoding, UnicodeEncoding::Utf8, "UTF-8");
            ui.selectable_value(&mut self.encoding, UnicodeEncoding::Utf16, "UTF-16");
            ui.selectable_value(&mut self.encoding, UnicodeEncoding::Utf32, "UTF-32");
        });

        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.mode, DisplayMode::Binary, "Binary");
            ui.selectable_value(&mut self.mode, DisplayMode::Decimal, "Decimal");
            ui.selectable_value(&mut self.mode, DisplayMode::Hex, "Hexadecimal");
        });
    }
}
