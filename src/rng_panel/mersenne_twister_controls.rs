use super::ClassicRngFrame;
use crate::ui_elements::{generate_randoms_box, UiElements};
use egui::Ui;
use rand::{thread_rng, Rng};
use rngs::mersenne_twister::{mt19937_32::Mt19937_32, mt19937_64::Mt19937_64};
use utils::byte_formatting::ByteFormat;

pub struct MTFrame {
    rng_32: Mt19937_32,
    rng_64: Mt19937_64,
    key_32: String,
    key_64: String,
    randoms: String,
    n_random: usize,
    mt64: bool,
}

impl Default for MTFrame {
    fn default() -> Self {
        let mut rng_32 = Mt19937_32::default();
        rng_32.ksa_default();
        let mut rng_64 = Mt19937_64::default();
        rng_64.ksa_default();
        Self {
            rng_32,
            rng_64,
            key_32: String::from("00001571"),
            key_64: String::from("0000000000001571"),
            randoms: String::new(),
            n_random: 5,
            mt64: false,
        }
    }
}

impl MTFrame {
    fn twist(&mut self) {
        if self.mt64 {
            self.rng_64.twist()
        } else {
            self.rng_32.twist()
        }
    }

    fn filter_key_string(&mut self, ui: &mut Ui) {
        if self.mt64 {
            if ui.text_edit_multiline(&mut self.key_64).changed() {
                self.key_64 = self
                    .key_64
                    .chars()
                    .filter(|c| c.is_ascii_hexdigit())
                    .collect();
            }
        } else {
            if ui.text_edit_multiline(&mut self.key_32).changed() {
                self.key_32 = self
                    .key_32
                    .chars()
                    .filter(|c| c.is_ascii_hexdigit())
                    .collect();
            }
        }
    }

    fn run_ksa(&mut self) {
        if self.mt64 {
            self.run_ksa_64()
        } else {
            self.run_ksa_32()
        }
    }

    fn run_ksa_32(&mut self) {
        while self.key_32.len() % 8 != 0 {
            self.key_32.push('0')
        }

        let key_vec = ByteFormat::Hex.text_to_u32_be(&self.key_32);

        if let Ok(vec) = key_vec {
            self.rng_32 = Mt19937_32::from_array(&vec)
        } else {
            unreachable!("Mersenne Twister key should be forced to valid hex digits by filtering")
        }
    }

    fn run_ksa_64(&mut self) {
        while self.key_64.len() % 16 != 0 {
            self.key_64.push('0')
        }
        let key_vec = ByteFormat::Hex.text_to_u64_be(&self.key_64);
        if let Ok(vec) = key_vec {
            self.rng_64 = Mt19937_64::from_array(&vec);
        } else {
            unreachable!("Mersenne Twister key should be forced to valid hex digits by filtering")
        }
    }
}

impl ClassicRngFrame for MTFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the MT32 code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/rngs/src/mersenne_twister/mt19937_32.rs",
        );
        ui.hyperlink_to(
            "see the MT64 code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/rngs/src/mersenne_twister/mt19937_64.rs",
        );
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.mt64, false, "MT32");
            ui.selectable_value(&mut self.mt64, true, "MT64");
        });
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.subheading("Key");
            if ui
                .button("🎲")
                .on_hover_text(if self.mt64 {
                    "initialize from a random 64-bit integer"
                } else {
                    "initialize from a random 32-bit integer"
                })
                .clicked()
            {
                self.randomize();
            }
        });
        ui.label("Key should be provided as a string of hexadecimal digits representing any number of bytes.");
        self.filter_key_string(ui);
        ui.add_space(8.0);

        ui.subheading("Key Scheduling Algorithm");
        ui.label(
            "The state of the internal Mersenne Twister array is built from the key using the KSA.",
        );
        if ui.button("Run KSA").clicked() {
            self.run_ksa()
        }
        ui.add_space(16.0);

        ui.subheading("Internal State");
        if ui.button("Twist").clicked() {
            self.twist()
        }

        ui.label(format!(
            "Index: {}",
            if self.mt64 {
                self.rng_64.index
            } else {
                self.rng_32.index
            }
        ));
        if self.mt64 {
            ui.collapsing("Array of 312 64-bit words", |ui| {
                egui::Grid::new("mt_array_64")
                    .num_columns(26)
                    .striped(true)
                    .show(ui, |ui| {
                        for (n, b) in self.rng_64.arr.iter_mut().enumerate() {
                            if n % 24 == 0 && n != 0 {
                                ui.end_row()
                            }
                            ui.u64_hex_edit(b);
                        }
                    });
            });
        } else {
            ui.collapsing("Array of 624 32-bit words", |ui| {
                egui::Grid::new("mt_array_32")
                    .num_columns(26)
                    .striped(true)
                    .show(ui, |ui| {
                        for (n, b) in self.rng_32.arr.iter_mut().enumerate() {
                            if n % 24 == 0 && n != 0 {
                                ui.end_row()
                            }
                            ui.u32_hex_edit(b);
                        }
                    });
            });
        }

        ui.add_space(16.0);
        if self.mt64 {
            generate_randoms_box(ui, &mut self.rng_64, &mut self.n_random, &mut self.randoms);
        } else {
            generate_randoms_box(ui, &mut self.rng_32, &mut self.n_random, &mut self.randoms);
        }

        ui.add_space(16.0);
    }

    fn rng(&mut self) -> &mut dyn rngs::ClassicRng {
        match self.mt64 {
            true => &mut self.rng_64,
            false => &mut self.rng_32,
        }
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        if self.mt64 {
            self.key_64 = format!("{:016X}", rng.gen::<u64>());
            self.run_ksa_64();
        } else {
            self.key_32 = format!("{:08X}", rng.gen::<u32>());
            self.run_ksa_32();
        }
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
