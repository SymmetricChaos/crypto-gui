use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::{substitution::Plugboard, Cipher};
use egui::Ui;
use rand::thread_rng;
use utils::{preset_alphabet::Alphabet, text_functions::random_sample};

#[derive(Default)]
pub struct PlugboardFrame {
    cipher: Plugboard,
    pairs: String,
}

impl CipherFrame for PlugboardFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.randomize_reset(self);

        ui.add_space(16.0);
        ui.subheading("Plugboard Pairs");
        if ui.control_string(&mut self.pairs).changed() {
            match self.cipher.set_plugboard(&self.pairs) {
                Ok(_) => (),
                Err(e) => {
                    ui.error_text(&e.inner());
                }
            }
        };

        let nrows = 8;
        let ncols = 8;
        ui.columns(ncols, |columns| {
            let mut ctr = 0;
            let mut col = 0;
            for pair in self.cipher.show_settings() {
                columns[col].mono_strong(pair);
                ctr += 1;
                if ctr % nrows == 0 {
                    col += 1
                }
            }
        });
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        let alpha = random_sample(Alphabet::BasicLatin.slice(), 14, &mut thread_rng());
        let mut cs = alpha.chars();
        self.pairs.clear();
        for _ in 0..7 {
            self.pairs.push(cs.next().unwrap());
            self.pairs.push(cs.next().unwrap());
            self.pairs.push(' ');
        }

        self.cipher
            .set_plugboard(&self.pairs)
            .expect("error randomly generating plugboard")
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
