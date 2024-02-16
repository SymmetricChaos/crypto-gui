use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::text_standards::code_pages::{Ccsid, CodePage, DisplayMode};
use utils::byte_formatting::ByteFormat;

pub struct CcsidFrame {
    code: Ccsid,
}

impl Default for CcsidFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for CcsidFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.subheading("Code Page");
            ui.selectable_value(&mut self.code.page, CodePage::CP1252, "CP1252");
            ui.selectable_value(&mut self.code.page, CodePage::CP437, "CP437");
        });
        ui.add_space(8.0);
        ui.group(|ui| {
            ui.subheading("Representation");
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.code.mode, DisplayMode::Binary, "Binary");
                ui.selectable_value(&mut self.code.mode, DisplayMode::Octal, "Octal");
                ui.selectable_value(&mut self.code.mode, DisplayMode::Decimal, "Decimal");
                ui.selectable_value(&mut self.code.mode, DisplayMode::Hex, "Hexadecimal");
            });
        });
        ui.add_space(8.0);
        match self.code.page {
            CodePage::CP1252 => ui.label("Also known as Windows-1252 this code page is compatible with the ASCII standard with special characters assigned to the 128 characters available with the extra bit unused by ASCII. The � symbol denotes positions with now assigned value and will be rejected when encoding."),
            CodePage::CP437 => ui.label("This code page was used by the influential IBM PC from 1981. English letters and common symbols are identical to ASCCI but most control characters are replaced with printing symbols. The characters encoded with the eight bit include both characters for European languages that also use the Latin alphabet along with drawing symbols."),
        };

        ui.add_space(8.0);
        ui.checkbox(&mut self.code.spaced, "Use Spaces");
        ui.add_space(16.0);
        ui.two_column_table("Character", "Code", self.code.chars_codes());
        ui.add_space(32.0)
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}

pub struct CcsidBinaryFrame {
    code: Ccsid,
}

impl Default for CcsidBinaryFrame {
    fn default() -> Self {
        let mut code = Ccsid::default();
        code.b2t_mode = Some(ByteFormat::Hex);
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for CcsidBinaryFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.subheading("Code Page");
            ui.selectable_value(&mut self.code.page, CodePage::CP1252, "CP1252");
            ui.selectable_value(&mut self.code.page, CodePage::CP437, "CP437");
        });

        ui.add_space(16.0);
        ui.label("Encoding Mode");
        ui.selectable_value(&mut self.code.b2t_mode, Some(ByteFormat::Hex), "Hex")
            .on_hover_text("interpret input as hexcode");
        ui.selectable_value(&mut self.code.b2t_mode, Some(ByteFormat::Utf8), "UTF-8")
            .on_hover_text("treat text as bytes of UTF-8");
        ui.selectable_value(&mut self.code.b2t_mode, Some(ByteFormat::Base64), "Base64")
            .on_hover_text("treat text as Base64");

        ui.add_space(16.0);
        ui.two_column_table("Character", "Code", self.code.chars_codes());
        ui.add_space(32.0)
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
