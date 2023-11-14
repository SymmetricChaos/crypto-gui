use codes::braille::braille_encoding::{BrailleEncoding, BrailleEncodingType, BrailleOrder};

use super::CodeFrame;
use crate::ui_elements::UiElements;

pub struct BrailleEncodingFrame {
    code: BrailleEncoding,
}

impl Default for BrailleEncodingFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for BrailleEncodingFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.subheading("Encoding");
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.code.mode, BrailleEncodingType::Dots, "Dots");
                ui.selectable_value(&mut self.code.mode, BrailleEncodingType::Hex, "Hex");
                ui.selectable_value(&mut self.code.mode, BrailleEncodingType::Bits, "Bit");
                ui.selectable_value(&mut self.code.mode, BrailleEncodingType::Ascii, "ASCII");
            });
        });

        ui.group(|ui| {
            ui.subheading("Ordering");
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.code.order, BrailleOrder::Ueb, "UEB");
                ui.selectable_value(&mut self.code.order, BrailleOrder::Unicode, "Unicode");
                ui.selectable_value(&mut self.code.order, BrailleOrder::Ascii, "ASCII");
            });
        });

        ui.add_space(16.0);
        ui.fill_code_columns(8, 8, Box::new(self.code.chars_codes()));
        ui.add_space(32.0)
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
