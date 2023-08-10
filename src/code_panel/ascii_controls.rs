use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::text_standards::ascii::{Ascii, DisplayMode};

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
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.subheading("Representation");
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.code.mode, DisplayMode::EightBitBinary, "8-Bit");
                ui.selectable_value(&mut self.code.mode, DisplayMode::SevenBitBinary, "7-Bit");
                ui.selectable_value(&mut self.code.mode, DisplayMode::Octal, "Octal");
                ui.selectable_value(&mut self.code.mode, DisplayMode::Decimal, "Decimal");
                ui.selectable_value(&mut self.code.mode, DisplayMode::Hex, "Hexadecimal");
            });
        });
        ui.add_space(16.0);
        ui.two_column_table("Code", "Character", self.code.chars_codes_display());
        ui.add_space(32.0)
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
