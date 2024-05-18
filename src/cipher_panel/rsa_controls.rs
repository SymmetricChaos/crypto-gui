use std::str::FromStr;

use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::{digital::rsa::Rsa, Cipher};
use egui::Ui;
use num::BigUint;
use num_prime::{nt_funcs::is_prime, PrimalityTestConfig, RandPrime};
use rand::thread_rng;

fn prime_string(ui: &mut Ui, s: &mut String, n: &mut BigUint) {
    let mut primality_test = PrimalityTestConfig::strict();
    primality_test.sprp_random_trials = 1;
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
        match is_prime(n, Some(primality_test)) {
            num_prime::Primality::Yes => ui.label("prime"),
            num_prime::Primality::No => ui.error_text("NOT PRIME"),
            num_prime::Primality::Probable(f) => ui.label(format!("prime ({:.3})", f)),
        }
    });
}

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
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/digital/rsa.rs",
        );
        ui.add_space(8.0);

        ui.randomize_reset(self);
        ui.add_space(16.0);

        ui.byte_io_mode(
            &mut self.cipher.input_format,
            &mut self.cipher.output_format,
        );
        ui.add_space(16.0);

        ui.subheading("Prime (p)");
        prime_string(ui, &mut self.p, &mut self.p_num);

        ui.add_space(8.0);

        ui.subheading("Prime (q)");
        prime_string(ui, &mut self.q, &mut self.q_num);

        ui.add_space(16.0);

        if ui.button("Calculate Keys").clicked() {
            self.run_ksa()
        }
        ui.add_space(16.0);

        ui.subheading(format!("Product (n) {}-bits", self.cipher.n.bits()));
        ui.label(format!("{}", &self.cipher.n));

        ui.add_space(16.0);

        ui.subheading("Public Key");
        ui.label("The public consists of n (the product of the primes) and e (a small constant) are needed");
        ui.label(format!("e = {}", self.cipher.e));

        ui.add_space(8.0);

        ui.subheading("Private Key");
        ui.label("The private consists of n (the product of the primes) and d (the inverse of e) are needed");
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
