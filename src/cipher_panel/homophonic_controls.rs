use ciphers::substitution::homophonic::Homophonic;
use rand::{thread_rng, Rng};

use crate::{cipher_panel::CipherFrame, ui_elements::UiElements};

pub struct HomophonicFrame {
    cipher: Homophonic,
    characters: String,
    group_sizes: String,
    seed: u64,
}

impl Default for HomophonicFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            characters: String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
            group_sizes: String::from("40, 7, 15, 25, 60, 15, 10, 30, 35, 3, 3, 20, 15, 35, 35, 10, 3, 30, 30, 45, 15, 5, 10, 3, 10, 3"),
            seed: 0xBAD5EED0BAD5EED0,
        }
    }
}

impl HomophonicFrame {}

impl CipherFrame for HomophonicFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/substitution/homophonic.rs",
        );
        ui.add_space(8.0);

        ui.subheading("Define Code Groups");
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.subheading("Encryption Seed");
                if ui.button("🎲").clicked() {
                    self.seed = thread_rng().gen();
                }
            });
            ui.add_space(4.0);
            ui.subheading("Characters");
            ui.text_edit_singleline(&mut self.characters);
            ui.add_space(4.0);
            ui.subheading("Frequency");
            ui.text_edit_singleline(&mut self.group_sizes);
        });
        ui.add_space(8.0);

        ui.subheading("Null Rate");
        ui.label("Probability that a null group will be inserted before or after each real group.");
        ui.add(egui::DragValue::new(&mut self.cipher.null_rate).speed(0.1));
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.subheading("Encryption Seed");
            if ui.button("🎲").clicked() {
                self.cipher.enc_seed = thread_rng().gen();
            }
        });
        ui.label("Determines the pseudorandom selections made during encryption.");
        ui.u64_hex_edit(&mut self.cipher.enc_seed);
    }

    fn randomize(&mut self) {
        // self.cipher.set_groups(characters, groups_sizes, seed)
    }

    crate::simple_cipher! {}
}
