// use super::CipherFrame;
// use crate::ui_elements::{block_cipher_iv_64, block_cipher_mode, block_cipher_padding, UiElements};
// use ciphers::{
//     digital::block_ciphers::{
//         block_cipher::{BCMode, BCPadding},
//         speck::{
//             speck128::{Speck128_128, Speck128_192, Speck128_256},
//             speck32::Speck32_64,
//             speck64::{Speck64_128, Speck64_96},
//             SpeckVariant,
//         },
//     },
//     Cipher,
// };
// use egui::Ui;
// use rand::{thread_rng, Rng};
// use utils::byte_formatting::ByteFormat;

// pub struct SpeckFrame {
//     variant: SpeckVariant,
//     ciphers: (
//         Speck32_64,
//         Speck64_96,
//         Speck64_128,
//         Speck128_128,
//         Speck128_192,
//         Speck128_256,
//     ),
//     input_format: ByteFormat,
//     output_format: ByteFormat,
//     mode: BCMode,
//     padding: BCPadding,
// }

// impl Default for SpeckFrame {
//     fn default() -> Self {
//         Self {
//             variant: SpeckVariant::Speck128_128,
//             ciphers: Default::default(),
//             input_format: ByteFormat::Hex,
//             output_format: ByteFormat::Hex,
//             mode: Default::default(),
//             padding: Default::default(),
//         }
//     }
// }

// impl SpeckFrame {
//     pub fn selected(&mut self) -> &mut dyn Cipher {
//         match self.variant {
//             SpeckVariant::Speck32_64 => &mut self.ciphers.0,
//             SpeckVariant::Speck64_96 => &mut self.ciphers.1,
//             SpeckVariant::Speck64_128 => &mut self.ciphers.2,
//             SpeckVariant::Speck128_128 => &mut self.ciphers.3,
//             SpeckVariant::Speck128_192 => &mut self.ciphers.4,
//             SpeckVariant::Speck128_256 => &mut self.ciphers.5,
//         }
//     }
// }

// impl CipherFrame for SpeckFrame {
//     fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
//         ui.hyperlink_to(
//             "see the code",
//             "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/digital/block_ciphers/speck",
//         );
//         ui.add_space(8.0);

//         ui.randomize_reset(self);
//         ui.add_space(16.0);

//         ui.byte_io_mode_cipher(&mut self.input_format, &mut self.output_format);

//         ui.add_space(16.0);

//         block_cipher_mode(ui, &mut self.mode);
//         ui.add_space(4.0);
//         block_cipher_padding(ui, &mut self.padding);
//         ui.add_space(8.0);

//         ui.horizontal(|ui| {
//             ui.subheading("Key");
//             ui.random_bytes_button(&mut self.cipher.subkeys);
//         });
//         ui.label(format!("{} uses a 64-bit key, here controlled as four 16-bit keys.",self.variant));
//         for i in 0..4 {
//             ui.u32_hex_edit(&mut self.cipher.subkeys[i]);
//         }

//         ui.add_space(8.0);

//         block_cipher_iv_64(ui, &mut self.cipher.iv, self.cipher.mode);

//         ui.add_space(16.0);
//     }

//     fn cipher(&self) -> &dyn Cipher {
//         match self.variant {
//             SpeckVariant::Speck32_64 => &self.ciphers.0,
//             SpeckVariant::Speck64_96 => &self.ciphers.1,
//             SpeckVariant::Speck64_128 => &self.ciphers.2,
//             SpeckVariant::Speck128_128 => &self.ciphers.3,
//             SpeckVariant::Speck128_192 => &self.ciphers.4,
//             SpeckVariant::Speck128_256 => &self.ciphers.5,
//         }
//     }

//     fn randomize(&mut self) {
//         let mut rng = thread_rng();

//         let mut c = self.selected();
//         c.sub

//         self.cipher.subkeys[0] = rng.gen();
//         self.cipher.subkeys[1] = rng.gen();
//         self.cipher.subkeys[2] = rng.gen();
//         self.cipher.subkeys[3] = rng.gen();

//         if self.cipher.mode.iv_needed() {
//             self.cipher.iv = rng.gen();
//         }
//     }

//     fn reset(&mut self) {
//         *self = Self::default()
//     }
// }
