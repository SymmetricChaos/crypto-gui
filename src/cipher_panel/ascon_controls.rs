use ciphers::{
    digital::block_ciphers::ascon::{Ascon128, AsconVariant},
    Cipher,
};
use rand::{thread_rng, Rng};

use crate::ui_elements::UiElements;

use super::CipherFrame;

#[derive(Default)]
pub struct AsconFrame {
    cipher: Ascon128,
}

impl CipherFrame for AsconFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/tree/master/ciphers/src/digital/block_ciphers/ascon.rs",
        );
        ui.add_space(8.0);

        ui.selectable_value(
            &mut self.cipher.variant,
            AsconVariant::Ascon128,
            "Ascon-128",
        );
        ui.selectable_value(
            &mut self.cipher.variant,
            AsconVariant::Ascon128a,
            "Ascon-128a",
        );

        ui.randomize_reset(self);
        ui.add_space(16.0);

        ui.horizontal(|ui| {
            ui.subheading("Key");
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                let mut rng = thread_rng();
                self.cipher.subkeys[0] = rng.gen();
                self.cipher.subkeys[1] = rng.gen();
            }
        });
        ui.label("Ascon-128 uses a 128-bit key presented here as two 64-bit words.");
        for i in 0..2 {
            ui.u64_hex_edit(&mut self.cipher.subkeys[i]);
        }
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.subheading("Nonce");
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                let mut rng = thread_rng();
                self.cipher.nonce[0] = rng.gen();
                self.cipher.nonce[1] = rng.gen();
            }
        });
        ui.label("Ascon-128 uses a 128-bit nonce presented here as two 64-bit words.");
        for i in 0..2 {
            ui.u64_hex_edit(&mut self.cipher.nonce[i]);
        }
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.cipher.subkeys[0] = rng.gen();
        self.cipher.subkeys[1] = rng.gen();
        self.cipher.nonce[0] = rng.gen();
        self.cipher.nonce[1] = rng.gen();
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
