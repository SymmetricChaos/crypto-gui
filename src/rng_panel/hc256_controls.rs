use super::ClassicRngFrame;
use crate::ui_elements::{generate_randoms_box, UiElements};
use rand::{thread_rng, Rng};
use rngs::hc256::Hc256;

pub struct Hc256Frame {
    rng: Hc256,
    key: [u32; 8],
    iv: [u32; 8],
    n_random: usize,
    randoms: String,
}

impl Default for Hc256Frame {
    fn default() -> Self {
        Self {
            rng: Default::default(),
            key: [0; 8],
            iv: [0; 8],
            n_random: 1,
            randoms: String::new(),
        }
    }
}

impl Hc256Frame {}

impl ClassicRngFrame for Hc256Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/rngs/src/hc256.rs",
        );

        ui.add_space(8.0);
        ui.randomize_reset_rng(self);

        ui.add_space(8.0);
        ui.subheading("Key");
        for i in 0..8 {
            if ui.u32_hex_edit(&mut self.key[i]).lost_focus() {
                self.rng = Hc256::with_key_and_iv_u32(self.key, self.iv);
            }
        }

        ui.add_space(8.0);
        ui.subheading("Nonce");
        for i in 0..8 {
            if ui.u32_hex_edit(&mut self.iv[i]).lost_focus() {
                self.rng = Hc256::with_key_and_iv_u32(self.key, self.iv);
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
        rng.fill(&mut self.iv);
        self.rng = Hc256::with_key_and_iv_u32(self.key, self.iv);
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
