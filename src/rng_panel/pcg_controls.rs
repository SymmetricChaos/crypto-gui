use egui::TextStyle;
use rand::{thread_rng, Rng};
use rngs::{
    pcg::{Pcg, PcgTransform},
    ClassicRng,
};

use crate::ui_elements::{filter_and_parse_u64, UiElements};

use super::ClassicRngFrame;

pub struct PcgFrame {
    rng: Pcg,
    state_string: String,
    multiplier_string: String,
    increment_string: String,
}

impl Default for PcgFrame {
    fn default() -> Self {
        Self {
            rng: Default::default(),
            state_string: String::from("1257924810"),
            multiplier_string: String::from("1664525"),
            increment_string: String::from("1013904223"),
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
        ui.subheading("Calculation");
        let m = (self.rng.state)
            .wrapping_mul(self.rng.multiplier)
            .wrapping_add(self.rng.increment);
        ui.horizontal(|ui| {
            ui.subheading("(");
            Self::input_control(ui, &mut self.state_string, &mut self.rng.state);
            ui.subheading(" × ");
            Self::input_control(ui, &mut self.multiplier_string, &mut self.rng.multiplier);
            ui.subheading(" + ");
            Self::input_control(ui, &mut self.increment_string, &mut self.rng.increment);
            ui.subheading(" = ");

            ui.false_control_string(format!("{m}"));
        });

        ui.add_space(16.0);

        ui.subheading("Permutation");
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
            self.rng.step();
            self.set_all_strings();
        }
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
