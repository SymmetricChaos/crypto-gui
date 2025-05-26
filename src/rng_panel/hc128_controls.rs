use super::ClassicRngFrame;
use crate::ui_elements::{generate_randoms_box, UiElements};
use rand::{thread_rng, Rng};
use rngs::hc128::Hc128;

pub struct Hc128Frame {
    rng: Hc128,
    key: [u32; 4],
    iv: [u32; 4],
    n_random: usize,
    randoms: String,
}

impl Default for Hc128Frame {
    fn default() -> Self {
        Self {
            rng: Default::default(),
            key: [0; 4],
            iv: [0; 4],
            n_random: 5,
            randoms: String::new(),
        }
    }
}

impl Hc128Frame {}

impl ClassicRngFrame for Hc128Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/rngs/src/hc128.rs",
        );

        ui.add_space(8.0);
        if ui.button("Randomize").clicked() {
            self.randomize()
        }
        if ui.button("Reset").clicked() {
            self.reset()
        }

        ui.add_space(8.0);
        ui.subheading("Key");
        for i in 0..4 {
            if ui.u32_hex_edit(&mut self.key[i]).lost_focus() {
                self.rng = Hc128::with_key_and_iv_u32(self.key, self.iv);
            }
        }

        ui.add_space(8.0);
        ui.subheading("Nonce");
        for i in 0..4 {
            if ui.u32_hex_edit(&mut self.iv[i]).lost_focus() {
                self.rng = Hc128::with_key_and_iv_u32(self.key, self.iv);
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
        rng.fill(&mut self.iv);
        self.rng = Hc128::with_key_and_iv_u32(self.key, self.iv);
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
