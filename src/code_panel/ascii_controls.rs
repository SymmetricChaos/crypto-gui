use super::{generic_components::fill_code_columns, CodeFrame};
use crate::egui_aux::subheading;
use codes::ascii::{Ascii, DisplayMode};

pub struct AsciiFrame {
    code: Ascii,
}

impl Default for AsciiFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for AsciiFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.group(|ui| {
            ui.label(subheading("Representation"));
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.code.mode, DisplayMode::EightBitBinary, "8-Bit");
                ui.selectable_value(&mut self.code.mode, DisplayMode::SevenBitBinary, "7-Bit");
                ui.selectable_value(&mut self.code.mode, DisplayMode::Octal, "Octal");
                ui.selectable_value(&mut self.code.mode, DisplayMode::Decimal, "Decimal");
                ui.selectable_value(&mut self.code.mode, DisplayMode::Hex, "Hexadecimal");
            });
        });

        fill_code_columns(32, 4, ui, self.code.chars_codes_display());
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
