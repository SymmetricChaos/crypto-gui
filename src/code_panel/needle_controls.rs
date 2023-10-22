use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::text_standards::needle::Needle;
use rand::thread_rng;
use utils::text_functions::shuffled_str;

pub struct NeedleFrame {
    code: Needle,
}

impl Default for NeedleFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for NeedleFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.subheading("Alphabet");
            if ui.button("🎲").on_hover_text("shuffle").clicked() {
                self.code.alphabet = shuffled_str(&self.code.alphabet, &mut thread_rng());
                self.code.set_map()
            }
        });

        ui.add_space(16.0);
        ui.fill_code_columns(5, 4, self.code.chars_codes());
        ui.add_space(16.0);

        ui.image(egui::include_image!("needle_telegraph.png"))
            .rect
            .set_height(765.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
