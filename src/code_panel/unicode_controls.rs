use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::text_standards::unicode::{Unicode, UnicodeEncoding};
use utils::byte_formatting::ByteFormat;

pub struct UnicodeFrame {
    code: Unicode,
}

impl Default for UnicodeFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for UnicodeFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/codes/src/text_standards/unicode.rs",
        );
        ui.add_space(8.0);

        ui.group(|ui| {
            ui.subheading("Encoding");
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.code.encoding, UnicodeEncoding::Utf8, "UTF-8");
                ui.selectable_value(&mut self.code.encoding, UnicodeEncoding::Utf16, "UTF-16");
                ui.selectable_value(&mut self.code.encoding, UnicodeEncoding::Utf32, "UTF-32");
            });
        });
        ui.add_space(8.0);

        match self.code.encoding {
            UnicodeEncoding::Utf8 => ui.label("UTF-8 is the most widely used character encoding in the modern world, partly because it is a superset of ASCII the previous dominant standard, and is the current recommended way of encoding Unicode. It is a variable length code that uses between one and four bytes per character. The one byte codes are equivalent to ASCII. The two byte codes cover most of the remaining world alphabets. The three byte codes contain the common CJK (Chinese, Japanese, Korean) characters. Finally the four byte codes are used for a huge variety of less common symbols include emoji, rare CJK character, and other symbols."),
            UnicodeEncoding::Utf16 => ui.label("UTF-16 was the previous recommended standard for encoding Unicode, mostly prominently used by Microsoft Windows which adopted it before the creation of UTF-8. The encoding is variable width using either one or two code units of sixteen bits each. The single code unit characters cover all commonly used characters in world languages while less common symbols require two code units."),
            UnicodeEncoding::Utf32 => ui.label("UTF-32 is the simplest encoding of Unicode as it assigns 32 bits per character, representing the character's numeric value in Unicode. However it is quite inefficient as the first eleven bits are always zero because there are only 2^21 possible Unicode characters. Because of the large size and wasted space UTF-32 is rarely used for encoding text as a whole, but it is used when representing individual characters on their own."),
        };
        ui.add_space(16.0);

        ui.group(|ui| {
            ui.subheading("Representation");
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.code.mode, ByteFormat::Binary, "Binary");
                ui.selectable_value(&mut self.code.mode, ByteFormat::Hex, "Hexadecimal");
                ui.selectable_value(&mut self.code.mode, ByteFormat::Base64, "Base64");
            });
        });
        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
