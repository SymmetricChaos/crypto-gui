use crate::ui_elements::UiElements;

use super::CodeFrame;
use codes::text_standards::semaphore::Semaphore;
use egui::ImageSource;

// This need to be the same sequence as the names when given by .chars_codes() from the code
const FLAG_IMAGES: [ImageSource<'_>; 30] = [
    egui::include_image!("semaphore_flags/Alfa.png"),
    egui::include_image!("semaphore_flags/Bravo.png"),
    egui::include_image!("semaphore_flags/Charlie.png"),
    egui::include_image!("semaphore_flags/Delta.png"),
    egui::include_image!("semaphore_flags/Echo.png"),
    egui::include_image!("semaphore_flags/Foxtrot.png"),
    egui::include_image!("semaphore_flags/Golf.png"),
    egui::include_image!("semaphore_flags/Hotel.png"),
    egui::include_image!("semaphore_flags/India.png"),
    egui::include_image!("semaphore_flags/Juliett.png"),
    egui::include_image!("semaphore_flags/Kilo.png"),
    egui::include_image!("semaphore_flags/Lima.png"),
    egui::include_image!("semaphore_flags/Mike.png"),
    egui::include_image!("semaphore_flags/November.png"),
    egui::include_image!("semaphore_flags/Oscar.png"),
    egui::include_image!("semaphore_flags/Papa.png"),
    egui::include_image!("semaphore_flags/Quebec.png"),
    egui::include_image!("semaphore_flags/Romeo.png"),
    egui::include_image!("semaphore_flags/Sierra.png"),
    egui::include_image!("semaphore_flags/Tango.png"),
    egui::include_image!("semaphore_flags/Uniform.png"),
    egui::include_image!("semaphore_flags/Victor.png"),
    egui::include_image!("semaphore_flags/Whiskey.png"),
    egui::include_image!("semaphore_flags/X-ray.png"),
    egui::include_image!("semaphore_flags/Yankee.png"),
    egui::include_image!("semaphore_flags/Zulu.png"),
    egui::include_image!("semaphore_flags/_Numeric.png"),
    egui::include_image!("semaphore_flags/_Cancel.png"),
    egui::include_image!("semaphore_flags/_Error.png"),
    egui::include_image!("semaphore_flags/_Ready.png"),
];

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
            .min_row_height(200.0)
            .show(ui, |ui| {
                for ((a, b), img_src) in self.code.chars_codes().zip(FLAG_IMAGES) {
                    ui.mono_strong(a);
                    ui.mono_strong(b);
                    ui.image(img_src);
                    ui.end_row()
                }
            });

        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
