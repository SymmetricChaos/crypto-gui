use super::ClassicRngFrame;
use crate::ui_elements::{generate_randoms_box, UiElements};
use rand::{thread_rng, Rng};
use rngs::squares::Squares;

pub struct SquaresFrame {
    rng: Squares,
    randoms: String,
    n_random: usize,
}

impl Default for SquaresFrame {
    fn default() -> Self {
        Self {
            rng: Default::default(),
            randoms: String::new(),
            n_random: 1,
        }
    }
}

impl ClassicRngFrame for SquaresFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/rngs/src/squares.rs",
        );
        ui.add_space(8.0);

        ui.subheading("Key");
        ui.u64_hex_edit(&mut self.rng.key);
        ui.add_space(4.0);

        ui.subheading("Counter");
        ui.u64_hex_edit(&mut self.rng.ctr);

        ui.add_space(16.0);
        generate_randoms_box(ui, &mut self.rng, &mut self.n_random, &mut self.randoms);
        ui.add_space(16.0);
    }

    fn rng(&mut self) -> &mut dyn rngs::SimpleRng {
        &mut self.rng
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.rng.key = rng.gen();
        self.rng.ctr = 0;
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
