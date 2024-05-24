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
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/tree/master/codes/src/braille",
        );
        ui.add_space(8.0);

        ui.group(|ui| {
            ui.subheading("Encoding");
            ui.selectable_value(&mut self.code.mode, BrailleEncodingType::Dots, "Dots");
            ui.selectable_value(&mut self.code.mode, BrailleEncodingType::Hex, "Hex");
            ui.selectable_value(&mut self.code.mode, BrailleEncodingType::Bits, "Bit");
            ui.selectable_value(&mut self.code.mode, BrailleEncodingType::Ascii, "ASCII");
        });

        ui.add_space(16.0);
        ui.label(match self.code.mode {
            BrailleEncodingType::Dots => "Braille cells are commonly identified by simply naming the dots which are raised. So dots-0 stands for the cell with no dots raised and dots-135 stands for the cells with dots 1, 3, and 5 raised.",
            BrailleEncodingType::Bits => "A simple binary encoding of Braiile is to use six bits with them set if the corresponding cell is raised.",
            BrailleEncodingType::Hex => "The Unicode standard assigns each dot a numeric value (1, 2, 4, 8, 16, 32) and each cell is assigned a value equal to the sum of its dots. The range of Braille characters starts at the hex value of 2800 and each cell can be described by its offset.",
            BrailleEncodingType::Ascii => "A popular encoding of Braille assigns the cells to the range of ASCII characters from '!' to '_' with dots-0 represented by the space.",
        });
        ui.add_space(16.0);

        ui.group(|ui| {
            ui.subheading("Ordering (for display)");
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.code.order, BrailleOrder::Ueb, "UEB");
                ui.selectable_value(&mut self.code.order, BrailleOrder::Unicode, "Unicode");
                ui.selectable_value(&mut self.code.order, BrailleOrder::Ascii, "ASCII");
            });
        });

        ui.add_space(4.0);
        ui.label(match self.code.order {
            BrailleOrder::Ueb => "The UEB order is meant for easy human memorization, arranged into seven rows.",
            BrailleOrder::Unicode => "The Unicode order is a simple numerical ordering convenient for computers and programmers.",
            BrailleOrder::Ascii => "The ASCII order is largely arbitrary with only some Braille cells assigned to characters related to their meaning.",
        });
        ui.add_space(4.0);

        ui.add_space(16.0);
        ui.fill_code_columns(8, 8, Box::new(self.code.chars_codes()));
        ui.add_space(32.0)
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
