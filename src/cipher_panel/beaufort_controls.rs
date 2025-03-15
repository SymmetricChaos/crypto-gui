use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::{
    polyalphabetic::{Beaufort, PolyMode},
    Cipher,
};
use egui::{Slider, TextEdit, TextStyle, Ui};
use rand::{thread_rng, Rng};
use strum::IntoEnumIterator;
use utils::{
    preset_alphabet::Alphabet,
    text_functions::{filter_string, random_string_sample_replace},
};

pub struct BeaufortFrame {
    cipher: Beaufort,
    alphabet_string: String,
}

impl Default for BeaufortFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            alphabet_string: Alphabet::BasicLatin.into(),
        }
    }
}

impl CipherFrame for BeaufortFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/polyalphabetic/beaufort.rs",
        );
        ui.add_space(8.0);

        ui.randomize_reset_cipher(self);
        ui.add_space(16.0);

        ui.group(|ui| {
            ui.subheading("Common Alphabets");
            ui.horizontal(|ui| {
                for alphabet in [
                    Alphabet::Alphanumeric,
                    Alphabet::BasicLatin,
                    Alphabet::Ascii94,
                    Alphabet::Base64,
                ] {
                    if ui.button(alphabet.name()).clicked() {
                        self.cipher.assign_alphabet(alphabet.slice());
                        for keyword in self.cipher.keywords.iter_mut() {
                            filter_string(keyword, &self.alphabet_string)
                        }
                    }
                }
            });
        });

        ui.subheading("Alphabet");
        if ui.control_string(&mut self.alphabet_string).changed() {
            self.cipher.assign_alphabet(&self.alphabet_string);
            for keyword in self.cipher.keywords.iter_mut() {
                filter_string(keyword, &self.alphabet_string)
            }
        }
        ui.add_space(10.0);

        ui.group(|ui| {
            ui.subheading("Mode");
            ui.horizontal(|ui| {
                for variant in PolyMode::iter() {
                    ui.selectable_value(&mut self.cipher.mode, variant, variant.to_string());
                }
            });
        });

        ui.add_enabled_ui(self.cipher.mode == PolyMode::ProgKey, |ui| {
            ui.subheading("Step size");
            let alpha_range = 0..=(self.cipher.alphabet_len() - 1);
            ui.add(Slider::new(&mut self.cipher.prog_shift, alpha_range));
            ui.add_space(8.0);
        });

        ui.horizontal(|ui| {
            if self.cipher.multikey {
                ui.subheading("Keywords");
            } else {
                ui.subheading("Keyword ");
            }

            ui.separator();
            ui.checkbox(&mut self.cipher.multikey, "Multikey");
            ui.add_space(4.0);
            if self.cipher.multikey {
                if ui.button("+").on_hover_text("add keyword").clicked() {
                    if self.cipher.keywords.len() <= 9 {
                        self.cipher.keywords.push(String::new())
                    }
                }
                if ui.button("-").on_hover_text("remove keyword").clicked() {
                    if self.cipher.keywords.len() >= 2 {
                        self.cipher.keywords.pop();
                    }
                }
            }
        });

        if self.cipher.multikey {
            for keyword in self.cipher.keywords.iter_mut() {
                if ui.control_string(keyword).changed() {
                    filter_string(keyword, &self.alphabet_string)
                }
            }
        } else {
            ui.add(TextEdit::singleline(&mut self.cipher.keywords[0]).font(TextStyle::Monospace));
        }
        ui.add_space(16.0);
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        for keyword in self.cipher.keywords.iter_mut() {
            *keyword =
                random_string_sample_replace(&self.alphabet_string, rng.gen_range(3..12), &mut rng);
        }
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
