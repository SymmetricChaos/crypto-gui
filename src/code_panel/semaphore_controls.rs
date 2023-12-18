use std::collections::HashMap;

use super::CodeFrame;
use codes::text_standards::semaphore::Semaphore;
use egui::Image;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref SEMAPHORE_IMAGES: HashMap<&'static str, Image<'static>> = {
        let mut map = HashMap::with_capacity(30);
        map.insert(
            "A",
            egui::Image::new(egui::include_image!("semaphore_flags/Alpha.svg")),
        );
        map.insert(
            "B",
            egui::Image::new(egui::include_image!("semaphore_flags/Bravo.svg")),
        );
        map.insert(
            "C",
            egui::Image::new(egui::include_image!("semaphore_flags/Charlie.svg")),
        );
        map.insert(
            "D",
            egui::Image::new(egui::include_image!("semaphore_flags/Delta.svg")),
        );
        map.insert(
            "E",
            egui::Image::new(egui::include_image!("semaphore_flags/Echo.svg")),
        );
        map.insert(
            "F",
            egui::Image::new(egui::include_image!("semaphore_flags/Foxtrot.svg")),
        );
        map.insert(
            "G",
            egui::Image::new(egui::include_image!("semaphore_flags/Golf.svg")),
        );
        map.insert(
            "H",
            egui::Image::new(egui::include_image!("semaphore_flags/Hotel.svg")),
        );
        map.insert(
            "I",
            egui::Image::new(egui::include_image!("semaphore_flags/India.svg")),
        );
        map.insert(
            "J",
            egui::Image::new(egui::include_image!("semaphore_flags/Juliet.svg")),
        );
        map.insert(
            "K",
            egui::Image::new(egui::include_image!("semaphore_flags/Kilo.svg")),
        );
        map.insert(
            "L",
            egui::Image::new(egui::include_image!("semaphore_flags/Lima.svg")),
        );
        map.insert(
            "M",
            egui::Image::new(egui::include_image!("semaphore_flags/Mike.svg")),
        );
        map.insert(
            "N",
            egui::Image::new(egui::include_image!("semaphore_flags/November.svg")),
        );
        map.insert(
            "O",
            egui::Image::new(egui::include_image!("semaphore_flags/Oscar.svg")),
        );
        map.insert(
            "P",
            egui::Image::new(egui::include_image!("semaphore_flags/Papa.svg")),
        );
        map.insert(
            "Q",
            egui::Image::new(egui::include_image!("semaphore_flags/Quebec.svg")),
        );
        map.insert(
            "R",
            egui::Image::new(egui::include_image!("semaphore_flags/Romeo.svg")),
        );
        map.insert(
            "S",
            egui::Image::new(egui::include_image!("semaphore_flags/Sierra.svg")),
        );
        map.insert(
            "T",
            egui::Image::new(egui::include_image!("semaphore_flags/Tango.svg")),
        );
        map.insert(
            "U",
            egui::Image::new(egui::include_image!("semaphore_flags/Uniform.svg")),
        );
        map.insert(
            "V",
            egui::Image::new(egui::include_image!("semaphore_flags/Victor.svg")),
        );
        map.insert(
            "W",
            egui::Image::new(egui::include_image!("semaphore_flags/Whiskey.svg")),
        );
        map.insert(
            "X",
            egui::Image::new(egui::include_image!("semaphore_flags/X-ray.svg")),
        );
        map.insert(
            "Y",
            egui::Image::new(egui::include_image!("semaphore_flags/Yankee.svg")),
        );
        map.insert(
            "Z",
            egui::Image::new(egui::include_image!("semaphore_flags/Zulu.svg")),
        );
        map.insert(
            "cancel",
            egui::Image::new(egui::include_image!("semaphore_flags/_Cancel.svg")),
        );
        map.insert(
            "error",
            egui::Image::new(egui::include_image!("semaphore_flags/_Error.svg")),
        );
        map.insert(
            "numeric",
            egui::Image::new(egui::include_image!("semaphore_flags/_Numeric.svg")),
        );
        map.insert(
            "ready",
            egui::Image::new(egui::include_image!("semaphore_flags/_Ready.svg")),
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
            .show(ui, |ui| {
                for (a, b) in self.code.chars_codes() {
                    ui.label(a);
                    ui.label(b);
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
