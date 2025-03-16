use super::ClassicRngFrame;
use crate::ui_elements::{generate_randoms_box, UiElements};
use egui::{FontId, RichText};
use rand::{thread_rng, Rng};
use rngs::{ia::Ia, ibaa::Ibaa, isaac::Isaac, ClassicRng};
use std::num::ParseIntError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IsaacSelector {
    Isaac,
    Ia,
    Ibaa,
}

pub struct IsaacFrame {
    isaac: Isaac,
    ia: Ia,
    ibaa: Ibaa,
    extra_pass: bool,
    selector: IsaacSelector,
    key: String,
    randoms: String,
    n_random: usize,
}

impl Default for IsaacFrame {
    fn default() -> Self {
        Self {
            isaac: Default::default(),
            ia: Ia::default(),
            ibaa: Ibaa::default(),
            extra_pass: true,
            selector: IsaacSelector::Isaac,
            key: String::from("DEADBEEF42"),
            randoms: String::new(),
            n_random: 5,
        }
    }
}

impl IsaacFrame {
    fn run_ksa(&mut self) {
        let key_vec: Result<Vec<u8>, ParseIntError> = (0..self.key.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&self.key[i..i + 2], 16))
            .collect();
        match self.selector {
            IsaacSelector::Isaac => {
                if let Ok(vec) = key_vec {
                    self.isaac.seed(&vec, self.extra_pass);
                } else {
                    unreachable!("ISAAC key should be forced to valid hex digits by filtering")
                }
            }
            IsaacSelector::Ia => {
                if let Ok(vec) = key_vec {
                    self.ia.seed(&vec, self.extra_pass);
                } else {
                    unreachable!("IA key should be forced to valid hex digits by filtering")
                }
            }
            IsaacSelector::Ibaa => {
                if let Ok(vec) = key_vec {
                    self.ibaa.seed(&vec, self.extra_pass);
                } else {
                    unreachable!("IBAA key should be forced to valid hex digits by filtering")
                }
            }
        };
    }
}

impl ClassicRngFrame for IsaacFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the IA code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/rngs/src/ia.rs",
        );
        ui.hyperlink_to(
            "see the IBAA code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/rngs/src/ibaa.rs",
        );
        ui.hyperlink_to(
            "see the ISAAC code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/rngs/src/isaac.rs",
        );
        ui.add_space(8.0);

        ui.selectable_value(&mut self.selector, IsaacSelector::Ia, "IA");
        ui.selectable_value(&mut self.selector, IsaacSelector::Ibaa, "IBAA");
        ui.selectable_value(&mut self.selector, IsaacSelector::Isaac, "ISAAC");
        ui.add_space(16.0);

        ui.subheading("Discussion");
        match self.selector {
            IsaacSelector::Isaac => ui.label("ISAAC is the final PRNG designed in the sequence. It uses Indirection, Shifting, Accumulation, Addition, and Counting. The design is faster than IBAA and should have no bad seeds or short cycles."),
            IsaacSelector::Ia => ui.label(
                "IA is the first PRNG designed in the sequence. It uses Indirection and Addition. There is a slight bias in the outputs.",
            ),
            IsaacSelector::Ibaa => ui.label("IBAA is the second PRNG designed in the sequence. It uses Indirection, Barrelshifting (Rotation), Accumulation, and Addition. The design eliminates the bias in IA."),
        };
        ui.add_space(16.0);

        ui.horizontal(|ui| {
            ui.subheading("Key");
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                self.randomize();
            }
        });
        ui.label("Key should be provided as a string of hexadecimal digits representing between 1 and 256 bytes.");
        if ui.text_edit_multiline(&mut self.key).changed() {
            self.key = self.key.chars().filter(|c| c.is_ascii_hexdigit()).collect();
        }
        if ui.button("Set Array from Key").clicked() {
            if self.key.len() % 2 == 1 {
                self.key.push('0')
            }
            self.run_ksa()
        }
        ui.add_space(16.0);

        match self.selector {
            IsaacSelector::Isaac => {
                ui.subheading("Internal State");
                ui.label(format!("Output Counter: {}", self.isaac.ctr));
                ui.add_space(8.0);
                ui.label("Auxiliary Variables");
                ui.label(format!("a: {:08x}", self.isaac.a));
                ui.label(format!("b: {:08x}", self.isaac.b));
                ui.label(format!("c: {:08x}", self.isaac.c));
                ui.add_space(8.0);
                ui.collapsing("Array of State Words", |ui| {
                    egui::Grid::new("isaac_array")
                        .num_columns(16)
                        .striped(true)
                        .show(ui, |ui| {
                            for (n, b) in self.isaac.array.into_iter().enumerate() {
                                if n % 16 == 0 && n != 0 {
                                    ui.end_row()
                                }

                                ui.label(
                                    RichText::from(format!("{:08x}", b))
                                        .font(FontId::monospace(15.0)),
                                );
                            }
                        });
                });
                ui.add_space(8.0);
                ui.collapsing("Array of Output Words", |ui| {
                    egui::Grid::new("isaac_output")
                        .num_columns(16)
                        .striped(true)
                        .show(ui, |ui| {
                            for (n, b) in self.isaac.array.into_iter().enumerate() {
                                if n % 16 == 0 && n != 0 {
                                    ui.end_row()
                                }
                                if n == self.isaac.ctr {
                                    ui.label(
                                        RichText::from(format!("{:08x}", b))
                                            .strong()
                                            .font(FontId::monospace(15.0)),
                                    );
                                } else {
                                    ui.label(
                                        RichText::from(format!("{:08x}", b))
                                            .font(FontId::monospace(15.0)),
                                    );
                                }
                            }
                        });
                });
            }
            IsaacSelector::Ia => {
                ui.subheading("Internal State");
                ui.label(format!("Output Counter: {}", self.ia.ctr));
                ui.add_space(8.0);
                ui.label("Auxiliary Variables");
                ui.label(format!("b: {:08x}", self.ia.b));
                ui.add_space(8.0);
                ui.collapsing("Array of State Words", |ui| {
                    egui::Grid::new("ia_array")
                        .num_columns(16)
                        .striped(true)
                        .show(ui, |ui| {
                            for (n, b) in self.ia.array.into_iter().enumerate() {
                                if n % 16 == 0 && n != 0 {
                                    ui.end_row()
                                }

                                ui.label(
                                    RichText::from(format!("{:08x}", b))
                                        .font(FontId::monospace(15.0)),
                                );
                            }
                        });
                });
                ui.add_space(8.0);
                ui.collapsing("Array of Output Words", |ui| {
                    egui::Grid::new("ia_output")
                        .num_columns(16)
                        .striped(true)
                        .show(ui, |ui| {
                            for (n, b) in self.isaac.array.into_iter().enumerate() {
                                if n % 16 == 0 && n != 0 {
                                    ui.end_row()
                                }
                                if n == self.isaac.ctr {
                                    ui.label(
                                        RichText::from(format!("{:08x}", b))
                                            .strong()
                                            .font(FontId::monospace(15.0)),
                                    );
                                } else {
                                    ui.label(
                                        RichText::from(format!("{:08x}", b))
                                            .font(FontId::monospace(15.0)),
                                    );
                                }
                            }
                        });
                });
            }
            IsaacSelector::Ibaa => {
                ui.subheading("Internal State");
                ui.label(format!("Output Counter: {}", self.ibaa.ctr));
                ui.add_space(8.0);
                ui.label("Auxiliary Variables");
                ui.label(format!("a: {:08x}", self.ibaa.a));
                ui.label(format!("b: {:08x}", self.ibaa.b));
                ui.add_space(8.0);
                ui.collapsing("Array of State Words", |ui| {
                    egui::Grid::new("ibaa_array")
                        .num_columns(16)
                        .striped(true)
                        .show(ui, |ui| {
                            for (n, b) in self.ibaa.array.into_iter().enumerate() {
                                if n % 16 == 0 && n != 0 {
                                    ui.end_row()
                                }

                                ui.label(
                                    RichText::from(format!("{:08x}", b))
                                        .font(FontId::monospace(15.0)),
                                );
                            }
                        });
                });
                ui.add_space(8.0);
                ui.collapsing("Array of Output Words", |ui| {
                    egui::Grid::new("ibaa_output")
                        .num_columns(16)
                        .striped(true)
                        .show(ui, |ui| {
                            for (n, b) in self.isaac.array.into_iter().enumerate() {
                                if n % 16 == 0 && n != 0 {
                                    ui.end_row()
                                }
                                if n == self.isaac.ctr {
                                    ui.label(
                                        RichText::from(format!("{:08x}", b))
                                            .strong()
                                            .font(FontId::monospace(15.0)),
                                    );
                                } else {
                                    ui.label(
                                        RichText::from(format!("{:08x}", b))
                                            .font(FontId::monospace(15.0)),
                                    );
                                }
                            }
                        });
                });
            }
        };

        ui.add_space(16.0);
        if ui.button("step").clicked() {
            self.isaac.next_u32();
        }

        ui.add_space(16.0);
        generate_randoms_box(ui, &mut self.isaac, &mut self.n_random, &mut self.randoms);
        ui.add_space(16.0);
    }

    fn rng(&self) -> &dyn rngs::ClassicRng {
        match self.selector {
            IsaacSelector::Isaac => &self.isaac,
            IsaacSelector::Ia => &self.ia,
            IsaacSelector::Ibaa => &self.ibaa,
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
