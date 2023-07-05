use ciphers::shamir::ShamirSecretSharing;
use egui::Slider;
use utils::{
    functions::filter_string,
    math_functions::{is_prime32, polynomial_string},
    preset_alphabet::Alphabet,
};

use crate::ui_elements::{control_string, error_text};

use super::CipherFrame;

pub struct ShamirSecretSharingFrame {
    cipher: ShamirSecretSharing,
    modulus_string: String,
    polynomial_string: String,
}

impl Default for ShamirSecretSharingFrame {
    fn default() -> Self {
        let cipher = ShamirSecretSharing::default();
        Self {
            modulus_string: format!("{}", cipher.modulus),
            polynomial_string: String::from("65, 2347, 542"),
            cipher,
        }
    }
}

impl CipherFrame for ShamirSecretSharingFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.label("Shares");
        ui.add(Slider::new(&mut self.cipher.shares, 3..=12));
        ui.add_space(8.0);

        ui.label("Threshold");
        ui.add(Slider::new(
            &mut self.cipher.threshold,
            3..=self.cipher.shares,
        ));
        ui.add_space(8.0);

        ui.label("Polynomial");
        if control_string(ui, &mut self.polynomial_string).lost_focus() {
            self.cipher.sting_to_vec(&self.polynomial_string)
        }
        ui.label(polynomial_string(&self.cipher.polynomial, true));
        ui.add_space(8.0);

        ui.label("Field Size");
        if control_string(ui, &mut self.modulus_string).changed() {
            filter_string(&mut self.modulus_string, Alphabet::Digits0.into());
            match i32::from_str_radix(&self.modulus_string, 10) {
                Ok(n) => match is_prime32(n as u32) {
                    true => self.cipher.modulus = n,
                    false => {
                        ui.label(error_text("field size must be prime"));
                    }
                },
                Err(e) => {
                    ui.label(error_text(e.to_string()));
                }
            }
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
