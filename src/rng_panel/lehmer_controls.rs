use super::ClassicRngFrame;
use crate::ui_elements::{generate_randoms_box, UiElements};
use rand::{thread_rng, Rng};
use rngs::{lehmer::Lehmer, ClassicRng};

pub struct LehmerFrame {
    rng: Lehmer,
    randoms: String,
    n_random: usize,
}

impl Default for LehmerFrame {
    fn default() -> Self {
        Self {
            rng: Default::default(),
            randoms: String::new(),
            n_random: 5,
        }
    }
}

impl LehmerFrame {}

impl ClassicRngFrame for LehmerFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/rngs/src/lehmer.rs",
        );
        ui.add_space(8.0);

        ui.randomize_reset_rng(self);

        ui.add_space(16.0);
        ui.subheading("Multiplier");
        ui.label("The multipler should be odd, otherwise the period and statistical propeties will be refuced.");
        ui.u128_hex_edit(&mut self.rng.multiplier);
        ui.add_space(4.0);

        ui.subheading("State");
        ui.u128_hex_edit(&mut self.rng.state);

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
        self.rng.state = rng.gen();
        self.rng.multiplier = rng.gen();
        self.rng.multiplier |= 1; // force multiplier to be odd
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
