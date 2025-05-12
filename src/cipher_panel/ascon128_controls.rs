use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::{
    digital::block_ciphers::ascon::{ascon128::Ascon128, Ascon128Variant},
    CipherError,
};
use rand::{thread_rng, Rng};
use strum::IntoEnumIterator;
use utils::byte_formatting::ByteFormat;

pub struct Ascon128Frame {
    input_format: ByteFormat,
    output_format: ByteFormat,
    cipher: Ascon128,
    ad: String,
    ad_mode: ByteFormat,
}

impl Default for Ascon128Frame {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            cipher: Default::default(),
            ad: Default::default(),
            ad_mode: ByteFormat::Hex,
        }
    }
}

impl CipherFrame for Ascon128Frame {
    fn ui(&mut self, ui: &mut egui::Ui, errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/tree/master/ciphers/src/digital/block_ciphers/ascon",
        );
        ui.add_space(8.0);

        ui.selectable_value(
            &mut self.cipher.variant,
            Ascon128Variant::Ascon128,
            "Ascon-128",
        );
        ui.selectable_value(
            &mut self.cipher.variant,
            Ascon128Variant::Ascon128a,
            "Ascon-128a",
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
            }
        });
        ui.label(format!("{} uses a 128-bit key.", self.cipher.variant));
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
        ui.label(format!("{} uses a 128-bit nonce.", self.cipher.variant));
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
        self.cipher.nonce[0] = rng.gen();
        self.cipher.nonce[1] = rng.gen();
    }

    fn reset(&mut self) {
        *self = Self::default()
    }

    fn encrypt_string(&self, text: &str) -> Result<String, ciphers::CipherError> {
        let bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| CipherError::input("byte format error"))?;

        Ok(self
            .output_format
            .byte_slice_to_text(&self.cipher.encrypt_bytes(&bytes)))
    }

    fn decrypt_string(&self, text: &str) -> Result<String, ciphers::CipherError> {
        let bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| CipherError::input("byte format error"))?;

        Ok(self
            .output_format
            .byte_slice_to_text(&self.cipher.decrypt_bytes(&bytes)?))
    }
}
