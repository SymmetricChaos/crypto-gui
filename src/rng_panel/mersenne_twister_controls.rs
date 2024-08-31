use super::ClassicRngFrame;
use crate::ui_elements::{generate_random_u32s_box, UiElements};
use egui::{FontId, RichText, Ui};
use rand::{thread_rng, Rng};
use rngs::{mt19937_32::Mt19937_32, mt19937_64::Mt19937_64};
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
            self.rng_32.ksa_from_array(&vec)
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
            self.rng_64.ksa_from_array(&vec)
        } else {
            unreachable!("Mersenne Twister key should be forced to valid hex digits by filtering")
        }
    }
}

impl ClassicRngFrame for MTFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.add_space(16.0);

        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.mt64, false, "MT32");
            ui.selectable_value(&mut self.mt64, true, "MT64");
        });

        ui.horizontal(|ui| {
            ui.subheading("Key");
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                self.randomize();
            }
        });

        ui.label("Key should be provided as a string of hexadecimal digits representing any number of bytes.");
        self.filter_key_string(ui);

        ui.subheading("Key Scheduling Algorithm");
        ui.label("The state of the internal Mersenne Twister array is build from the key using a Key Scheduling Algorithm.");
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
                        for (n, b) in self.rng_64.arr.into_iter().enumerate() {
                            if n % 24 == 0 && n != 0 {
                                ui.end_row()
                            }
                            if n == self.rng_32.index as usize {
                                ui.label(
                                    RichText::from(format!("{:016X}", b))
                                        .font(FontId::monospace(15.0))
                                        .strong(),
                                );
                            } else {
                                ui.label(
                                    RichText::from(format!("{:016X}", b))
                                        .font(FontId::monospace(15.0)),
                                );
                            }
                        }
                    });
            });
        } else {
            ui.collapsing("Array of 624 32-bit words", |ui| {
                egui::Grid::new("mt_array_32")
                    .num_columns(26)
                    .striped(true)
                    .show(ui, |ui| {
                        for (n, b) in self.rng_32.arr.into_iter().enumerate() {
                            if n % 24 == 0 && n != 0 {
                                ui.end_row()
                            }
                            if n == self.rng_32.index as usize {
                                ui.label(
                                    RichText::from(format!("{:08X}", b))
                                        .font(FontId::monospace(15.0))
                                        .strong(),
                                );
                            } else {
                                ui.label(
                                    RichText::from(format!("{:08X}", b))
                                        .font(FontId::monospace(15.0)),
                                );
                            }
                        }
                    });
            });
        }

        //ui.collapsing("explain", |ui| ui.label(""));

        ui.add_space(16.0);
        generate_random_u32s_box(ui, &mut self.rng_32, &mut self.n_random, &mut self.randoms);
        ui.add_space(16.0);
    }

    fn rng(&self) -> &dyn rngs::ClassicRng {
        match self.mt64 {
            true => &self.rng_64,
            false => &self.rng_32,
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
