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
use rand::{thread_rng, Rng, RngCore};

fn prime_string(ui: &mut Ui, s: &mut String, n: &mut BigUint) {
    ui.horizontal(|ui| {
        if ui.control_string(s).changed() {
            *s = s.chars().filter(|c| c.is_ascii_digit()).take(38).collect();
            *n = BigUint::from_str(s).expect("invalid inputs should be filtered out")
        };
        if ui
            .button("🎲")
            .on_hover_text("random 64-bit prime")
            .clicked()
        {
            *n = thread_rng().gen_prime(64, None);
            *s = n.to_str_radix(10);
        }
        match is_prime(n, None) {
            num_prime::Primality::Yes => ui.label("prime"),
            num_prime::Primality::No => ui.error_text("NOT PRIME"),
            num_prime::Primality::Probable(f) => ui.label(format!("prime ({:.3})", f)),
        }
    });
}

#[derive(Default)]
pub struct ElGamalFrame {
    cipher: ElGamal,
    group_size: String,
    message_key: String,
    generator: String,
    private_key: String,
}

impl ElGamalFrame {
    fn run_ksa(&mut self) {
        self.cipher.set_key()
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
            self.cipher.set_key()
        });
        ui.add_space(16.0);

        ui.subheading("Group Size");
        prime_string(ui, &mut self.group_size, &mut self.cipher.group_size);

        ui.add_space(8.0);

        ui.subheading("Generator");
        if ui.control_string(&mut self.generator).changed() {
            self.generator = self
                .generator
                .chars()
                .filter(|c| c.is_ascii_digit())
                .take(38)
                .collect();
            self.cipher.generator =
                BigUint::from_str(&self.generator).expect("invalid inputs should be filtered out");
            self.cipher.set_key()
        }
        ui.add_space(8.0);

        ui.subheading("Private Key");
        if ui.control_string(&mut self.private_key).changed() {
            self.private_key = self
                .private_key
                .chars()
                .filter(|c| c.is_ascii_digit())
                .take(38)
                .collect();
            self.cipher.private_key = BigUint::from_str(&self.private_key)
                .expect("invalid inputs should be filtered out");
            self.cipher.set_key()
        }

        ui.add_space(8.0);

        ui.subheading("Message Key (to be changed for every message)");
        if ui.control_string(&mut self.message_key).changed() {
            self.message_key = self
                .message_key
                .chars()
                .filter(|c| c.is_ascii_digit())
                .take(38)
                .collect();
            self.cipher.message_key =
                BigUint::from_str(&self.message_key).expect("invalid inputs should be filtered out")
        }

        ui.add_space(16.0);

        if ui.button("Calculate Key").clicked() {
            self.run_ksa()
        }

        ui.label("Point");
        ui.label(format!("{}", self.cipher.point));

        ui.add_space(16.0);
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();

        self.cipher.group_size = rng.gen_prime(64, None);
        self.group_size = self.cipher.group_size.to_str_radix(10);

        self.cipher.private_key =
            BigUint::from(rng.gen_range(2..u64::MAX)) % &self.cipher.generator;
        self.private_key = self.cipher.private_key.to_str_radix(10);

        self.cipher.message_key =
            BigUint::from(rng.gen_range(2..u64::MAX)) % &self.cipher.generator;
        self.message_key = self.cipher.message_key.to_str_radix(10);

        self.cipher.generator = BigUint::from(rng.gen_range(2..u64::MAX)) % &self.cipher.generator;
        self.generator = self.cipher.generator.to_str_radix(10);

        self.run_ksa();
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
