use codes::mathematical::twos_complement::{TwosComplement, Width};
use utils::byte_formatting::ByteFormat;

use super::CodeFrame;
use crate::ui_elements::UiElements;

pub struct TwosComplementFrame {
    code: TwosComplement,
}

impl Default for TwosComplementFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for TwosComplementFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/codes/src/mathematical/twos_complement.rs",
        );
        ui.add_space(8.0);

        ui.add_space(8.0);
        ui.checkbox(&mut self.code.spaced, "Use Spaces");
        ui.add_space(8.0);

        ui.group(|ui| {
            ui.subheading("Representation");
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.code.byte_format, ByteFormat::Binary, "Binary");
                ui.selectable_value(&mut self.code.byte_format, ByteFormat::Hex, "Hexadecimal");
                ui.selectable_value(&mut self.code.byte_format, ByteFormat::Base64, "Base64");
            });
        });

        ui.add_space(8.0);
        ui.group(|ui| {
            ui.subheading("Bit Width");
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.code.width, Width::W8, " 8 bits");
                ui.selectable_value(&mut self.code.width, Width::W16, "16 bits");
                ui.selectable_value(&mut self.code.width, Width::W32, "32 bits");
                ui.selectable_value(&mut self.code.width, Width::W64, "64 bits");
            });
        });

        ui.add_space(8.0);
        ui.subheading("Example of Finding the Negative (8-bits)");
        ui.label("The additive inverse (negative) of a number in two's complement is found by inverting all of the bits and then adding one to the result.");
        ui.mono("number       00111000\nbits flipped 11000111\nplus one     11001000");
        ui.label("Adding these numbers in the usual way, but ignoring any carry past the last bit, gives all zeroes.");
        ui.mono(" 00111000\n+11001000\n 00000000");
        ui.label("So if we say that 00111000 = 56 it must be that 11001000 = -56.");
        ui.add_space(16.0);

        ui.subheading("Exception: Most Negative Number (8-bits)");
        ui.label("The most negative number in two's complement is unlike others because its additive inverse is itself, a property that only zero has in standard arithmetic.");
        ui.mono("number       10000000\nbits flipped 01111111\nplus one     10000000");
        ui.label("because of this programs usually assume either that the most negaative number will never be inverted or crash if it is.");
        ui.add_space(16.0);

        ui.label("Example of conversion between \"standard\" base-10 numbers and their representation as two's complement. Encoding is done uses 32-bits but could be done with any number of bits.");
        let pairs = (-15..=16).map(|n| (n.to_string(), format!("{n:0>32b}")));
        ui.fill_code_columns(16, 5, Box::new(pairs));

        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
