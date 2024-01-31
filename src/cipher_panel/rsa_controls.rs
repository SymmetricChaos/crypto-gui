use std::{num::ParseIntError, str::FromStr};

use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::{
    digital::{rsa::Rsa, ByteFormat},
    Cipher,
};
use egui::Ui;
use num::BigUint;
use num_prime::{nt_funcs::is_prime, PrimalityUtils, RandPrime};
use rand::{thread_rng, Rng};

#[derive(Default)]
pub struct RsaFrame {
    cipher: Rsa,
    p: String,
    p_num: BigUint,
    q: String,
    q_num: BigUint,
}

impl RsaFrame {
    fn run_ksa(&mut self) {}
}

impl CipherFrame for RsaFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.randomize_reset(self);
        ui.add_space(16.0);

        ui.subheading("Input Format");
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

        ui.add_space(8.0);

        ui.subheading("Output Format");
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

        ui.add_space(16.0);

        ui.subheading("Prime (p)");
        ui.horizontal(|ui| {
            if ui.control_string(&mut self.p).changed() {
                self.p = self
                    .p
                    .chars()
                    .filter(|c| c.is_ascii_digit())
                    .take(40)
                    .collect();
                self.p_num =
                    BigUint::from_str(&self.p).expect("invalid inputs should be filtered out")
            };
            if ui
                .button("🎲")
                .on_hover_text("random 128-bit prime")
                .clicked()
            {
                self.p_num = thread_rng().gen_prime(128, None);
                self.p = self.p_num.to_str_radix(10);
            }
            match is_prime(&self.p_num, None) {
                num_prime::Primality::Yes => ui.label("prime"),
                num_prime::Primality::No => ui.error_text("NOT PRIME"),
                num_prime::Primality::Probable(f) => ui.label(format!("prime ({:.3})", f)),
            }
        });

        ui.subheading("Prime (q)");
        ui.horizontal(|ui| {
            if ui.control_string(&mut self.q).changed() {
                self.q = self
                    .p
                    .chars()
                    .filter(|c| c.is_ascii_digit())
                    .take(40)
                    .collect();
                self.q_num =
                    BigUint::from_str(&self.q).expect("invalid inputs should be filtered out")
            };
            if ui
                .button("🎲")
                .on_hover_text("random 128-bit prime")
                .clicked()
            {
                self.q_num = thread_rng().gen_prime(128, None);
                self.q = self.q_num.to_str_radix(10);
            }
            match is_prime(&self.q_num, None) {
                num_prime::Primality::Yes => ui.label("prime"),
                num_prime::Primality::No => ui.error_text("NOT PRIME"),
                num_prime::Primality::Probable(f) => ui.label(format!("prime ({:.3})", f)),
            }
        });

        ui.subheading("Key (n)");
        ui.label(format!(
            "{} × {} = {}",
            self.p,
            self.q,
            &self.p_num * &self.q_num
        ));

        ui.add_space(16.0);
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();

        self.p_num = rng.gen_prime(128, None);
        self.p = self.p_num.to_str_radix(10);

        self.q_num = rng.gen_prime(128, None);
        self.q = self.q_num.to_str_radix(10);

        self.run_ksa();
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
