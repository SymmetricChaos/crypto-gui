use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::text_standards::ics_flags::IcsFlags;
use egui::ImageSource;
use std::collections::HashMap;

const FLAG_IMAGES: [ImageSource<'_>; 26] = [
    egui::include_image!("ics_flags/Alfa.png"),
    egui::include_image!("ics_flags/Bravo.png"),
    egui::include_image!("ics_flags/Charlie.png"),
    egui::include_image!("ics_flags/Delta.png"),
    egui::include_image!("ics_flags/Echo.png"),
    egui::include_image!("ics_flags/Foxtrot.png"),
    egui::include_image!("ics_flags/Golf.png"),
    egui::include_image!("ics_flags/Hotel.png"),
    egui::include_image!("ics_flags/India.png"),
    egui::include_image!("ics_flags/Juliett.png"),
    egui::include_image!("ics_flags/Kilo.png"),
    egui::include_image!("ics_flags/Lima.png"),
    egui::include_image!("ics_flags/Mike.png"),
    egui::include_image!("ics_flags/November.png"),
    egui::include_image!("ics_flags/Oscar.png"),
    egui::include_image!("ics_flags/Papa.png"),
    egui::include_image!("ics_flags/Quebec.png"),
    egui::include_image!("ics_flags/Romeo.png"),
    egui::include_image!("ics_flags/Sierra.png"),
    egui::include_image!("ics_flags/Tango.png"),
    egui::include_image!("ics_flags/Uniform.png"),
    egui::include_image!("ics_flags/Victor.png"),
    egui::include_image!("ics_flags/Whiskey.png"),
    egui::include_image!("ics_flags/X-ray.png"),
    egui::include_image!("ics_flags/Yankee.png"),
    egui::include_image!("ics_flags/Zulu.png"),
];

static FLAG_MAP: std::sync::LazyLock<HashMap<char, egui::Image<'static>>> =
    std::sync::LazyLock::new(|| {
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
            .chars()
            .zip(FLAG_IMAGES.iter().map(|i| egui::Image::new(i.clone())))
            .collect()
    });

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
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/codes/src/text_standards/ics_flags.rs",
        );
        ui.add_space(8.0);

        egui::Grid::new("ics_flag_grid")
            .num_columns(3)
            .striped(true)
            .min_row_height(200.0)
            .show(ui, |ui| {
                for (a, b) in self.code.chars_codes() {
                    ui.mono_strong(a);
                    ui.mono_strong(b);
                    if let Some(img) = FLAG_MAP.get(&a.chars().next().unwrap()) {
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
