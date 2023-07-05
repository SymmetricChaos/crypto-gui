use ciphers::shamir::ShamirSecretSharing;
use utils::{functions::filter_string, preset_alphabet::Alphabet};

use crate::ui_elements::control_string;

use super::CipherFrame;

pub struct ShamirSecretSharingFrame {
    cipher: ShamirSecretSharing,
    modulus_string: String,
}

impl Default for ShamirSecretSharingFrame {
    fn default() -> Self {
        let cipher = ShamirSecretSharing::default();
        Self {
            modulus_string: format!("{}", cipher.modulus),
            cipher,
        }
    }
}

impl CipherFrame for ShamirSecretSharingFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        if control_string(ui, &mut self.modulus_string).changed() {
            filter_string(&mut self.modulus_string, Alphabet::Digits0.into());
            // self.cipher.modulus = i32::from_str_radix(&self.modulus_string, 10);
        }
    }

    fn cipher(&self) -> &dyn ciphers::Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        todo!()
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
