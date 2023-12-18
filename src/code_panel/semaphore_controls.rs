use std::collections::HashMap;

use crate::ui_elements::UiElements;

use super::CodeFrame;
use codes::text_standards::semaphore::Semaphore;
use egui::Image;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref SEMAPHORE_IMAGES: HashMap<&'static str, Image<'static>> = {
        let mut map = HashMap::with_capacity(30);
        map.insert(
            "A",
            egui::Image::new(egui::include_image!("semaphore_flags/Alpha.png")),
        );
        map.insert(
            "B",
            egui::Image::new(egui::include_image!("semaphore_flags/Bravo.png")),
        );
        map.insert(
            "C",
            egui::Image::new(egui::include_image!("semaphore_flags/Charlie.png")),
        );
        map.insert(
            "D",
            egui::Image::new(egui::include_image!("semaphore_flags/Delta.png")),
        );
        map.insert(
            "E",
            egui::Image::new(egui::include_image!("semaphore_flags/Echo.png")),
        );
        map.insert(
            "F",
            egui::Image::new(egui::include_image!("semaphore_flags/Foxtrot.png")),
        );
        map.insert(
            "G",
            egui::Image::new(egui::include_image!("semaphore_flags/Golf.png")),
        );
        map.insert(
            "H",
            egui::Image::new(egui::include_image!("semaphore_flags/Hotel.png")),
        );
        map.insert(
            "I",
            egui::Image::new(egui::include_image!("semaphore_flags/India.png")),
        );
        map.insert(
            "J",
            egui::Image::new(egui::include_image!("semaphore_flags/Juliet.png")),
        );
        map.insert(
            "K",
            egui::Image::new(egui::include_image!("semaphore_flags/Kilo.png")),
        );
        map.insert(
            "L",
            egui::Image::new(egui::include_image!("semaphore_flags/Lima.png")),
        );
        map.insert(
            "M",
            egui::Image::new(egui::include_image!("semaphore_flags/Mike.png")),
        );
        map.insert(
            "N",
            egui::Image::new(egui::include_image!("semaphore_flags/November.png")),
        );
        map.insert(
            "O",
            egui::Image::new(egui::include_image!("semaphore_flags/Oscar.png")),
        );
        map.insert(
            "P",
            egui::Image::new(egui::include_image!("semaphore_flags/Papa.png")),
        );
        map.insert(
            "Q",
            egui::Image::new(egui::include_image!("semaphore_flags/Quebec.png")),
        );
        map.insert(
            "R",
            egui::Image::new(egui::include_image!("semaphore_flags/Romeo.png")),
        );
        map.insert(
            "S",
            egui::Image::new(egui::include_image!("semaphore_flags/Sierra.png")),
        );
        map.insert(
            "T",
            egui::Image::new(egui::include_image!("semaphore_flags/Tango.png")),
        );
        map.insert(
            "U",
            egui::Image::new(egui::include_image!("semaphore_flags/Uniform.png")),
        );
        map.insert(
            "V",
            egui::Image::new(egui::include_image!("semaphore_flags/Victor.png")),
        );
        map.insert(
            "W",
            egui::Image::new(egui::include_image!("semaphore_flags/Whiskey.png")),
        );
        map.insert(
            "X",
            egui::Image::new(egui::include_image!("semaphore_flags/X-ray.png")),
        );
        map.insert(
            "Y",
            egui::Image::new(egui::include_image!("semaphore_flags/Yankee.png")),
        );
        map.insert(
            "Z",
            egui::Image::new(egui::include_image!("semaphore_flags/Zulu.png")),
        );
        map.insert(
            "cancel",
            egui::Image::new(egui::include_image!("semaphore_flags/_Cancel.png")),
        );
        map.insert(
            "error",
            egui::Image::new(egui::include_image!("semaphore_flags/_Error.png")),
        );
        map.insert(
            "numeric",
            egui::Image::new(egui::include_image!("semaphore_flags/_Numeric.png")),
        );
        map.insert(
            "ready",
            egui::Image::new(egui::include_image!("semaphore_flags/_Ready.png")),
        );
        map
    };
}

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
                for (a, b) in self.code.chars_codes() {
                    ui.mono_strong(a);
                    ui.mono_strong(b);
                    if let Some(img) = SEMAPHORE_IMAGES.get(a) {
                        ui.add(img.clone());
                    }
                    ui.end_row()
                }
            });

        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
