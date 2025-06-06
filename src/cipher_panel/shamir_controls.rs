use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::digital::shamir_secret_sharing::ShamirSecretSharing;
use egui::Slider;
use utils::{math_functions::is_prime64, text_functions::filter_string};

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
            polynomial_string: String::from("65, 2347"),
            cipher,
        }
    }
}

impl CipherFrame for ShamirSecretSharingFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/digital/shamir_secret_sharing.rs",
        );
        ui.add_space(8.0);

        ui.subheading("Threshold");
        ui.label("Number of points needed to reconstruct the secret number.");
        ui.add(Slider::new(&mut self.cipher.threshold, 3..=20));

        ui.add_space(16.0);
        ui.subheading("Shares");
        ui.label("Number of points to create in total. Cannot be less than the threshold.");
        ui.add(Slider::new(&mut self.cipher.shares, 3..=40));
        if self.cipher.threshold > self.cipher.shares {
            self.cipher.shares = self.cipher.threshold;
        }

        // Nonrandom shares only needed for testing?
        // ui.checkbox(&mut self.cipher.random_shares, "Use Random Shares");
        // ui.add_space(8.0);

        ui.add_space(16.0);
        ui.subheading("Polynomial");
        ui.label(format!(
            "Exactly {} integers must be provided.",
            self.cipher.threshold - 1
        ));
        if ui.control_string(&mut self.polynomial_string).changed() {
            match self
                .cipher
                .polynomial_string_to_vec(&self.polynomial_string)
            {
                Ok(_) => (),
                Err(e) => {
                    ui.error_text(e);
                }
            }
        }
        ui.label(self.cipher.polynomial_string());
        ui.add_space(4.0);
        ui.label(
            "Note that the constant coefficient is not used. During calculation the secret is inserted there.",
        );
        ui.add_space(8.0);

        ui.subheading("Field Size");
        ui.label("A positive prime less than 2^64-1. The secret message cannot have a value larger than the field size.");
        if ui.control_string(&mut self.modulus_string).changed() {
            filter_string(&mut self.modulus_string, &"0123456789");
            match u64::from_str_radix(&self.modulus_string, 10) {
                Ok(n) => match n > 0 {
                    true => match is_prime64(n as u64) {
                        true => self.cipher.modulus = n,
                        false => {
                            ui.error_text("field size must be prime");
                        }
                    },
                    false => {
                        ui.error_text("field size must be positive");
                    }
                },
                Err(e) => {
                    ui.error_text(e.to_string());
                }
            }
        }
        ui.add_space(16.0);
    }

    fn randomize(&mut self) {
        todo!()
    }

    crate::simple_cipher! {}
}
