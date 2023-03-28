use super::{View, ViewableCode};
use crate::{
    codes::{unicode::UnicodeEncoding, Unicode},
    text_aux::bytes_as_text::NumRep,
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
            ui.selectable_value(&mut self.mode, NumRep::Binary, "Binary");
            ui.selectable_value(&mut self.mode, NumRep::Octal, "Octal");
            ui.selectable_value(&mut self.mode, NumRep::Decimal, "Decimal");
            ui.selectable_value(&mut self.mode, NumRep::HexLower, "Hexadecimal");
            ui.selectable_value(
                &mut self.mode,
                NumRep::HexUpper,
                "Hexadecimal (capitalized)",
            );
        });
    }
}
