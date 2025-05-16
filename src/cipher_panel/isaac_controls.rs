use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::digital::stream_ciphers::isaac::Isaac;
use egui::Ui;
use rand::{thread_rng, Rng};
use std::num::Wrapping;

pub struct IsaacFrame {
    cipher: Isaac,
    array: [u32; 256],
    extra_pass: bool,
}

impl Default for IsaacFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            array: [0; 256],
            extra_pass: false,
        }
    }
}

impl CipherFrame for IsaacFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/digital/stream_ciphers/isaac.rs",
        );
        ui.add_space(8.0);

        ui.randomize_reset_cipher(self);
        ui.add_space(16.0);

        ui.byte_io_mode_cipher(
            &mut self.cipher.input_format,
            &mut self.cipher.output_format,
        );
        ui.add_space(8.0);

        ui.checkbox(&mut self.extra_pass, "Use Extra Pass");
        ui.add_space(8.0);

        if ui.button("Initialize").clicked() {
            self.cipher.init(self.extra_pass);
        }
        ui.add_space(8.0);

        ui.label(format!("a: {:08x}", self.cipher.a));
        ui.label(format!("b: {:08x}", self.cipher.b));
        ui.label(format!("c: {:08x}", self.cipher.c));
        ui.add_space(8.0);

        ui.collapsing("Array of Words", |ui| {
            egui::Grid::new("isaac_array")
                .num_columns(32)
                .striped(true)
                .show(ui, |ui| {
                    for (n, b) in self.array.iter_mut().enumerate() {
                        if n % 8 == 0 && n != 0 {
                            ui.end_row()
                        }
                        if ui.u32_hex_edit(b).lost_focus() {
                            self.cipher.array[n] = Wrapping(*b)
                        }
                    }
                });
        });
        ui.add_space(16.0);
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        rng.fill(&mut self.array);
        self.cipher.array = self.array.map(|n| Wrapping(n));
        self.cipher.init(self.extra_pass);
    }

    crate::simple_cipher! {}
}
