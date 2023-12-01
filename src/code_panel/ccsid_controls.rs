use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::{
    binary_to_text::BinaryToTextMode,
    text_standards::code_pages::{Ccsid, CodePage, DisplayMode},
};

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
        code.b2t_mode = Some(codes::binary_to_text::BinaryToTextMode::Hex);
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

        ui.label("Encoding Mode");
        ui.selectable_value(&mut self.code.b2t_mode, Some(BinaryToTextMode::Hex), "Hex")
            .on_hover_text("interpret input as hexcode");
        ui.selectable_value(
            &mut self.code.b2t_mode,
            Some(BinaryToTextMode::Utf8),
            "UTF-8",
        )
        .on_hover_text("treat text as bytes of UTF-8");

        ui.add_space(16.0);
        ui.two_column_table("Character", "Code", self.code.chars_codes());
        ui.add_space(32.0)
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
