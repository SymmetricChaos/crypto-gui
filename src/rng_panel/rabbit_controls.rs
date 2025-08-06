use super::ClassicRngFrame;
use crate::ui_elements::{generate_randoms_box, UiElements};
use rand::{thread_rng, Rng};
use rngs::rabbit::Rabbit;

pub struct RabbitFrame {
    rng: Rabbit,
    key: [u32; 4],
    iv: [u32; 2],
    n_random: usize,
    randoms: String,
    use_iv: bool,
}

impl Default for RabbitFrame {
    fn default() -> Self {
        Self {
            rng: Default::default(),
            key: [0; 4],
            iv: [0; 2],
            n_random: 1,
            randoms: String::new(),
            use_iv: false,
        }
    }
}

impl RabbitFrame {
    fn set_rng(&mut self) {
        if self.use_iv {
            self.rng = Rabbit::with_key_and_iv_u32(self.key, self.iv)
        } else {
            self.rng = Rabbit::with_key_u32(self.key)
        }
    }
}

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
                self.set_rng();
            }
        }

        ui.add_space(8.0);
        ui.checkbox(&mut self.use_iv, "Use Nonce");
        ui.add_space(2.0);
        ui.subheading("Nonce");
        for i in 0..2 {
            if ui.u32_hex_edit(&mut self.iv[i]).lost_focus() {
                self.set_rng();
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
        rng.fill(&mut self.key);
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
