use super::ClassicRngFrame;
use crate::ui_elements::UiElements;
use num::Integer;
use rand::{thread_rng, Rng};
use rngs::weyl::WeylSequence;
use utils::text_functions::filter_string;

pub struct WeylSequenceFrame {
    rng: WeylSequence,
    modulus_string: String,
    increment_string: String,
}

impl Default for WeylSequenceFrame {
    fn default() -> Self {
        let r = WeylSequence::default();
        let m = r.modulus.to_string();
        let i = r.increment.to_string();
        Self {
            rng: r,
            modulus_string: m,
            increment_string: i,
        }
    }
}

impl WeylSequenceFrame {}

impl ClassicRngFrame for WeylSequenceFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.subheading("Modulus");
        if ui.control_string(&mut self.modulus_string).changed() {
            filter_string(&mut self.modulus_string, &"0123456789");
            self.rng.modulus = self.modulus_string.parse().unwrap();
        }
        ui.add_space(16.0);
        ui.subheading("Increment");
        if ui.control_string(&mut self.increment_string).changed() {
            filter_string(&mut self.increment_string, &"0123456789");
            self.rng.increment = self.increment_string.parse().unwrap();
        }
        if self.rng.increment.gcd(&self.rng.modulus) == 1 {
            ui.error_text("");
        } else {
            ui.error_text("Increment must be co-prime to the Modulus.");
        }
    }

    fn rng(&self) -> &dyn rngs::ClassicRng {
        &self.rng
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        for _ in 0..100 {
            let n = rng.gen_range(0..self.rng.modulus);
            if n.gcd(&self.rng.modulus) == 1 {
                self.rng.state;
                return;
            }
        }
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
