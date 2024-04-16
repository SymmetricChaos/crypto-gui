use super::ClassicRngFrame;
use crate::ui_elements::{generate_random_u32s_box, UiElements};
use egui::DragValue;

use num::BigUint;
use num_prime::RandPrime;
use rand::thread_rng;
use rngs::blumblumshub::BlumBlumShub;

pub struct BlumBlumShubFrame {
    rng: BlumBlumShub,
    p: u64,
    q: u64,
    // random_bytes: String,
    randoms: String,
    // n_random_bytes: usize,
    n_random: usize,
    valid_m: bool,
}

impl Default for BlumBlumShubFrame {
    fn default() -> Self {
        let mut rng = BlumBlumShub::default();
        let p = 179;
        let q = 467;
        rng.m = BigUint::from(p * q);
        Self {
            rng,
            p,
            q,
            // random_bytes: String::new(),
            randoms: String::new(),
            // n_random_bytes: 5,
            n_random: 5,
            valid_m: true,
        }
    }
}

impl BlumBlumShubFrame {}

impl ClassicRngFrame for BlumBlumShubFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.add_space(16.0);

        ui.horizontal(|ui| {
            ui.subheading("P");
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                let mut rng = thread_rng();
                self.p = rng.gen_safe_prime(64);
                self.valid_m = self.rng.set_m(self.p, self.q).is_ok();
            }
        });
        if ui.add(DragValue::new(&mut self.p)).changed() {
            self.valid_m = self.rng.set_m(self.p, self.q).is_ok();
        };
        ui.horizontal(|ui| {
            ui.subheading("Q");
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                let mut rng = thread_rng();
                self.q = rng.gen_safe_prime(64);
                self.valid_m = self.rng.set_m(self.p, self.q).is_ok();
            }
        });
        if ui.add(DragValue::new(&mut self.q)).changed() {
            self.valid_m = self.rng.set_m(self.p, self.q).is_ok();
        };

        ui.subheading("Modulus");
        if self.valid_m {
            ui.label(format!("{} {} = {}", self.p, self.q, self.rng.m));
        } else {
            ui.error_text(format!(
                "{} {} = {}; p and q are not safe primes",
                self.p, self.q, self.rng.m
            ));
        }

        ui.add_space(16.0);
        ui.subheading(
            "Normally the Blum-Blum-Shub algorithm steps 32 times to produce an integer output.",
        );
        if ui.button("step").clicked() {
            self.rng.step();
        }

        ui.add_space(16.0);
        generate_random_u32s_box(ui, &mut self.rng, &mut self.n_random, &mut self.randoms);
        ui.add_space(16.0);
    }

    fn rng(&self) -> &dyn rngs::ClassicRng {
        &self.rng
    }

    fn randomize(&mut self) {
        // let mut rng = thread_rng();
        // self.key = format!("{:08X}", rng.gen::<u64>());
        // self.rng.i = 0;
        // self.rng.j = 0;
        // self.run_ksa();
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
