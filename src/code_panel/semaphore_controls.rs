use std::collections::HashMap;

use super::CodeFrame;
use codes::text_standards::semaphore::Semaphore;
use egui::ImageSource;
use lazy_static::lazy_static;

// lazy_static! {
//     pub static ref SEMAPHORE_IMAGES: HashMap<&'static str, ImageSource<'static>> = {
//         let mut map = HashMap::with_capacity(30);
//         map.insert(
//             "A",
//             egui::include_image!("semaphore_flags/Semaphore_Alpha.svg"),
//         );
//         map.insert(
//             "B",
//             egui::include_image!("semaphore_flags/Semaphore_Bravo.svg"),
//         );
//         map
//     };
// }

pub struct SemaphoreFrame {
    code: Semaphore,
}

impl Default for SemaphoreFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for SemaphoreFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("flag_grid")
            .num_columns(3)
            .striped(true)
            .min_row_height(100.0)
            .show(ui, |ui| {
                for (a, b) in self.code.chars_codes() {
                    ui.label(a);
                    ui.label(b);
                    // if let Some(img) = SEMAPHORE_IMAGES.get(a) {
                    //     ui.image(img.clone()).rect.set_height(200.0);
                    // }

                    ui.end_row()
                }
            });

        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
