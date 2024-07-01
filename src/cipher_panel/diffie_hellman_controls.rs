use super::CipherFrame;

use crate::ui_elements::UiElements;
use ciphers::digital::diffie_hellman::DiffieHellman;
use ciphers::traits::Cipher;
use eframe::egui::Ui;
use rand::{thread_rng, Rng};
use utils::math_functions::is_prime32;

pub struct DiffieHellmanFrame {
    cipher: DiffieHellman,
    g_is_valid: bool,
    m_is_prime: bool,
}

impl Default for DiffieHellmanFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            g_is_valid: true,
            m_is_prime: true,
        }
    }
}

impl DiffieHellmanFrame {}

impl CipherFrame for DiffieHellmanFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/digital/diffie_hellman.rs",
        );

        ui.subheading("Modulus");
        if ui.u32_drag_value_dec(&mut self.cipher.modulus).changed() {
            self.m_is_prime = is_prime32(self.cipher.modulus)
        }
        if !self.m_is_prime {
            ui.error_text(
                "The modulus should be prime to maximize the size of the multiplicative group and simplify selection of the base",
            );
        } else {
            ui.error_text("");
        }

        ui.add_space(8.0);
        ui.subheading("Base");
        if ui.u32_drag_value_dec(&mut self.cipher.generator).changed() {
            self.cipher.generator = self.cipher.generator.clamp(2, self.cipher.modulus - 1);
            self.g_is_valid = self.cipher.g_is_valid();
        }
        if !self.g_is_valid {
            ui.error_text(
                "Base must be coprime to the modulus so that it is a generator of the multiplicative group",
            );
        } else {
            ui.error_text("");
        }

        ui.add_space(8.0);
        ui.subheading("Private Keys with Corresponding Public Keys");
        ui.horizontal(|ui| {
            if ui.button("-").clicked() {
                if self.cipher.private_keys.len() > 2 {
                    self.cipher.private_keys.pop();
                }
            };
            ui.label(format!("{}", self.cipher.private_keys.len()));
            if ui.button("+").clicked() {
                if self.cipher.private_keys.len() < 10 {
                    self.cipher.private_keys.push(2);
                }
            };
        });
        for (i, p) in self.cipher.public_keys().iter().enumerate() {
            ui.horizontal(|ui| {
                ui.u32_drag_value_dec(&mut self.cipher.private_keys[i]);
                ui.label(p.to_string());
            });
        }

        ui.add_space(8.0);
        ui.subheading("Shared Secret Key");
        ui.label(self.cipher.shared_key().to_string());

        ui.add_space(8.0);
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        for p in self.cipher.private_keys.iter_mut() {
            *p = rng.gen_range(2..=self.cipher.modulus - 1)
        }
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
