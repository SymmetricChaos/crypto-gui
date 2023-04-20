use eframe::egui::Ui;
use egui::{Color32, RichText};
use lazy_static::lazy_static;
use rand::{prelude::StdRng, SeedableRng};
use std::sync::Mutex;

lazy_static! {
    pub static ref GLOBAL_RNG: Mutex<StdRng> = Mutex::new(StdRng::seed_from_u64(3141592654));
    pub static ref RNG_CONTROLS: Mutex<RngController> = Mutex::new(RngController {
        seed_string: String::from("3141592654"),
        rng_err: String::new(),
        cached_seed: None,
    });
}

pub struct RngController {
    pub seed_string: String,
    pub rng_err: String,
    cached_seed: Option<u64>,
}

impl RngController {
    pub fn cache_seed(&mut self) {
        match self.seed_string.parse::<u64>() {
            Ok(n) => self.cached_seed = Some(n),
            Err(e) => self.rng_err = e.to_string(),
        }
    }

    pub fn restore_cached_seed(&mut self) {
        match self.cached_seed {
            Some(n) => {
                seed_global_rng(n);
                self.seed_string = n.to_string()
            }
            None => (),
        }
    }

    pub fn controls(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.label("Control RNG");
            ui.text_edit_singleline(&mut self.seed_string)
                .on_hover_text("input a seed number");
            if ui.button("Randomize").clicked() {
                self.seed_string.clear();
                self.rng_err.clear();
                randomize_global_rng();
            }
            if ui.button("Set").clicked() {
                match self.seed_string.parse::<u64>() {
                    Ok(n) => {
                        seed_global_rng(n);
                        self.rng_err.clear();
                    }
                    Err(e) => self.rng_err = e.to_string(),
                }
            };

            ui.group(|ui| {
                ui.horizontal(|ui| {
                    if ui
                        .button("Cache")
                        .on_hover_text("save the current seed")
                        .clicked()
                    {
                        self.cache_seed()
                    };
                    if ui
                        .button("Restore")
                        .on_hover_text("seed RNG from the cached value")
                        .clicked()
                    {
                        self.restore_cached_seed()
                    };
                    if ui
                        .button("Clear")
                        .on_hover_text("remove cached seed")
                        .clicked()
                    {
                        self.rng_err.clear();
                        self.cached_seed = None
                    };
                });
                if let Some(n) = self.cached_seed {
                    ui.label(n.to_string());
                }
            });

            ui.label(
                RichText::new(self.rng_err.clone())
                    .color(Color32::RED)
                    .background_color(Color32::BLACK)
                    .monospace(),
            );
        });
    }
}
pub fn seed_global_rng(n: u64) {
    *GLOBAL_RNG.lock().unwrap() = StdRng::seed_from_u64(n);
}

pub fn randomize_global_rng() {
    *GLOBAL_RNG.lock().unwrap() = StdRng::from_entropy();
}

pub fn get_global_rng() -> std::sync::MutexGuard<'static, StdRng> {
    let mut x = RNG_CONTROLS.lock().unwrap();
    x.seed_string.clear();
    x.rng_err.clear();

    GLOBAL_RNG.lock().unwrap()
}

pub fn global_rng_controls(ui: &mut Ui) {
    RNG_CONTROLS.lock().unwrap().controls(ui)
}
