use ciphers::{digital::block_ciphers::ascon::ascon80pq::Ascon80pq, Cipher};
use rand::{thread_rng, Rng};
use strum::IntoEnumIterator;
use utils::byte_formatting::ByteFormat;

use crate::ui_elements::UiElements;

use super::CipherFrame;

pub struct Ascon80pqFrame {
    cipher: Ascon80pq,
    ad: String,
    ad_mode: ByteFormat,
}

impl Default for Ascon80pqFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            ad: Default::default(),
            ad_mode: ByteFormat::Hex,
        }
    }
}

impl CipherFrame for Ascon80pqFrame {
    fn ui(&mut self, ui: &mut egui::Ui, errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/tree/master/ciphers/src/digital/block_ciphers/ascon",
        );
        ui.add_space(8.0);

        ui.byte_io_mode_cipher(
            &mut self.cipher.input_format,
            &mut self.cipher.output_format,
        );
        ui.add_space(8.0);

        ui.randomize_reset_cipher(self);
        ui.add_space(16.0);

        ui.horizontal(|ui| {
            ui.subheading("Key");
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                let mut rng = thread_rng();
                self.cipher.subkeys[0] = rng.gen();
                self.cipher.subkeys[1] = rng.gen();
                self.cipher.subkeys[2] = rng.gen();
                self.cipher.subkeys[2] &= 0xffffffff00000000;
            }
        });
        ui.label(
            "Ascon-80pq uses a 160-bit key presented here as three wo 64-bit words with the lowest 32 bits of the last set to zero.",
        );
        for i in 0..2 {
            ui.u64_hex_edit(&mut self.cipher.subkeys[i]);
        }
        if ui.u64_hex_edit(&mut self.cipher.subkeys[2]).changed() {
            self.cipher.subkeys[2] &= 0xffffffff00000000;
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
        ui.label("Ascon-80pq uses a 128-bit nonce.");
        for i in 0..2 {
            ui.u64_hex_edit(&mut self.cipher.nonce[i]);
        }

        ui.subheading("Associated Data");
        ui.label("Arbitrary data can be associated with the message. This is usually data that cannot be encrypted such as routing information. The tag of Ascon-128 authenticates this data.");
        ui.horizontal(|ui| {
            for variant in ByteFormat::iter() {
                if ui
                    .selectable_value(&mut self.ad_mode, variant, variant.to_string())
                    .clicked()
                {
                    match self.ad_mode.text_to_bytes(&self.ad) {
                        Ok(v) => self.cipher.associated_data = v,
                        Err(_) => errors.push_str("Error formatting associated data as bytes"),
                    }
                }
            }
        });
        if ui.control_string(&mut self.ad).lost_focus() {
            match self.ad_mode.text_to_bytes(&self.ad) {
                Ok(v) => self.cipher.associated_data = v,
                Err(_) => {
                    errors.push_str("Error formatting associated data as bytes");
                    self.cipher.associated_data.clear();
                }
            }
        }
    }


    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.cipher.subkeys[0] = rng.gen();
        self.cipher.subkeys[1] = rng.gen();
        self.cipher.subkeys[2] = rng.gen();
        self.cipher.subkeys[2] &= 0xffffffff00000000;
        self.cipher.nonce[0] = rng.gen();
        self.cipher.nonce[1] = rng.gen();
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
