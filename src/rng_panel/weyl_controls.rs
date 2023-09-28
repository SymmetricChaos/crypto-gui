use super::ClassicRngFrame;
use crate::ui_elements::UiElements;
use num::Integer;
use rand::{thread_rng, Rng};
use rngs::{weyl::WeylSequence, ClassicRng};
use utils::text_functions::filter_string;

pub struct WeylSequenceFrame {
    rng: WeylSequence,
    state_string: String,
    modulus_string: String,
    increment_string: String,
}

impl Default for WeylSequenceFrame {
    fn default() -> Self {
        let rng = WeylSequence::default();
        let state_string = rng.state.to_string();
        let modulus_string = rng.modulus.to_string();
        let increment_string = rng.increment.to_string();
        Self {
            rng,
            state_string,
            modulus_string,
            increment_string,
        }
    }
}

impl WeylSequenceFrame {
    fn filter_and_parse(number: &mut u64, string: &mut String) {
        filter_string(string, &"0123456789");
        if string.is_empty() {
            *string = String::from("0");
            *number = 0;
        }
        *number = match string.parse() {
            Ok(n) => n,
            Err(_) => {
                *string = u64::MAX.to_string();
                u64::MAX
            }
        }
    }
}

impl ClassicRngFrame for WeylSequenceFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.subheading("Set State");
        if ui.control_string(&mut self.state_string).changed() {
            Self::filter_and_parse(&mut self.rng.state, &mut self.state_string);
        }
        ui.add_space(16.0);
        ui.subheading("Set Increment");
        if ui.control_string(&mut self.increment_string).changed() {
            Self::filter_and_parse(&mut self.rng.increment, &mut self.increment_string);
        }
        ui.add_space(16.0);
        ui.subheading("Set Modulus");
        if ui.control_string(&mut self.modulus_string).changed() {
            Self::filter_and_parse(&mut self.rng.modulus, &mut self.modulus_string);
        }
        ui.add_space(16.0);
        ui.subheading("Calculation");
        ui.label(format!(
            "({} + {}) % {} = {}",
            self.rng.state,
            self.rng.increment,
            self.rng.modulus,
            (self.rng.state + self.rng.increment) % self.rng.modulus
        ));

        if self.rng.increment.gcd(&self.rng.modulus) == 1 {
            ui.error_text("");
        } else {
            ui.error_text("Increment must be co-prime to the Modulus.");
        }
        ui.add_space(16.0);
        if ui.button("step").clicked() {
            self.rng.step()
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
