use super::ClassicRngFrame;
use crate::ui_elements::{generate_randoms_box, UiElements};
use egui::DragValue;
use num::{BigUint, FromPrimitive};
use num_prime::RandPrime;
use rand::thread_rng;
use rngs::blum_blum_shub::BlumBlumShub;

pub struct BlumBlumShubFrame {
    rng: BlumBlumShub,
    p: u64,
    q: u64,
    local_state: u64,
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
            local_state: 2,
            randoms: String::new(),
            n_random: 5,
            valid_m: true,
        }
    }
}

impl BlumBlumShubFrame {}

impl ClassicRngFrame for BlumBlumShubFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/rngs/src/blum_blum_shub.rs",
        );
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.subheading("Prime-P");
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                let mut rng = thread_rng();
                self.p = rng.gen_safe_prime(64);
                self.valid_m = self.rng.set_m(self.p, self.q).is_ok();
            }
        });
        if ui.add(DragValue::new(&mut self.p)).changed() {
            self.valid_m = self.rng.set_m(self.p, self.q).is_ok();
        };
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.subheading("Prime-Q");
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                let mut rng = thread_rng();
                self.q = rng.gen_safe_prime(64);
                self.valid_m = self.rng.set_m(self.p, self.q).is_ok();
            }
        });
        if ui.add(DragValue::new(&mut self.q)).changed() {
            self.valid_m = self.rng.set_m(self.p, self.q).is_ok();
        };

        ui.add_space(16.0);
        ui.subheading("Modulus");
        if self.valid_m {
            ui.label(format!("{} × {} = {}", self.p, self.q, self.rng.m));
        } else {
            ui.error_text(format!(
                "{} × {} = {}; p and q should both be safe primes",
                self.p, self.q, self.rng.m
            ));
        }

        ui.add_space(16.0);
        ui.subheading("Seed Value");
        if ui.add(DragValue::new(&mut self.local_state)).changed() {
            self.rng.state = BigUint::from_u64(self.local_state).unwrap()
        }
        ui.add_space(4.0);
        ui.subheading("Current State");
        ui.label(self.rng.state.to_str_radix(10));

        ui.add_space(16.0);
        ui.subheading("Step Once");
        ui.label("Blum-Blum-Shub steps 32 times to produce a new integer. This steps once.");
        if ui.button("step").clicked() {
            self.rng.step();
        }

        ui.add_space(16.0);
        generate_randoms_box(ui, &mut self.rng, &mut self.n_random, &mut self.randoms);
        ui.add_space(16.0);
    }

    fn rng(&mut self) -> &mut dyn rngs::ClassicRng {
        &mut self.rng
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.q = rng.gen_safe_prime(64);
        self.p = rng.gen_safe_prime(64);
        self.valid_m = self.rng.set_m(self.p, self.q).is_ok();
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
