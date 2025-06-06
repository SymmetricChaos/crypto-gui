use super::CipherFrame;
use crate::ui_elements::{
    block_cipher_iv_128, block_cipher_iv_32, block_cipher_iv_48, block_cipher_iv_64,
    block_cipher_iv_96, block_cipher_mode_and_padding, UiElements,
};
use ciphers::{
    digital::block_ciphers::simon::{
        simon128::{Simon128_128, Simon128_192, Simon128_256},
        simon32::Simon32_64,
        simon48::{Simon48_72, Simon48_96},
        simon64::{Simon64_128, Simon64_96},
        simon96::{Simon96_144, Simon96_96},
        SimonVariant,
    },
    Cipher,
};
use egui::Ui;
use rand::{thread_rng, Rng};
use strum::IntoEnumIterator;
use utils::byte_formatting::ByteFormat;

pub struct SimonFrame {
    cipher_32_64: Simon32_64,
    cipher_48_72: Simon48_72,
    cipher_48_96: Simon48_96,
    cipher_64_96: Simon64_96,
    cipher_64_128: Simon64_128,
    cipher_96_96: Simon96_96,
    cipher_96_144: Simon96_144,
    cipher_128_128: Simon128_128,
    cipher_128_192: Simon128_192,
    cipher_128_256: Simon128_256,
    selector: SimonVariant,
    key: String,
    input_format: ByteFormat,
    output_format: ByteFormat,
}

impl Default for SimonFrame {
    fn default() -> Self {
        Self {
            cipher_32_64: Default::default(),
            cipher_48_72: Default::default(),
            cipher_48_96: Default::default(),
            cipher_64_96: Default::default(),
            cipher_64_128: Default::default(),
            cipher_96_96: Default::default(),
            cipher_96_144: Default::default(),
            cipher_128_128: Default::default(),
            cipher_128_192: Default::default(),
            cipher_128_256: Default::default(),
            selector: Default::default(),
            key: Default::default(),
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
        }
    }
}

impl SimonFrame {
    fn run_ksa(&mut self) {
        let key_vec = ByteFormat::Hex.text_to_bytes(&self.key);

        if let Ok(vec) = key_vec {
            match self.selector {
                SimonVariant::Simon32_64 => self
                    .cipher_32_64
                    .ksa(vec.try_into().expect("wrong key size")),
                SimonVariant::Simon48_72 => self
                    .cipher_48_72
                    .ksa(vec.try_into().expect("wrong key size")),
                SimonVariant::Simon48_96 => self
                    .cipher_48_96
                    .ksa(vec.try_into().expect("wrong key size")),
                SimonVariant::Simon64_96 => self
                    .cipher_64_96
                    .ksa(vec.try_into().expect("wrong key size")),
                SimonVariant::Simon64_128 => self
                    .cipher_64_128
                    .ksa(vec.try_into().expect("wrong key size")),
                SimonVariant::Simon96_96 => self
                    .cipher_96_96
                    .ksa(vec.try_into().expect("wrong key size")),
                SimonVariant::Simon96_144 => self
                    .cipher_96_144
                    .ksa(vec.try_into().expect("wrong key size")),
                SimonVariant::Simon128_128 => self
                    .cipher_128_128
                    .ksa(vec.try_into().expect("wrong key size")),
                SimonVariant::Simon128_192 => self
                    .cipher_128_192
                    .ksa(vec.try_into().expect("wrong key size")),
                SimonVariant::Simon128_256 => self
                    .cipher_128_256
                    .ksa(vec.try_into().expect("wrong key size")),
            }
        } else {
            unreachable!("Simon key should be forced to valid hex digits by filtering")
        }
    }

    fn cipher(&self) -> &dyn Cipher {
        match self.selector {
            SimonVariant::Simon32_64 => &self.cipher_32_64,
            SimonVariant::Simon48_72 => &self.cipher_48_72,
            SimonVariant::Simon48_96 => &self.cipher_48_96,
            SimonVariant::Simon64_96 => &self.cipher_64_96,
            SimonVariant::Simon64_128 => &self.cipher_64_128,
            SimonVariant::Simon96_96 => &self.cipher_96_96,
            SimonVariant::Simon96_144 => &self.cipher_96_144,
            SimonVariant::Simon128_128 => &self.cipher_128_128,
            SimonVariant::Simon128_192 => &self.cipher_128_192,
            SimonVariant::Simon128_256 => &self.cipher_128_256,
        }
    }
}

impl CipherFrame for SimonFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/digital/block_ciphers/Simon",
        );
        ui.add_space(8.0);

        ui.randomize_reset_cipher(self);
        ui.add_space(16.0);

        for variant in SimonVariant::iter() {
            ui.selectable_value(&mut self.selector, variant, variant.to_string());
        }

        ui.add_space(16.0);

        if ui.byte_io_mode_cipher(&mut self.input_format, &mut self.output_format) {
            self.cipher_32_64.input_format = self.input_format;
            self.cipher_32_64.output_format = self.output_format;

            self.cipher_48_72.input_format = self.input_format;
            self.cipher_48_72.output_format = self.output_format;

            self.cipher_48_96.input_format = self.input_format;
            self.cipher_48_96.output_format = self.output_format;

            self.cipher_64_96.input_format = self.input_format;
            self.cipher_64_96.output_format = self.output_format;

            self.cipher_64_128.input_format = self.input_format;
            self.cipher_64_128.output_format = self.output_format;

            self.cipher_96_96.input_format = self.input_format;
            self.cipher_96_96.output_format = self.output_format;

            self.cipher_96_144.input_format = self.input_format;
            self.cipher_96_144.output_format = self.output_format;

            self.cipher_128_128.input_format = self.input_format;
            self.cipher_128_128.output_format = self.output_format;

            self.cipher_128_192.input_format = self.input_format;
            self.cipher_128_192.output_format = self.output_format;

            self.cipher_128_256.input_format = self.input_format;
            self.cipher_128_256.output_format = self.output_format;
        }

        ui.add_space(4.0);
        match self.selector {
            SimonVariant::Simon32_64 => block_cipher_mode_and_padding(
                ui,
                &mut self.cipher_32_64.mode,
                &mut self.cipher_32_64.padding,
            ),
            SimonVariant::Simon64_96 => block_cipher_mode_and_padding(
                ui,
                &mut self.cipher_64_96.mode,
                &mut self.cipher_64_96.padding,
            ),
            SimonVariant::Simon64_128 => block_cipher_mode_and_padding(
                ui,
                &mut self.cipher_64_128.mode,
                &mut self.cipher_64_128.padding,
            ),
            SimonVariant::Simon128_128 => block_cipher_mode_and_padding(
                ui,
                &mut self.cipher_128_128.mode,
                &mut self.cipher_128_128.padding,
            ),
            SimonVariant::Simon128_192 => block_cipher_mode_and_padding(
                ui,
                &mut self.cipher_128_192.mode,
                &mut self.cipher_128_192.padding,
            ),
            SimonVariant::Simon128_256 => block_cipher_mode_and_padding(
                ui,
                &mut self.cipher_128_256.mode,
                &mut self.cipher_128_256.padding,
            ),
            SimonVariant::Simon48_72 => block_cipher_mode_and_padding(
                ui,
                &mut self.cipher_48_72.mode,
                &mut self.cipher_48_72.padding,
            ),
            SimonVariant::Simon48_96 => block_cipher_mode_and_padding(
                ui,
                &mut self.cipher_48_96.mode,
                &mut self.cipher_48_96.padding,
            ),
            SimonVariant::Simon96_96 => block_cipher_mode_and_padding(
                ui,
                &mut self.cipher_96_96.mode,
                &mut self.cipher_96_96.padding,
            ),
            SimonVariant::Simon96_144 => block_cipher_mode_and_padding(
                ui,
                &mut self.cipher_96_144.mode,
                &mut self.cipher_96_144.padding,
            ),
        };

        ui.add_space(16.0);

        ui.subheading("Key");
        ui.label(format!(
            "{} takes a key of exactly {} bytes ({} hexadecimal digits). This key is used to create {} subkeys, one for each round of encryption. The subkeys are not updated until the button below is pressed.",
            self.selector,
            self.selector.key_size(),
            self.selector.key_size() * 2,
            self.selector.rounds(),
        ));
        if ui.text_edit_multiline(&mut self.key).changed() {
            self.key = self.key.chars().filter(|c| c.is_ascii_hexdigit()).collect();
        }
        if ui.button("Generate Subkeys").clicked() {
            if self.key.len() > (self.selector.key_size() as usize * 2) {
                self.key.truncate(self.selector.key_size() as usize * 2)
            }
            while self.key.len() < (self.selector.key_size() as usize * 2) {
                self.key.push('0')
            }
            self.run_ksa()
        }

        ui.add_space(16.0);

        match self.selector {
            SimonVariant::Simon32_64 => {
                block_cipher_iv_32(ui, &mut self.cipher_32_64.iv, self.cipher_32_64.mode)
            }
            SimonVariant::Simon64_96 => {
                block_cipher_iv_64(ui, &mut self.cipher_64_96.iv, self.cipher_64_96.mode)
            }
            SimonVariant::Simon64_128 => {
                block_cipher_iv_64(ui, &mut self.cipher_64_128.iv, self.cipher_64_128.mode)
            }
            SimonVariant::Simon128_128 => {
                block_cipher_iv_128(ui, &mut self.cipher_128_128.iv, self.cipher_128_128.mode)
            }
            SimonVariant::Simon128_192 => {
                block_cipher_iv_128(ui, &mut self.cipher_128_192.iv, self.cipher_128_192.mode)
            }
            SimonVariant::Simon128_256 => {
                block_cipher_iv_128(ui, &mut self.cipher_128_256.iv, self.cipher_128_256.mode)
            }
            SimonVariant::Simon48_72 => {
                block_cipher_iv_48(ui, &mut self.cipher_48_72.iv, self.cipher_48_72.mode)
            }
            SimonVariant::Simon48_96 => {
                block_cipher_iv_48(ui, &mut self.cipher_48_96.iv, self.cipher_48_96.mode)
            }
            SimonVariant::Simon96_96 => {
                block_cipher_iv_96(ui, &mut self.cipher_96_96.iv, self.cipher_96_96.mode)
            }
            SimonVariant::Simon96_144 => {
                block_cipher_iv_96(ui, &mut self.cipher_96_144.iv, self.cipher_96_144.mode)
            }
        }

        ui.add_space(16.0);
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.key = format!("{:08X}", rng.gen::<u64>());
        self.run_ksa();
    }

    fn reset(&mut self) {
        *self = Self::default()
    }

    fn encrypt_string(&self, text: &str) -> Result<String, ciphers::CipherError> {
        ciphers::Cipher::encrypt(self.cipher(), text)
    }

    fn decrypt_string(&self, text: &str) -> Result<String, ciphers::CipherError> {
        ciphers::Cipher::decrypt(self.cipher(), text)
    }
}
