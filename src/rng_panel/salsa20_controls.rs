use super::ClassicRngFrame;
use crate::ui_elements::{generate_randoms_box, UiElements};
use egui::Slider;
use rand::{thread_rng, Rng};
use rngs::salsa20::Salsa20;

pub struct Salsa20Frame {
    rng: Salsa20,
    n_random: usize,
    randoms: String,
}

impl Default for Salsa20Frame {
    fn default() -> Self {
        Self {
            rng: Default::default(),
            n_random: 5,
            randoms: String::new(),
        }
    }
}

impl Salsa20Frame {
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

impl ClassicRngFrame for Salsa20Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/rngs/src/salsa20.rs",
        );

        ui.add_space(8.0);
        ui.subheading("Key");
        for i in 0..4 {
            if ui.u32_hex_edit(&mut self.rng.key[i]).changed() {
                self.rng.saved_keystream.clear();
            }
        }

        ui.add_space(8.0);
        ui.subheading("Nonce");
        ui.label("A nonce should never be reused with the same key.");
        for i in 0..2 {
            if ui.u32_hex_edit(&mut self.rng.key[i]).changed() {
                self.rng.saved_keystream.clear();
            }
        }

        ui.add_space(8.0);
        ui.subheading("Counter");
        ui.label("The counter ensures that each block of the keystream is different. It can usually be left to start at zero.");
        if ui.u64_hex_edit(&mut self.rng.ctr).changed() {
            self.rng.saved_keystream.clear();
        }

        ui.add_space(8.0);
        ui.subheading("Number of Rounds");
        ui.horizontal(|ui| {
            if ui.small_button("Salsa20/8").clicked() {
                self.rng.rounds = 8;
                self.rng.saved_keystream.clear();
            }
            if ui.small_button("Salsa20/12").clicked() {
                self.rng.rounds = 12;
                self.rng.saved_keystream.clear();
            }
            if ui.small_button("Salsa20/20").clicked() {
                self.rng.rounds = 20;
                self.rng.saved_keystream.clear();
            }
        });
        ui.add(Slider::new(&mut self.rng.rounds, 2..=20)).changed();
        ui.add_space(8.0);

        ui.subheading("Starting State");
        ui.label(self.start_state());

        ui.add_space(8.0);
        generate_randoms_box(ui, &mut self.rng, &mut self.n_random, &mut self.randoms);
    }

    fn rng(&self) -> &dyn rngs::ClassicRng {
        &self.rng
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.rng.saved_keystream.clear();
        rng.fill(&mut self.rng.key);
        rng.fill(&mut self.rng.nonce);
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
