use egui::RichText;

use super::CodeFrame;
use codes::braille::{braille_data::UEB_ROWS, unified_english_braille::UnifiedEnglishBraille};

pub struct UebFrame {
    code: UnifiedEnglishBraille,
}

impl Default for UebFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for UebFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label("Braille Order");
        egui::Grid::new("columnar_grid")
            .num_columns(10)
            .min_col_width(5.0)
            .striped(true)
            .show(ui, |ui| {
                for row in 0..7 {
                    let mut cells = UEB_ROWS[row].chars();
                    for _col in 0..10 {
                        if let Some(c) = cells.next() {
                            ui.label(RichText::from(c.to_string()).monospace().size(24.0));
                        }
                    }
                    ui.end_row();
                }
            });
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
