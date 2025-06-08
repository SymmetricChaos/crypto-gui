use rand::{thread_rng, Rng};
use rngs::randu::Randu;

use crate::{
    rng_panel::ClassicRngFrame,
    ui_elements::{generate_randoms_box, UiElements},
};

pub struct RanduFrame {
    rng: Randu,
    n_random: usize,
    randoms: String,
}

impl Default for RanduFrame {
    fn default() -> Self {
        Self {
            rng: Default::default(),
            n_random: 5,
            randoms: String::new(),
        }
    }
}

impl ClassicRngFrame for RanduFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/rngs/src/randu.rs",
        );

        ui.add_space(8.0);
        ui.subheading("State");
        ui.label("Only odd numbers less than 2^31 are allowed");
        if ui.u32_drag_value_dec(&mut self.rng.state).lost_focus() {
            self.rng.state = self.rng.state % 0x80000000;
            if self.rng.state % 2 == 0 {
                self.rng.state += 1; // cannot overflow
            }
        }

        ui.add_space(8.0);
        generate_randoms_box(ui, &mut self.rng, &mut self.n_random, &mut self.randoms);
    }

    fn rng(&mut self) -> &mut dyn rngs::ClassicRng {
        &mut self.rng
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.rng.state = rng.gen::<u32>() % 0x80000000;
        if self.rng.state % 2 == 0 {
            self.rng.state += 1; // cannot overflow
        }
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
