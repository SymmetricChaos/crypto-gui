use super::CipherFrame;
use crate::ui_elements::{block_cipher_iv_128, block_cipher_mode_and_padding, UiElements};
use ciphers::{digital::block_ciphers::serpent::Serpent, Cipher};
use rand::{thread_rng, Rng};

#[derive(Debug, PartialEq, Eq)]
enum SerpentKeySize {
    B128,
    B192,
    B256,
}

pub struct SerpentFrame {
    cipher: Serpent,
    key: [u32; 8],
    key_size: SerpentKeySize,
    key_range: usize,
}

impl Default for SerpentFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            key: [0u32; 8],
            key_size: SerpentKeySize::B128,
            key_range: 4,
        }
    }
}

impl CipherFrame for SerpentFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/digital/block_ciphers/serpent.rs",
        );
        ui.add_space(8.0);

        ui.randomize_reset(self);
        ui.add_space(16.0);

        ui.byte_io_mode_cipher(
            &mut self.cipher.input_format,
            &mut self.cipher.output_format,
        );

        ui.add_space(16.0);

        block_cipher_mode_and_padding(ui, &mut self.cipher.mode, &mut self.cipher.padding);
        ui.add_space(8.0);

        ui.subheading("Key");
        ui.label("Serpent accepts a key of 128 to 256 bits (16 to 32 bytes) but 128, 192, and 256 bit keys were standard. Short keys are padded to 256 bits before the key schedule is run and so have an equivalent 256-bit key.");

        if ui
            .selectable_value(&mut self.key_size, SerpentKeySize::B128, "Serpent128")
            .clicked()
        {
            self.key_range = 4;
        }
        if ui
            .selectable_value(&mut self.key_size, SerpentKeySize::B192, "Serpent192")
            .clicked()
        {
            self.key_range = 6;
        }
        if ui
            .selectable_value(&mut self.key_size, SerpentKeySize::B256, "Serpent256")
            .clicked()
        {
            self.key_range = 8;
        }

        for i in 0..self.key_range {
            if ui.u32_hex_edit(&mut self.key[i]).changed() {
                self.cipher.ksa_u32(&self.key[0..self.key_range]);
            }
        }

        // ui.add_space(8.0);

        // ui.collapsing("Expanded Key", |ui| {
        //     ui.subheading("Round Keys");
        //     ui.label(self.cipher.round_keys);
        // });

        ui.add_space(8.0);

        block_cipher_iv_128(ui, &mut self.cipher.iv, self.cipher.mode);

        ui.add_space(16.0);
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.key = [0; 8];
        for i in 0..self.key_range {
            self.key[i] = rng.gen();
        }
        self.cipher.ksa_u32(&self.key[0..self.key_range]);

        if self.cipher.mode.iv_needed() {
            self.cipher.iv = rng.gen();
        }
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
