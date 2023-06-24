use super::CodeFrame;
use crate::ui_elements::subheading;
use codes::text_standards::ascii::{Ascii, DisplayMode};
use egui::RichText;

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
        use egui_extras::{Column, TableBuilder};
        let table = TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::initial(70.0).range(20.0..=300.0))
            .column(Column::remainder())
            .min_scrolled_height(0.0);

        table
            .header(30.0, |mut header| {
                header.col(|ui| {
                    ui.strong(RichText::new("Code").size(20.0));
                });
                header.col(|ui| {
                    ui.strong(RichText::new("Character").size(20.0));
                });
            })
            .body(|mut body| {
                for (chr, code) in self.code.chars_codes_display() {
                    body.row(20.0, |mut row| {
                        row.col(|ui| {
                            ui.label(RichText::new(code).size(18.0));
                        });

                        row.col(|ui| {
                            ui.label(RichText::new(*chr).size(18.0));
                        });
                    });
                }
            });
        ui.add_space(32.0)
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
