use std::str::FromStr;

use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::{
    digital::{rsa::Rsa, ByteFormat},
    Cipher,
};
use egui::Ui;
use num::BigUint;
use num_prime::{nt_funcs::is_prime, RandPrime};
use rand::thread_rng;

#[derive(Default)]
pub struct RsaFrame {
    cipher: Rsa,
    p: String,
    p_num: BigUint,
    q: String,
    q_num: BigUint,
}

impl RsaFrame {
    fn run_ksa(&mut self) {
        self.cipher.set_key(&self.p_num, &self.q_num)
    }
}

impl CipherFrame for RsaFrame {
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

        ui.subheading("Prime (p)");
        ui.horizontal(|ui| {
            if ui.control_string(&mut self.p).changed() {
                self.p = self
                    .p
                    .chars()
                    .filter(|c| c.is_ascii_digit())
                    .take(38)
                    .collect();
                self.p_num =
                    BigUint::from_str(&self.p).expect("invalid inputs should be filtered out")
            };
            if ui
                .button("🎲")
                .on_hover_text("random 64-bit prime")
                .clicked()
            {
                self.p_num = thread_rng().gen_prime(64, None);
                self.p = self.p_num.to_str_radix(10);
            }
            match is_prime(&self.p_num, None) {
                num_prime::Primality::Yes => ui.label("prime"),
                num_prime::Primality::No => ui.error_text("NOT PRIME"),
                num_prime::Primality::Probable(f) => ui.label(format!("prime ({:.3})", f)),
            }
        });

        ui.add_space(8.0);

        ui.subheading("Prime (q)");
        ui.horizontal(|ui| {
            if ui.control_string(&mut self.q).changed() {
                self.q = self
                    .q
                    .chars()
                    .filter(|c| c.is_ascii_digit())
                    .take(38)
                    .collect();
                self.q_num =
                    BigUint::from_str(&self.q).expect("invalid inputs should be filtered out")
            };
            if ui
                .button("🎲")
                .on_hover_text("random 64-bit prime")
                .clicked()
            {
                self.q_num = thread_rng().gen_prime(64, None);
                self.q = self.q_num.to_str_radix(10);
            }
            match is_prime(&self.q_num, None) {
                num_prime::Primality::Yes => ui.label("prime"),
                num_prime::Primality::No => ui.error_text("NOT PRIME"),
                num_prime::Primality::Probable(f) => ui.label(format!("prime ({:.3})", f)),
            }
        });

        ui.add_space(16.0);

        if ui.button("Calculate Keys").clicked() {
            self.run_ksa()
        }
        ui.add_space(16.0);

        ui.subheading(format!("Product (n) {}-bits", self.cipher.n.bits()));
        ui.label(format!("{}", &self.cipher.n));

        ui.add_space(16.0);

        ui.subheading("Public Key");
        ui.label("To use the public only n (the product of the primes) and e (a small constant) are needed");
        ui.label(format!("e = {}", self.cipher.e));

        ui.add_space(8.0);

        ui.subheading("Private Key");
        ui.label("To use the private only n (the product of the primes) and d (the inverse of e) are needed");
        ui.label(format!("d = {}", self.cipher.d));

        ui.add_space(16.0);
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();

        self.p_num = rng.gen_prime(64, None);
        self.p = self.p_num.to_str_radix(10);

        self.q_num = rng.gen_prime(64, None);
        self.q = self.q_num.to_str_radix(10);

        self.run_ksa();
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
