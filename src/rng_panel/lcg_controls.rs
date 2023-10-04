use egui::TextStyle;
use rand::{thread_rng, Rng};
use rngs::{lcg::Lcg, ClassicRng};

use crate::ui_elements::{filter_and_parse_u32, generate_random_nums_box, UiElements};

use super::ClassicRngFrame;

pub struct LcgFrame {
    rng: Lcg,
    state_string: String,
    multiplier_string: String,
    increment_string: String,
    modulus_string: String,
    randoms: String,
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
        // ui.subheading("State");
        // if ui.control_string(&mut self.state_string).changed() {
        //     filter_and_parse_u32(&mut self.rng.state, &mut self.state_string);
        // }
        // ui.add_space(8.0);

        // ui.subheading("Multiplier");
        // if ui.control_string(&mut self.multiplier_string).changed() {
        //     filter_and_parse_u32(&mut self.rng.multiplier, &mut self.multiplier_string);
        // }
        // ui.add_space(8.0);

        // ui.subheading("Increment");
        // if ui.control_string(&mut self.increment_string).changed() {
        //     filter_and_parse_u32(&mut self.rng.increment, &mut self.increment_string);
        // }
        // ui.add_space(8.0);

        // ui.subheading("Modulus (Divisor)");
        // if ui.control_string(&mut self.modulus_string).changed() {
        //     filter_and_parse_u32(&mut self.rng.modulus, &mut self.modulus_string);
        // }
        // ui.add_space(8.0);

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
        ui.add_space(8.0);
        if ui.button("step").clicked() {
            self.rng.step();
            self.set_all_strings();
        }
        ui.add_space(8.0);
        generate_random_nums_box(ui, &mut self.rng, 10, &mut self.randoms);
        self.state_string = self.rng.state.to_string();
    }

    fn rng(&self) -> &dyn rngs::ClassicRng {
        &self.rng
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
