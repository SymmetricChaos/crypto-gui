use super::ClassicRngFrame;
use crate::ui_elements::{generate_randoms_box, UiElements};
use rand::{thread_rng, Rng};
use rngs::{plcg::Plcg32, ClassicRng};

pub struct PlcgFrame {
    rng: Plcg32,
    randoms: String,
    n_random: usize,
}

impl Default for PlcgFrame {
    fn default() -> Self {
        Self {
            rng: Default::default(),

            randoms: String::new(),
            n_random: 5,
        }
    }
}

impl PlcgFrame {}

impl ClassicRngFrame for PlcgFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/rngs/src/plcg.rs",
        );
        ui.add_space(8.0);

        ui.randomize_reset_rng(self);
        ui.add_space(4.0);

        ui.subheading("State");
        ui.u32_drag_value_dec(&mut self.rng.state);
        ui.add_space(4.0);

        ui.subheading("Modulus");
        ui.u32_drag_value_dec(&mut self.rng.modulus);
        ui.add_space(4.0);

        ui.subheading("Coefficients");
        ui.label("The coefficients are given starting with the constant term as is standard in computer science.");
        for i in 0..5 {
            ui.u32_drag_value_dec(&mut self.rng.coefs[i]);
        }
        ui.add_space(8.0);

        if ui.button("step").clicked() {
            self.rng.next_u32();
        }

        ui.add_space(8.0);
        generate_randoms_box(ui, &mut self.rng, &mut self.n_random, &mut self.randoms);
    }

    fn rng(&mut self) -> &mut dyn rngs::ClassicRng {
        &mut self.rng
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.rng.modulus = rng.gen();
        self.rng.state = rng.gen::<u32>() % self.rng.modulus;
        for i in 0..5 {
            self.rng.coefs[i] = rng.gen::<u32>() % self.rng.modulus;
        }
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
