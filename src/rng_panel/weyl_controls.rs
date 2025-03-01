use super::ClassicRngFrame;
use crate::ui_elements::{generate_random_u32s_box, UiElements};
use egui::RichText;
use num::Integer;
use rand::{thread_rng, Rng};
use rngs::{weyl::WeylSequence, ClassicRng};

pub struct WeylSequenceFrame {
    rng: WeylSequence,
    incr_err: bool,
    randoms: String,
    n_random: usize,
}

impl Default for WeylSequenceFrame {
    fn default() -> Self {
        Self {
            rng: WeylSequence::default(),
            incr_err: false,
            randoms: String::new(),
            n_random: 5,
        }
    }
}

impl WeylSequenceFrame {}

impl ClassicRngFrame for WeylSequenceFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/rngs/src/weyl.rs",
        );
        ui.add_space(8.0);

        if ui.button("Randomize").clicked() {
            self.randomize();
        }

        ui.horizontal(|ui| {
            ui.subheading("State");
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                let mut rng = thread_rng();
                self.rng.state = rng.gen_range(0..(1 << 20));
            }
        });
        ui.u32_drag_value_dec(&mut self.rng.state);

        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.subheading("Modulus");
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                let mut rng = thread_rng();
                self.rng.state = rng.gen_range(0..(1 << 20));
            }
        });
        ui.u32_drag_value_dec(&mut self.rng.modulus);

        ui.add_space(8.0);
        ui.subheading("Increment");
        if ui.u32_drag_value_dec(&mut self.rng.increment).lost_focus() {
            self.incr_err = self.rng.increment.gcd(&self.rng.modulus) == 1;
        }
        if self.incr_err {
            ui.error_text("");
        } else {
            ui.error_text("Increment must be co-prime to the Modulus.");
        }

        ui.add_space(8.0);
        ui.subheading("Calculation");
        let calc = format!(
            "({} + {}) % {} = {}",
            self.rng.state,
            self.rng.increment,
            self.rng.modulus,
            (self.rng.state + self.rng.increment) % self.rng.modulus
        );
        ui.label(RichText::new(calc).size(16.0));

        ui.add_space(16.0);
        if ui.button("step").clicked() {
            self.rng.next_u32();
        }
        ui.add_space(16.0);
        generate_random_u32s_box(ui, &mut self.rng, &mut self.n_random, &mut self.randoms);
    }

    fn rng(&self) -> &dyn rngs::ClassicRng {
        &self.rng
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();

        self.rng.state = rng.gen_range(0..(1 << 20));
        self.rng.modulus = rng.gen_range(0..(1 << 20));

        for _ in 0..1000 {
            let n = rng.gen_range(0..self.rng.modulus);
            if n.gcd(&self.rng.modulus) == 1 {
                self.rng.increment = n;
                break;
            }
        }
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
