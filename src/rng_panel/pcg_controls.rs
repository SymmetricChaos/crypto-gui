use super::ClassicRngFrame;
use crate::ui_elements::{generate_random_u32s_box, UiElements};
use rand::{thread_rng, Rng};
use rngs::{
    pcg::{Pcg, PcgTransform},
    ClassicRng,
};
use strum::IntoEnumIterator;

pub struct PcgFrame {
    rng: Pcg,
    randoms: String,
    n_random: usize,
}

impl Default for PcgFrame {
    fn default() -> Self {
        Self {
            randoms: String::new(),
            rng: Pcg::default(),
            n_random: 5,
        }
    }
}

impl PcgFrame {}

impl ClassicRngFrame for PcgFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/rngs/src/pcg.rs",
        );
        ui.add_space(8.0);

        if ui.button("Randomize").clicked() {
            self.randomize()
        }

        ui.add_space(16.0);
        ui.subheading("Calculations");
        ui.horizontal(|ui| {
            ui.u64_hex_edit(&mut self.rng.state);
            ui.subheading(" × ");
            ui.u64_hex_edit(&mut self.rng.multiplier);
            ui.subheading(" + ");
            ui.u64_hex_edit(&mut self.rng.increment);
            ui.subheading(" = ");
            if self.rng.increment % 2 == 0 {
                self.rng.increment = self.rng.increment.wrapping_add(1);
            }
            if self.rng.multiplier == 0 {
                self.rng.multiplier = 1;
            }

            ui.false_control_string(format!(
                "{}",
                (self.rng.state)
                    .wrapping_mul(self.rng.multiplier)
                    .wrapping_add(self.rng.increment)
            ));
        });

        ui.add_space(16.0);

        ui.subheading("Permutation Function");

        ui.horizontal(|ui| {
            for variant in PcgTransform::iter() {
                ui.selectable_value(&mut self.rng.transform, variant, variant.to_string());
            }
        });

        ui.add_space(8.0);

        if ui.button("step").clicked() {
            self.rng.next_u32();
        }
        ui.add_space(8.0);

        generate_random_u32s_box(ui, &mut self.rng, &mut self.n_random, &mut self.randoms);
    }

    fn rng(&self) -> &dyn rngs::ClassicRng {
        &self.rng
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.rng.state = rng.gen();
        self.rng.multiplier = rng.gen();
        self.rng.increment = rng.gen();
        if self.rng.increment % 2 == 0 {
            self.rng.increment = self.rng.increment.wrapping_add(1);
        }
        if self.rng.multiplier == 0 {
            self.rng.multiplier = 1;
        }
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
