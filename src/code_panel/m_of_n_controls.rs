use codes::ecc::m_of_n::MofNCode;
use egui::Slider;

use crate::ui_elements::UiElements;

use super::CodeFrame;

pub struct MofNCodeFrame {
    code: MofNCode,
}

impl Default for MofNCodeFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for MofNCodeFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.subheading("Length");
        ui.label("Total number of bits in each code.");
        ui.add(Slider::new(&mut self.code.length, 0..=10));
        ui.add_space(8.0);

        ui.subheading("Weight");
        ui.label("Number of 1s in each code.");
        if ui.add(Slider::new(&mut self.code.weight, 1..=10)).changed() {
            self.code.weight = self.code.weight.clamp(1, self.code.length - 1);
        }
        ui.add_space(16.0);

        // use egui_extras::{Column, TableBuilder};
        // let table = TableBuilder::new(ui)
        //     .striped(true)
        //     .resizable(true)
        //     .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
        //     .column(Column::initial(70.0).range(20.0..=300.0))
        //     .column(Column::remainder())
        //     .min_scrolled_height(0.0);
        // table.body(|mut body| {
        //     for code in self.code.list_codes().iter() {
        //         body.row(20.0, |mut row| {
        //             row.col(|ui| {
        //                 ui.mono(code);
        //             });
        //         });
        //     }
        // });
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
