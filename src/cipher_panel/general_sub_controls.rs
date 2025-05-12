use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::substitution::GeneralSubstitution;
use egui::Ui;
use rand::thread_rng;
use utils::{preset_alphabet::Alphabet, text_functions::shuffled_str};

pub struct GeneralSubstitutionFrame {
    cipher: GeneralSubstitution,
    pt_alphabet_string: String,
    ct_alphabet_string: String,
}

impl Default for GeneralSubstitutionFrame {
    fn default() -> Self {
        let mut frame = Self {
            cipher: Default::default(),
            pt_alphabet_string: String::from(Alphabet::BasicLatin),
            ct_alphabet_string: String::from("BANYEMSWCUQPRKOVTIDHJXLZFG"),
        };
        frame.cipher.assign_ct_alphabet(&frame.ct_alphabet_string);
        frame
    }
}

impl CipherFrame for GeneralSubstitutionFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/substitution/substitution.rs",
        );
        ui.add_space(8.0);

        ui.randomize_reset_cipher(self);
        ui.add_space(16.0);

        ui.subheading("Plaintext Alphabet");
        if ui.control_string(&mut self.pt_alphabet_string).changed() {
            self.cipher.assign_pt_alphabet(&self.pt_alphabet_string);
        }
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.subheading("Ciphertext Alphabet");
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                self.ct_alphabet_string = shuffled_str(&self.pt_alphabet_string, &mut thread_rng());
                self.cipher.assign_ct_alphabet(&self.ct_alphabet_string);
            }
        });

        if ui.control_string(&mut self.ct_alphabet_string).changed() {
            self.cipher.assign_ct_alphabet(&self.ct_alphabet_string);
        }
        ui.add_space(16.0);

        if ui.button("Atbash").clicked() {
            self.pt_alphabet_string = String::from(Alphabet::BasicLatin);
            self.ct_alphabet_string = String::from("ZYXWVUTSRQPONMLKJIHGFEDCBA");
            self.cipher.assign_pt_alphabet(&self.pt_alphabet_string);
            self.cipher.assign_ct_alphabet(&self.ct_alphabet_string);
        }
        ui.add_space(16.0);
    }

    fn randomize(&mut self) {
        self.ct_alphabet_string = shuffled_str(&self.pt_alphabet_string, &mut thread_rng());
        self.cipher.assign_ct_alphabet(&self.ct_alphabet_string);
    }

    crate::simple_cipher! {}
}
