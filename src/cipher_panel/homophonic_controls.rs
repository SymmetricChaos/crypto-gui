use ciphers::substitution::homophonic::Homophonic;
use itertools::Itertools;
use rand::{thread_rng, Rng};
use utils::errors::GeneralError;

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

impl HomophonicFrame {
    fn parse_group_sizes(&self) -> Result<Vec<usize>, GeneralError> {
        let mut out = Vec::new();
        for s in self.group_sizes.split(',') {
            match usize::from_str_radix(s.trim(), 10) {
                Ok(n) => out.push(n),
                Err(e) => return Err(GeneralError::input(e.to_string())),
            }
        }
        Ok(out)
    }
}

impl CipherFrame for HomophonicFrame {
    fn ui(&mut self, ui: &mut egui::Ui, errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/substitution/homophonic.rs",
        );
        ui.add_space(8.0);

        if ui.button("Reset").clicked() {
            self.reset()
        }
        ui.add_space(16.0);

        ui.horizontal(|ui| {
            ui.subheading("Encryption Seed");
            if ui.button("🎲").clicked() {
                self.cipher.enc_seed = thread_rng().gen();
            }
        });
        ui.label("Determines the selection of code groups. Should be changed for every message.");
        ui.u64_hex_edit(&mut self.cipher.enc_seed);
        ui.add_space(8.0);

        ui.subheading("Null Rate");
        ui.label("Probability that a null group will be inserted before or after each real group.");
        ui.add(
            egui::DragValue::new(&mut self.cipher.null_rate)
                .speed(0.01)
                .range(0..=1),
        );
        ui.add_space(8.0);

        ui.group(|ui| {
            if ui.button("Create Code Groups").clicked() {
                match self.parse_group_sizes() {
                    Ok(group_sizes) => {
                        match self.cipher.set_groups(
                            self.characters.chars().collect_vec(),
                            group_sizes,
                            self.seed,
                        ) {
                            Ok(()) => (),
                            Err(e) => {
                                errors.clear();
                                errors.push_str(&e.to_string());
                            }
                        }
                    }
                    Err(e) => {
                        errors.clear();
                        errors.push_str(&e.to_string());
                    }
                };
            }
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.subheading("Seed");
                if ui.button("🎲").clicked() {
                    self.seed = thread_rng().gen();
                }
            });
            ui.u64_hex_edit(&mut self.seed);
            ui.add_space(8.0);
            ui.subheading("Characters");
            ui.text_edit_multiline(&mut self.characters);
            ui.add_space(8.0);
            ui.subheading("Groups Per Character");
            ui.label("Number of groups assigned to each character. Separate by commas.");
            ui.text_edit_multiline(&mut self.group_sizes);
        });
        ui.add_space(8.0);

        ui.collapsing("Code Groups", |ui| {
            for n in 0..self.cipher.characters.len() {
                ui.monospace(format!(
                    "{} = {}\n",
                    self.cipher.characters[n],
                    self.cipher.groups[n].join(", ")
                ));
            }
            ui.monospace(format!("NULLS = {}", self.cipher.nulls.join(", ")));
        });
        ui.add_space(8.0);
    }

    fn randomize(&mut self) {
        // self.cipher.set_groups(characters, groups_sizes, seed)
    }

    crate::simple_cipher! {}
}
