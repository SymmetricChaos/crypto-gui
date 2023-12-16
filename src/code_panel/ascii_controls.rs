use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::text_standards::ascii::{Ascii, DisplayMode, UpperBit};

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
                ui.selectable_value(&mut self.code.mode, DisplayMode::EightBit, "8-Bit");
                ui.selectable_value(&mut self.code.mode, DisplayMode::SevenBit, "7-Bit");
                ui.selectable_value(&mut self.code.mode, DisplayMode::Octal, "Octal");
                ui.selectable_value(&mut self.code.mode, DisplayMode::Decimal, "Decimal");
                ui.selectable_value(&mut self.code.mode, DisplayMode::Hex, "Hexadecimal");
            });
        });
        ui.add_space(8.0);
        ui.group(|ui| {
            ui.subheading("High Bit");
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut self.code.upper_bit,
                    UpperBit::Unset,
                    "Unset (most common)",
                );
                ui.selectable_value(&mut self.code.upper_bit, UpperBit::Set, "Set");
                ui.selectable_value(&mut self.code.upper_bit, UpperBit::Even, "Even Parity");
                ui.selectable_value(&mut self.code.upper_bit, UpperBit::Odd, "Odd Parity");
            });
        });
        ui.add_space(8.0);
        ui.checkbox(&mut self.code.spaced, "Use Spaces");
        ui.add_space(16.0);
        ui.two_column_table("Character", "Code", self.code.chars_codes_display());
        ui.add_space(32.0)
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
