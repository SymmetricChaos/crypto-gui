use super::CipherFrame;
use crate::ui_elements::{
    block_cipher_iv_128, block_cipher_iv_32, block_cipher_iv_64, block_cipher_mode_and_padding,
    UiElements,
};
use ciphers::{
    digital::block_ciphers::rc5::{rc5_16::Rc5_16, rc5_32::Rc5_32, rc5_64::Rc5_64},
    Cipher,
};
use egui::{FontId, RichText, Ui};
use rand::{thread_rng, Rng};
use utils::byte_formatting::ByteFormat;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SizeSelector {
    R16,
    R32,
    R64,
}

impl Default for SizeSelector {
    fn default() -> Self {
        Self::R64
    }
}

#[derive(Default)]
pub struct Rc5Frame {
    cipher_16: Rc5_16,
    cipher_32: Rc5_32,
    cipher_64: Rc5_64,
    selector: SizeSelector,
    key: String,
}

impl Rc5Frame {
    fn run_ksa(&mut self) {
        let key_vec = ByteFormat::Hex.text_to_bytes(&self.key);

        if let Ok(vec) = key_vec {
            match self.selector {
                SizeSelector::R16 => self.cipher_16.ksa(&vec),
                SizeSelector::R32 => self.cipher_32.ksa(&vec),
                SizeSelector::R64 => self.cipher_64.ksa(&vec),
            }
        } else {
            unreachable!("RC5 key should be forced to valid hex digits by filtering")
        }
    }
}

impl CipherFrame for Rc5Frame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/digital/block_ciphers/rc5",
        );
        ui.add_space(8.0);

        ui.randomize_reset(self);
        ui.add_space(16.0);

        ui.selectable_value(&mut self.selector, SizeSelector::R16, "16-bit");
        ui.selectable_value(&mut self.selector, SizeSelector::R32, "32-bit");
        ui.selectable_value(&mut self.selector, SizeSelector::R64, "64-bit");

        ui.add_space(16.0);

        match self.selector {
            SizeSelector::R16 => ui.byte_io_mode_cipher(
                &mut self.cipher_16.input_format,
                &mut self.cipher_16.output_format,
            ),
            SizeSelector::R32 => ui.byte_io_mode_cipher(
                &mut self.cipher_32.input_format,
                &mut self.cipher_32.output_format,
            ),
            SizeSelector::R64 => ui.byte_io_mode_cipher(
                &mut self.cipher_64.input_format,
                &mut self.cipher_64.output_format,
            ),
        };
        ui.add_space(4.0);

        match self.selector {
            SizeSelector::R16 => block_cipher_mode_and_padding(
                ui,
                &mut self.cipher_16.mode,
                &mut self.cipher_16.padding,
            ),
            SizeSelector::R32 => block_cipher_mode_and_padding(
                ui,
                &mut self.cipher_32.mode,
                &mut self.cipher_32.padding,
            ),
            SizeSelector::R64 => block_cipher_mode_and_padding(
                ui,
                &mut self.cipher_64.mode,
                &mut self.cipher_64.padding,
            ),
        };

        ui.add_space(16.0);

        ui.subheading("Key");
        ui.label("Key should be provided as a string of hexadecimal digits representing between 1 and 255 bytes.");
        if ui.text_edit_multiline(&mut self.key).changed() {
            self.key = self.key.chars().filter(|c| c.is_ascii_hexdigit()).collect();
        }
        if ui.button("Set State from Key").clicked() {
            if self.key.len() > 510 {
                self.key.truncate(510)
            }
            if self.key.len() % 2 == 1 {
                self.key.push('0')
            }
            self.run_ksa()
        }
        ui.add_space(16.0);

        match self.selector {
            SizeSelector::R16 => {
                block_cipher_iv_32(ui, &mut self.cipher_16.iv, self.cipher_16.mode);
            }
            SizeSelector::R32 => {
                block_cipher_iv_64(ui, &mut self.cipher_32.iv, self.cipher_32.mode);
            }
            SizeSelector::R64 => {
                block_cipher_iv_128(ui, &mut self.cipher_64.iv, self.cipher_64.mode);
            }
        }

        ui.subheading("Internal State");

        match self.selector {
            SizeSelector::R16 => ui.collapsing("Array of 16-bit Words", |ui| {
                egui::Grid::new("rc4_16_array")
                    .num_columns(16)
                    .striped(true)
                    .show(ui, |ui| {
                        for (n, b) in self.cipher_16.state.iter().enumerate() {
                            if n % 16 == 0 && n != 0 {
                                ui.end_row()
                            }
                            ui.label(
                                RichText::from(format!("{:04X}", b)).font(FontId::monospace(15.0)),
                            );
                        }
                    });
            }),
            SizeSelector::R32 => ui.collapsing("Array of 32-bit Words", |ui| {
                egui::Grid::new("rc4_32_array")
                    .num_columns(16)
                    .striped(true)
                    .show(ui, |ui| {
                        for (n, b) in self.cipher_32.state.iter().enumerate() {
                            if n % 16 == 0 && n != 0 {
                                ui.end_row()
                            }
                            ui.label(
                                RichText::from(format!("{:08X}", b)).font(FontId::monospace(15.0)),
                            );
                        }
                    });
            }),
            SizeSelector::R64 => ui.collapsing("Array of 64-bit Words", |ui| {
                egui::Grid::new("rc4_64_array")
                    .num_columns(16)
                    .striped(true)
                    .show(ui, |ui| {
                        for (n, b) in self.cipher_64.state.iter().enumerate() {
                            if n % 16 == 0 && n != 0 {
                                ui.end_row()
                            }
                            ui.label(
                                RichText::from(format!("{:016X}", b)).font(FontId::monospace(15.0)),
                            );
                        }
                    });
            }),
        };

        ui.add_space(16.0);
    }

    fn cipher(&self) -> &dyn Cipher {
        match self.selector {
            SizeSelector::R16 => &self.cipher_16,
            SizeSelector::R32 => &self.cipher_32,
            SizeSelector::R64 => &self.cipher_64,
        }
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.key = format!("{:08X}", rng.gen::<u64>());
        self.run_ksa();
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
