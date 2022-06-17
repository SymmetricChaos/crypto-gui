use eframe::egui::Ui;
use lazy_static::lazy_static;
use rand::{prelude::StdRng, SeedableRng};
use std::sync::Mutex;

pub struct RngController {
    seed_string: String,
}

impl RngController {
    pub fn controls(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.label("Control RNG");
            ui.text_edit_singleline(&mut self.seed_string)
                .on_hover_text("Input a number to seed the RNG.");
            if ui.button("Set").clicked() {
                seed_global_rng(self.seed_string.parse::<u64>().unwrap_or_else(|_| 0));
            }
            ui.add_space(12.0);
            if ui.button("Reset").clicked() {
                randomize_global_rng();
            }
        });
    }
}

lazy_static! {
    pub static ref GLOBAL_RNG: Mutex<StdRng> = Mutex::new(StdRng::seed_from_u64(3141592654));
    pub static ref RNG_CONTROLS: Mutex<RngController> = Mutex::new(RngController {
        seed_string: String::from("3141592654")
    });
}

pub fn seed_global_rng(n: u64) {
    *GLOBAL_RNG.lock().unwrap() = StdRng::seed_from_u64(n);
}

pub fn randomize_global_rng() {
    *GLOBAL_RNG.lock().unwrap() = StdRng::from_entropy();
}

pub fn get_global_rng() -> std::sync::MutexGuard<'static, StdRng> {
    GLOBAL_RNG.lock().unwrap()
}

pub fn global_rng_controls(ui: &mut Ui) {
    RNG_CONTROLS.lock().unwrap().controls(ui)
}
