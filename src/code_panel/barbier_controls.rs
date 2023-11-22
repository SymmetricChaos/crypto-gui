use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::other::barbier::BarbierCode;

pub struct BarbierFrame {
    code: BarbierCode,
}

impl Default for BarbierFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for BarbierFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.add_space(8.0);

        ui.subheading("Grid");
        egui::Grid::new("columnar_grid")
            .num_columns(6)
            .min_col_width(5.0)
            .striped(true)
            .show(ui, |ui| {
                let mut sylls = BarbierCode::GRID.into_iter();
                for _row in 0..6 {
                    for _col in 0..6 {
                        ui.mono(sylls.next().unwrap());
                    }
                    ui.end_row();
                }
            });

        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
