use egui::TextStyle;
use rand::{thread_rng, Rng};
use rngs::{
    pcg::{Pcg, PcgTransform},
    ClassicRng,
};

use crate::ui_elements::{filter_and_parse_u64, generate_random_u32s_box, UiElements};

use super::ClassicRngFrame;

pub struct PcgFrame {
    rng: Pcg,
    state_string: String,
    multiplier_string: String,
    increment_string: String,
    randoms: String,
    n_random: usize,
}

impl Default for PcgFrame {
    fn default() -> Self {
        let rng = Pcg::default();
        Self {
            state_string: rng.state.to_string(),
            multiplier_string: rng.multiplier.to_string(),
            increment_string: rng.increment.to_string(),
            randoms: String::new(),
            rng,
            n_random: 5,
        }
    }
}

impl PcgFrame {
    fn input_control(ui: &mut egui::Ui, string: &mut String, n: &mut u64) {
        if ui
            .add_sized(
                [40.0, 20.0],
                egui::TextEdit::singleline(string)
                    .font(TextStyle::Monospace)
                    .clip_text(false),
            )
            .changed()
        {
            filter_and_parse_u64(n, string);
        }
    }

    fn set_all_strings(&mut self) {
        self.state_string = self.rng.state.to_string();
        self.multiplier_string = self.rng.multiplier.to_string();
        self.increment_string = self.rng.increment.to_string();
    }
}

impl ClassicRngFrame for PcgFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        if ui.button("Randomize").clicked() {
            self.randomize()
        }

        ui.add_space(16.0);
        ui.subheading("Calculations");
        let m = (self.rng.state)
            .wrapping_mul(self.rng.multiplier)
            .wrapping_add(self.rng.increment);
        ui.horizontal(|ui| {
            Self::input_control(ui, &mut self.state_string, &mut self.rng.state);
            ui.subheading(" × ");
            Self::input_control(ui, &mut self.multiplier_string, &mut self.rng.multiplier);
            ui.subheading(" + ");
            Self::input_control(ui, &mut self.increment_string, &mut self.rng.increment);
            ui.subheading(" = ");
            if self.rng.increment % 2 == 0 {
                self.rng.increment = self.rng.increment.wrapping_add(1);
                self.increment_string = self.rng.increment.to_string();
            }
            if self.rng.multiplier == 0 {
                self.rng.multiplier = 1;
                self.multiplier_string = String::from("1");
            }

            ui.false_control_string(format!("{m}"));
        });

        ui.add_space(16.0);

        ui.subheading("Permutation Function");
        ui.horizontal(|ui| {
            ui.selectable_value(
                &mut self.rng.transform,
                PcgTransform::Rs,
                PcgTransform::Rs.name(),
            );
            ui.selectable_value(
                &mut self.rng.transform,
                PcgTransform::Rr,
                PcgTransform::Rr.name(),
            );
        });
        ui.horizontal(|ui| {
            ui.selectable_value(
                &mut self.rng.transform,
                PcgTransform::XshRs,
                PcgTransform::XshRs.name(),
            );
            ui.selectable_value(
                &mut self.rng.transform,
                PcgTransform::XshRr,
                PcgTransform::XshRr.name(),
            );
        });

        ui.add_space(16.0);

        ui.subheading("Output");
        ui.label(format!("{}", self.rng.transform(m)));

        if ui.button("step").clicked() {
            self.rng.next_u32();
            self.state_string = self.rng.state.to_string();
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
        self.rng.state = rng.gen();
        self.rng.multiplier = rng.gen();
        self.rng.increment = rng.gen();
        self.set_all_strings();
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
