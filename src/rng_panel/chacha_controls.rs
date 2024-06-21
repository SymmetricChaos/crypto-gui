use super::ClassicRngFrame;
use crate::ui_elements::{generate_random_u32s_box, UiElements};
use egui::Slider;
use rand::{thread_rng, Rng};
use rngs::chacha::ChaCha;

pub struct ChaChaFrame {
    rng: ChaCha,
    n_random: usize,
    randoms: String,
}

impl Default for ChaChaFrame {
    fn default() -> Self {
        Self {
            rng: Default::default(),
            n_random: 5,
            randoms: String::new(),
        }
    }
}

impl ChaChaFrame {
    fn start_state(&self) -> String {
        let mut out = String::new();

        let state = self.rng.create_state();

        for line in state.chunks_exact(4) {
            for word in line {
                out.push_str(&format!("{:08x?}  ", word))
            }
            out.push('\n')
        }

        out
    }
}

impl ClassicRngFrame for ChaChaFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        // ui.hyperlink_to(
        //     "see the code",
        //     "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/digital/chacha.rs",
        // );
        // ui.add_space(8.0);

        ui.add_space(16.0);

        ui.add_space(8.0);
        ui.subheading("Key");

        for i in 0..4 {
            ui.u32_drag_value(&mut self.rng.key[i]);
        }

        ui.add_space(8.0);
        ui.subheading("Nonce");
        ui.label("A nonce should never be reused with the same key.");

        for i in 0..2 {
            ui.u32_drag_value(&mut self.rng.nonce[i]);
        }

        ui.add_space(8.0);
        ui.subheading("Counter");
        ui.label("The counter ensures that each block of the keystream is different. It can usually be left to start at zero.");
        ui.u64_drag_value(&mut self.rng.ctr);

        ui.add_space(8.0);
        ui.subheading("Number of Rounds");
        ui.horizontal(|ui| {
            if ui.small_button("ChaCha8").clicked() {
                self.rng.rounds = 8;
            }
            if ui.small_button("ChaCha12").clicked() {
                self.rng.rounds = 12;
            }
            if ui.small_button("ChaCha20").clicked() {
                self.rng.rounds = 20;
            }
        });
        ui.add(Slider::new(&mut self.rng.rounds, 2..=20)).changed();
        ui.add_space(8.0);

        ui.subheading("Starting State");
        ui.label(self.start_state());

        ui.add_space(8.0);
        generate_random_u32s_box(ui, &mut self.rng, &mut self.n_random, &mut self.randoms);
    }

    fn rng(&self) -> &dyn rngs::ClassicRng {
        &self.rng
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();

        rng.fill(&mut self.rng.key);
        rng.fill(&mut self.rng.nonce);
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
