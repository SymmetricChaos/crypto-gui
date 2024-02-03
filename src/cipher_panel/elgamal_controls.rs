use std::str::FromStr;

use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::{
    digital::{elgamal::ElGamal, ByteFormat},
    Cipher,
};
use egui::Ui;
use num::BigUint;
use num_prime::{nt_funcs::is_prime, RandPrime};
use rand::thread_rng;

#[derive(Default)]
pub struct ElGamalFrame {
    cipher: ElGamal,
    group_size: String,
    group_size_n: BigUint,
    message_key: String,
    message_key_n: BigUint,
    generator: String,
    generator_n: BigUint,
    private_key: String,
    private_key_n: BigUint,
}

impl ElGamalFrame {
    fn run_ksa(&mut self) {
        self.cipher
            .set_key(&self.group_size_n, &self.generator_n, &self.private_key_n)
    }
}

impl CipherFrame for ElGamalFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.randomize_reset(self);
        ui.add_space(16.0);

        ui.collapsing("Input Format", |ui| {
            ui.label("Input can be text (interpreted as UTF-8), hexadecimal representing bytes, or Base64 representing bytes.");
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut self.cipher.input_format,
                    ByteFormat::Utf8,
                    "Text (UTF-8)",
                );
                ui.selectable_value(
                    &mut self.cipher.input_format,
                    ByteFormat::Hex,
                    "Hexadecimal",
                );
                ui.selectable_value(&mut self.cipher.input_format, ByteFormat::Utf8, "Base64");
            });
        });

        ui.add_space(8.0);

        ui.collapsing("Output Format", |ui| {
            ui.label("Output can be text (but information will be lost if the encrypted bytes are not valid UTF-8), hexadecimal representing bytes, or Base64 representing bytes.");
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut self.cipher.output_format,
                    ByteFormat::Utf8,
                    "Text (UTF-8)",
                );
                ui.selectable_value(
                    &mut self.cipher.output_format,
                    ByteFormat::Hex,
                    "Hexadecimal",
                );
                ui.selectable_value(&mut self.cipher.output_format, ByteFormat::Base64, "Base64");
            });
        });
        ui.add_space(16.0);

        ui.subheading("Group Size");
        ui.horizontal(|ui| {
            if ui.control_string(&mut self.group_size).changed() {
                self.group_size = self
                    .group_size
                    .chars()
                    .filter(|c| c.is_ascii_digit())
                    .take(38)
                    .collect();
                self.group_size_n = BigUint::from_str(&self.group_size)
                    .expect("invalid inputs should be filtered out")
            };
            if ui
                .button("🎲")
                .on_hover_text("random 64-bit prime")
                .clicked()
            {
                self.group_size_n = thread_rng().gen_prime(64, None);
                self.group_size = self.group_size_n.to_str_radix(10);
            }
            match is_prime(&self.group_size_n, None) {
                num_prime::Primality::Yes => ui.label("prime"),
                num_prime::Primality::No => ui.error_text("NOT PRIME"),
                num_prime::Primality::Probable(f) => ui.label(format!("prime ({:.3})", f)),
            }
        });

        ui.add_space(8.0);

        ui.add_space(16.0);

        if ui.button("Calculate Key").clicked() {
            self.run_ksa()
        }
        ui.add_space(16.0);

        ui.subheading("Public Key");
        ui.label("To use the public only n (the product of the primes) and e (a small constant) are needed");

        ui.add_space(8.0);

        ui.subheading("Private Key");
        ui.label("To use the private only n (the product of the primes) and d (the inverse of e) are needed");

        ui.add_space(16.0);
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();

        self.group_size_n = rng.gen_prime(64, None);
        self.group_size = self.group_size_n.to_str_radix(10);

        self.run_ksa();
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
