use crate::ui_elements::{generate_randoms_box, UiElements};

use super::ClassicRngFrame;
use rand::{thread_rng, Rng};
use rngs::kiss::Kiss;

pub struct KissFrame {
    rng: Kiss,
    randoms: String,
    n_random: usize,
}

impl Default for KissFrame {
    fn default() -> Self {
        Self {
            rng: Default::default(),
            randoms: Default::default(),
            n_random: 5,
        }
    }
}

impl ClassicRngFrame for KissFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/rngs/src/kiss.rs",
        );

        ui.add_space(8.0);
        ui.randomize_reset_rng(self);

        ui.add_space(8.0);
        ui.checkbox(
            &mut self.rng.corrected,
            "Use Corrected Multipliy-with-Carry",
        );

        ui.add_space(8.0);
        ui.subheading("Multipliy-with-Carry State Variables");
        ui.label("w");
        ui.u32_hex_edit(&mut self.rng.w);

        ui.add_space(4.0);
        ui.label("z");
        ui.u32_hex_edit(&mut self.rng.z);

        ui.add_space(8.0);
        ui.subheading("Linear Congruential Generator State");
        ui.u32_hex_edit(&mut self.rng.jcong);

        ui.add_space(8.0);
        ui.subheading("Xorshift State");
        ui.u32_hex_edit(&mut self.rng.jsr);

        ui.add_space(8.0);
        generate_randoms_box(ui, &mut self.rng, &mut self.n_random, &mut self.randoms);
    }

    fn rng(&mut self) -> &mut dyn rngs::ClassicRng {
        &mut self.rng
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.rng.jcong = rng.gen::<u32>();
        self.rng.jsr = rng.gen::<u32>();
        self.rng.z = rng.gen::<u32>();
        self.rng.w = rng.gen::<u32>();
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
