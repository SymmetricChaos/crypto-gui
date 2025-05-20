use super::ClassicRngFrame;
use crate::ui_elements::{generate_randoms_box, UiElements};
use rand::{thread_rng, Rng};
use rngs::rabbit::Rabbit;

pub struct RabbitFrame {
    rng: Rabbit,
    key: [u32; 4],
    n_random: usize,
    randoms: String,
}

impl Default for RabbitFrame {
    fn default() -> Self {
        Self {
            rng: Default::default(),
            key: [0; 4],
            n_random: 5,
            randoms: String::new(),
        }
    }
}

impl RabbitFrame {}

impl ClassicRngFrame for RabbitFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/rngs/src/rabbit.rs",
        );

        ui.add_space(8.0);
        ui.subheading("Key");
        for i in 0..4 {
            if ui.u32_hex_edit(&mut self.key[i]).lost_focus() {
                self.rng = Rabbit::with_key_u32(self.key)
            }
        }

        ui.add_space(8.0);
        generate_randoms_box(ui, &mut self.rng, &mut self.n_random, &mut self.randoms);
    }

    fn rng(&self) -> &dyn rngs::ClassicRng {
        &self.rng
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        rng.fill(&mut self.key);
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
