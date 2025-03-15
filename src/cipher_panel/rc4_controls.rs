use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::{digital::stream_ciphers::rc4::Rc4, Cipher};
use egui::{DragValue, FontId, RichText, Ui};
use rand::{thread_rng, Rng};
use utils::byte_formatting::ByteFormat;

#[derive(Default)]
pub struct Rc4Frame {
    cipher: Rc4,
    key: String,
}

impl Rc4Frame {
    fn run_ksa(&mut self) {
        let key_vec = ByteFormat::Hex.text_to_bytes(&self.key);
        if let Ok(vec) = key_vec {
            self.cipher.ksa(&vec)
        } else {
            unreachable!("RC4 key should be forced to valid hex digits by filtering")
        }
    }
}

impl CipherFrame for Rc4Frame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/digital/stream_ciphers/rc4.rs",
        );
        ui.add_space(8.0);

        ui.randomize_reset_cipher(self);
        ui.add_space(16.0);

        ui.byte_io_mode_cipher(
            &mut self.cipher.input_format,
            &mut self.cipher.output_format,
        );

        ui.add_space(16.0);

        ui.subheading("Key");
        ui.label("Key should be provided as a string of hexadecimal digits representing between 1 and 256 bytes.");
        if ui.text_edit_multiline(&mut self.key).changed() {
            self.key = self
                .key
                .chars()
                .filter(|c| c.is_ascii_hexdigit())
                .take(512)
                .collect();
        }
        if ui.button("Set Byte Array from Key").clicked() {
            if self.key.len() % 2 == 1 {
                self.key.push('0')
            }
            self.run_ksa()
        }
        ui.add_space(16.0);
        ui.subheading("Drop Bytes");
        ui.label("The early bytes of RC4 can leak information about the key so some implementations drop the first few hundred bytes.");
        ui.add(DragValue::new(&mut self.cipher.drop).range(0..=1024));
        ui.add_space(16.0);

        ui.subheading("Internal State");
        ui.group(|ui| {
            ui.label("Pointers into the array. These always start at zero.");
            ui.horizontal(|ui| {
                ui.label("i");
                ui.add(DragValue::new(&mut self.cipher.i).range(0..=255));
            });
            ui.horizontal(|ui| {
                ui.label("j");
                ui.add(DragValue::new(&mut self.cipher.j).range(0..=255));
            });
        });
        ui.collapsing("Array of Bytes", |ui| {
            egui::Grid::new("rc4_array")
                .num_columns(16)
                .striped(true)
                .show(ui, |ui| {
                    for (n, b) in self.cipher.arr.into_iter().enumerate() {
                        if n % 16 == 0 && n != 0 {
                            ui.end_row()
                        }
                        if n == self.cipher.i as usize || n == self.cipher.j as usize {
                            ui.label(
                                RichText::from(format!("{:02X}", b))
                                    .font(FontId::monospace(15.0))
                                    .strong(),
                            );
                        } else {
                            ui.label(
                                RichText::from(format!("{:02X}", b)).font(FontId::monospace(15.0)),
                            );
                        }
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
        self.cipher.i = 0;
        self.cipher.j = 0;
        self.run_ksa();
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
