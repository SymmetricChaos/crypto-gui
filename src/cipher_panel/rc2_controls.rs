use super::CipherFrame;
use crate::ui_elements::{block_cipher_mode_and_padding, UiElements};
use ciphers::{digital::block_ciphers::rc2::Rc2, Cipher};
use egui::{DragValue, FontId, RichText, Ui};
use rand::{thread_rng, Rng};
use utils::byte_formatting::ByteFormat;

#[derive(Default)]
pub struct Rc2Frame {
    cipher: Rc2,
    key: String,
}

impl Rc2Frame {
    fn run_ksa(&mut self) {
        let key_vec = ByteFormat::Hex.text_to_bytes(&self.key);
        if let Ok(vec) = key_vec {
            self.cipher.ksa(&vec)
        } else {
            unreachable!("RC2 key should be forced to valid hex digits by filtering")
        }
    }
}

impl CipherFrame for Rc2Frame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/digital/block_ciphers/rc2.rs",
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

        ui.subheading("Effective Key Length (T1)");
        ui.label("The T1 parameter limits the effective size of the key (in bits) regardless of how many bytes are actually given.");
        ui.add(DragValue::new(&mut self.cipher.effective_bits).range(1..=1024));
        ui.add_space(8.0);

        ui.subheading("Key");
        ui.label("Key should be provided as a string of hexadecimal digits representing between 1 and 128 bytes.");
        if ui.text_edit_multiline(&mut self.key).changed() {
            self.key = self
                .key
                .chars()
                .filter(|c| c.is_ascii_hexdigit())
                .take(256)
                .collect();
        };
        if ui.button("Generate Round Keys").clicked() {
            if self.key.len() % 2 == 1 {
                self.key.push('0')
            }
            self.run_ksa()
        }

        ui.add_space(16.0);

        ui.subheading("Round Keys");
        ui.collapsing("Array of 16-bit Words", |ui| {
            egui::Grid::new("rc2_16_array")
                .num_columns(8)
                .striped(true)
                .show(ui, |ui| {
                    for (n, b) in self.cipher.round_keys.iter().enumerate() {
                        if n % 8 == 0 && n != 0 {
                            ui.end_row()
                        }
                        ui.label(
                            RichText::from(format!("{:04X}", b)).font(FontId::monospace(15.0)),
                        );
                    }
                });
        });
        ui.add_space(16.0);
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
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
