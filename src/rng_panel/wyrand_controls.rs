use rand::{thread_rng, Rng};
use rngs::wyrand::WyRand;

use crate::{
    rng_panel::ClassicRngFrame,
    ui_elements::{generate_randoms_box, UiElements},
};

pub struct WyRandFrame {
    rng: WyRand,
    randoms: String,
    n_random: usize,
}

impl Default for WyRandFrame {
    fn default() -> Self {
        Self {
            rng: Default::default(),
            randoms: String::new(),
            n_random: 1,
        }
    }
}

impl ClassicRngFrame for WyRandFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/rngs/src/wyrand.rs",
        );
        ui.add_space(8.0);

        ui.randomize_reset_rng(self);
        ui.add_space(8.0);

        ui.subheading("State");
        ui.u64_hex_edit(&mut self.rng.state);
        ui.add_space(8.0);

        generate_randoms_box(ui, &mut self.rng, &mut self.n_random, &mut self.randoms);
        ui.add_space(16.0);
    }

    fn rng(&mut self) -> &mut dyn rngs::SimpleRng {
        &mut self.rng
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.rng.state = rng.gen()
    }

    fn reset(&mut self) {
        *self = Self::default();
    }
}
