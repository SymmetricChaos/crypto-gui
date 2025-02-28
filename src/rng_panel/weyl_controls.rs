use super::ClassicRngFrame;
use crate::ui_elements::{filter_and_parse_u32, generate_random_u32s_box, UiElements};
use egui::RichText;
use num::Integer;
use rand::{thread_rng, Rng};
use rngs::{weyl::WeylSequence, ClassicRng};

pub struct WeylSequenceFrame {
    rng: WeylSequence,
    state_string: String,
    modulus_string: String,
    increment_string: String,
    randoms: String,
    n_random: usize,
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

        ui.subheading("State");
        let state = ui.control_string(&mut self.state_string);
        if state.changed() || state.lost_focus() {
            filter_and_parse_u32(&mut self.rng.state, &mut self.state_string);
        }
        ui.add_space(8.0);
        ui.subheading("Increment");
        if ui.control_string(&mut self.increment_string).changed() {
            filter_and_parse_u32(&mut self.rng.increment, &mut self.increment_string);
        }
        ui.add_space(8.0);
        ui.subheading("Modulus");
        if ui.control_string(&mut self.modulus_string).changed() {
            filter_and_parse_u32(&mut self.rng.modulus, &mut self.modulus_string);
        }
        if self.rng.increment.gcd(&self.rng.modulus) == 1 {
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
        self.state_string = self.rng.state.to_string();
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
