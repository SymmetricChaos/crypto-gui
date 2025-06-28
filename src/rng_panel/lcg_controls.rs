use super::ClassicRngFrame;
use crate::ui_elements::{filter_and_parse_u32, generate_randoms_box, UiElements};
use egui::TextStyle;
use num::Integer;
use rand::{thread_rng, Rng};
use rngs::{lcg::Lcg, ClassicRng};

pub struct LcgFrame {
    rng: Lcg,
    state_string: String,
    multiplier_string: String,
    increment_string: String,
    modulus_string: String,
    randoms: String,
    n_random: usize,
}

impl Default for LcgFrame {
    fn default() -> Self {
        Self {
            rng: Default::default(),
            state_string: String::from("1257924810"),
            multiplier_string: String::from("1664525"),
            increment_string: String::from("1013904223"),
            modulus_string: String::from("4294967295"),
            randoms: String::new(),
            n_random: 5,
        }
    }
}

impl LcgFrame {
    fn input_control(ui: &mut egui::Ui, string: &mut String, n: &mut u32) {
        if ui
            .add_sized(
                [40.0, 20.0],
                egui::TextEdit::singleline(string)
                    .font(TextStyle::Monospace)
                    .clip_text(false),
            )
            .changed()
        {
            filter_and_parse_u32(n, string);
        }
    }

    fn set_all_strings(&mut self) {
        self.state_string = self.rng.state.to_string();
        self.multiplier_string = self.rng.multiplier.to_string();
        self.increment_string = self.rng.increment.to_string();
        self.modulus_string = self.rng.modulus.to_string();
    }
}

impl ClassicRngFrame for LcgFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/rngs/src/lcg.rs",
        );
        ui.add_space(8.0);

        ui.add_space(8.0);
        ui.randomize_reset_rng(self);

        ui.add_space(16.0);
        ui.subheading("Calculation");
        ui.horizontal(|ui| {
            ui.subheading("(");
            Self::input_control(ui, &mut self.state_string, &mut self.rng.state);
            ui.subheading(" × ");
            Self::input_control(ui, &mut self.multiplier_string, &mut self.rng.multiplier);
            ui.subheading(" + ");
            Self::input_control(ui, &mut self.increment_string, &mut self.rng.increment);
            ui.subheading(") % ");
            Self::input_control(ui, &mut self.modulus_string, &mut self.rng.modulus);
            ui.subheading(" = ");
            let mut m =
                (self.rng.multiplier as u64 * self.rng.state as u64) % self.rng.modulus as u64;
            m = (m + self.rng.increment as u64) % self.rng.modulus as u64;
            ui.false_control_string(format!("{m}"));
        });
        if self.rng.modulus < 2 {
            self.rng.modulus = 2;
        }
        if self.rng.multiplier == 0 {
            self.rng.multiplier = 1;
        }
        if self.rng.increment > self.rng.modulus {
            self.rng.increment %= self.rng.modulus;
        }
        if self.rng.multiplier > self.rng.modulus {
            self.rng.multiplier %= self.rng.modulus;
        }
        self.set_all_strings();
        ui.add_space(8.0);
        if ui.button("step").clicked() {
            self.rng.next_u32();
            self.set_all_strings();
        }
        ui.add_space(8.0);
        generate_randoms_box(ui, &mut self.rng, &mut self.n_random, &mut self.randoms);
        self.state_string = self.rng.state.to_string();
    }

    fn rng(&mut self) -> &mut dyn rngs::ClassicRng {
        &mut self.rng
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.rng.state = rng.gen();
        self.rng.multiplier = rng.gen();
        self.rng.increment = rng.gen();
        self.rng.modulus = rng.gen();
        self.set_all_strings();
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
