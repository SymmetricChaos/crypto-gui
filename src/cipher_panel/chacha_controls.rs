use ciphers::digital::{chacha::ChaCha, chacha_extended_nonce::ChaChaExtendedNonce};
use egui::{DragValue, Slider};
use rand::{thread_rng, Rng};

use crate::ui_elements::UiElements;

use super::CipherFrame;

pub struct ChaChaFrame {
    regular: ChaCha,
    extended: ChaChaExtendedNonce,
    ex: bool,
}

impl Default for ChaChaFrame {
    fn default() -> Self {
        Self {
            regular: Default::default(),
            extended: Default::default(),
            ex: false,
        }
    }
}

impl ChaChaFrame {
    fn start_state(&self) -> String {
        let mut out = String::new();

        let state = if self.ex {
            self.extended.create_state(self.extended.ctr)
        } else {
            self.regular.create_state(self.regular.ctr)
        };

        for line in state.chunks_exact(4) {
            for word in line {
                out.push_str(&format!("{:08x?}  ", word))
            }
            out.push('\n')
        }

        out
    }
}

impl CipherFrame for ChaChaFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.randomize_reset(self);
        ui.add_space(16.0);

        ui.add_space(8.0);
        ui.selectable_value(&mut self.ex, false, "64-bit Nonce");
        ui.selectable_value(&mut self.ex, true, "96-bit Nonce");

        ui.add_space(8.0);
        ui.subheading("Key");
        if self.ex {
            for i in 0..4 {
                ui.add(DragValue::new(&mut self.extended.key[i]).hexadecimal(8, false, false));
            }
        } else {
            for i in 0..4 {
                ui.add(DragValue::new(&mut self.regular.key[i]).hexadecimal(8, false, false));
            }
        }

        ui.add_space(8.0);
        ui.subheading("Nonce");
        ui.label("A nonce should never be reused with the same key.");
        if self.ex {
            for i in 0..3 {
                ui.add(DragValue::new(&mut self.extended.nonce[i]).hexadecimal(8, false, false));
            }
        } else {
            for i in 0..2 {
                ui.add(DragValue::new(&mut self.regular.nonce[i]).hexadecimal(8, false, false));
            }
        }
        ui.add_space(8.0);
        ui.subheading("Counter");
        ui.label("The counter ensures that each block of the keystream is different. It can usually be left to start at zero.");
        if self.ex {
            ui.add(DragValue::new(&mut self.regular.ctr).hexadecimal(8, false, false));
        } else {
            ui.add(DragValue::new(&mut self.regular.ctr).hexadecimal(16, false, false));
        }

        ui.add_space(8.0);
        ui.subheading("Number of Rounds");
        ui.horizontal(|ui| {
            if ui.small_button("ChaCha8").clicked() {
                self.regular.rounds = 8;
                self.extended.rounds = 8;
            }
            if ui.small_button("ChaCha12").clicked() {
                self.regular.rounds = 12;
                self.extended.rounds = 12;
            }
            if ui.small_button("ChaCha20").clicked() {
                self.regular.rounds = 20;
                self.extended.rounds = 20;
            }
        });
        if ui
            .add(Slider::new(&mut self.regular.rounds, 2..=20))
            .changed()
        {
            self.extended.rounds = self.regular.rounds
        }
        ui.add_space(8.0);

        ui.subheading("Starting State");
        ui.label(self.start_state());
    }

    fn cipher(&self) -> &dyn ciphers::Cipher {
        &self.regular
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        if self.ex {
            rng.fill(&mut self.extended.key);
            rng.fill(&mut self.extended.nonce);
        } else {
            rng.fill(&mut self.regular.key);
            rng.fill(&mut self.regular.nonce);
        }
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
