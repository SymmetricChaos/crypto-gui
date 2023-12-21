use std::collections::HashMap;

use crate::ui_elements::UiElements;

use super::CodeFrame;
use codes::text_standards::ics_flags::IcsFlags;
use egui::Image;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref ICS_IMAGES: HashMap<&'static str, Image<'static>> = {
        let mut map = HashMap::with_capacity(30);
        map.insert(
            "A",
            egui::Image::new(egui::include_image!("ics_flags/Alfa.png")),
        );
        map.insert(
            "B",
            egui::Image::new(egui::include_image!("ics_flags/Bravo.png")),
        );
        map.insert(
            "C",
            egui::Image::new(egui::include_image!("ics_flags/Charlie.png")),
        );
        map.insert(
            "D",
            egui::Image::new(egui::include_image!("ics_flags/Delta.png")),
        );
        map.insert(
            "E",
            egui::Image::new(egui::include_image!("ics_flags/Echo.png")),
        );
        map.insert(
            "F",
            egui::Image::new(egui::include_image!("ics_flags/Foxtrot.png")),
        );
        map.insert(
            "G",
            egui::Image::new(egui::include_image!("ics_flags/Golf.png")),
        );
        map.insert(
            "H",
            egui::Image::new(egui::include_image!("ics_flags/Hotel.png")),
        );
        map.insert(
            "I",
            egui::Image::new(egui::include_image!("ics_flags/India.png")),
        );
        map.insert(
            "J",
            egui::Image::new(egui::include_image!("ics_flags/Juliett.png")),
        );
        map.insert(
            "K",
            egui::Image::new(egui::include_image!("ics_flags/Kilo.png")),
        );
        map.insert(
            "L",
            egui::Image::new(egui::include_image!("ics_flags/Lima.png")),
        );
        map.insert(
            "M",
            egui::Image::new(egui::include_image!("ics_flags/Mike.png")),
        );
        map.insert(
            "N",
            egui::Image::new(egui::include_image!("ics_flags/November.png")),
        );
        map.insert(
            "O",
            egui::Image::new(egui::include_image!("ics_flags/Oscar.png")),
        );
        map.insert(
            "P",
            egui::Image::new(egui::include_image!("ics_flags/Papa.png")),
        );
        map.insert(
            "Q",
            egui::Image::new(egui::include_image!("ics_flags/Quebec.png")),
        );
        map.insert(
            "R",
            egui::Image::new(egui::include_image!("ics_flags/Romeo.png")),
        );
        map.insert(
            "S",
            egui::Image::new(egui::include_image!("ics_flags/Sierra.png")),
        );
        map.insert(
            "T",
            egui::Image::new(egui::include_image!("ics_flags/Tango.png")),
        );
        map.insert(
            "U",
            egui::Image::new(egui::include_image!("ics_flags/Uniform.png")),
        );
        map.insert(
            "V",
            egui::Image::new(egui::include_image!("ics_flags/Victor.png")),
        );
        map.insert(
            "W",
            egui::Image::new(egui::include_image!("ics_flags/Whiskey.png")),
        );
        map.insert(
            "X",
            egui::Image::new(egui::include_image!("ics_flags/X-ray.png")),
        );
        map.insert(
            "Y",
            egui::Image::new(egui::include_image!("ics_flags/Yankee.png")),
        );
        map.insert(
            "Z",
            egui::Image::new(egui::include_image!("ics_flags/Zulu.png")),
        );
        map
    };
}

pub struct IcsFlagsFrame {
    code: IcsFlags,
}

impl Default for IcsFlagsFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for IcsFlagsFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("ics_flag_grid")
            .num_columns(3)
            .striped(true)
            .min_row_height(200.0)
            .show(ui, |ui| {
                for (a, b) in self.code.chars_codes() {
                    ui.mono_strong(a);
                    ui.mono_strong(b);
                    if let Some(img) = ICS_IMAGES.get(a) {
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
