use egui::DragValue;
use hashers::{
    errors::HasherError,
    keccak::{KeccackState, Keccak},
    traits::ClassicHasher,
};

use crate::ui_elements::UiElements;

use super::{byte_formatting_io, HasherFrame};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Sha3Variant {
    Sha3_224,
    Sha3_256,
    Sha3_384,
    Sha3_512,
    Shake128,
    Shake256,
}

pub struct Sha3Frame {
    hasher: Keccak,
    variant: Sha3Variant,
    shake_output_len: usize,
    example_state: KeccackState,
    example_round: usize,
}

impl Default for Sha3Frame {
    fn default() -> Self {
        Self {
            hasher: Default::default(),
            variant: Sha3Variant::Sha3_256,
            shake_output_len: 128,
            example_state: KeccackState::new(),
            example_round: 0,
        }
    }
}

impl Sha3Frame {
    fn set_hasher(&mut self) {
        match self.variant {
            Sha3Variant::Sha3_224 => self.hasher = Keccak::sha3_224(),
            Sha3Variant::Sha3_256 => self.hasher = Keccak::sha3_256(),
            Sha3Variant::Sha3_384 => self.hasher = Keccak::sha3_384(),
            Sha3Variant::Sha3_512 => self.hasher = Keccak::sha3_512(),
            Sha3Variant::Shake128 => self.hasher = Keccak::shake_128(self.shake_output_len),
            Sha3Variant::Shake256 => self.hasher = Keccak::shake_256(self.shake_output_len),
        }
    }
}

impl HasherFrame for Sha3Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.add_space(16.0);

        byte_formatting_io(
            ui,
            &mut self.hasher.input_format,
            &mut self.hasher.output_format,
        );

        ui.add_space(16.0);
        ui.subheading("SHA-3 Hash Algorithms");
        ui.horizontal(|ui| {
            if ui
                .selectable_value(&mut self.variant, Sha3Variant::Sha3_224, "SHA3-224")
                .changed()
                || ui
                    .selectable_value(&mut self.variant, Sha3Variant::Sha3_256, "SHA3-256")
                    .changed()
                || ui
                    .selectable_value(&mut self.variant, Sha3Variant::Sha3_384, "SHA3-384")
                    .changed()
                || ui
                    .selectable_value(&mut self.variant, Sha3Variant::Sha3_512, "SHA3-512")
                    .changed()
            {
                self.set_hasher()
            }
        });
        ui.add_space(8.0);
        ui.subheading("SHA-3 Extensible Output Functions");
        ui.horizontal(|ui| {
            if ui
                .selectable_value(&mut self.variant, Sha3Variant::Shake128, "SHAKE128")
                .changed()
                || ui
                    .selectable_value(&mut self.variant, Sha3Variant::Shake256, "SHAKE256")
                    .changed()
            {
                self.set_hasher()
            }
        });

        ui.add_space(8.0);
        ui.subheading("SHAKE Output Length (in bytes)");
        ui.add_enabled(
            self.variant == Sha3Variant::Shake128 || self.variant == Sha3Variant::Shake256,
            DragValue::new(&mut self.shake_output_len).clamp_range(1..=512),
        );

        ui.add_space(16.0);
        ui.subheading("Discussion");
        match self.variant {
            Sha3Variant::Sha3_224 => ui.label("SHA3-224 keeps 448 bits of state reserved and absorbs 1152 bits at a time."),
            Sha3Variant::Sha3_256 => ui.label("SHA3-256 keeps 512 bits of state reserved and absorbs 1088 bits at a time."),
            Sha3Variant::Sha3_384 => ui.label("SHA3-384 keeps 768 bits of state reserved and absorbs 832 bits at a time."),
            Sha3Variant::Sha3_512 => ui.label("SHA3-512 keeps 1024 bits of state reserved and absorbs 576 bits at a time."),
            Sha3Variant::Shake128 => ui.label("SHAKE128 keeps 256 bits of state reserved and absorbs 1344 bits at a time. It can be set to return any number of bits."),
            Sha3Variant::Shake256 => ui.label("SHAKE256 keeps 512 bits of state reserved and absorbs 1088 bits at a time. It can be set to return any number of bits."),
        };

        ui.add_space(16.0);
        ui.collapsing("Interactive State", |ui| {
            for y in 0..5 {
                ui.horizontal(|ui| {
                    for x in 0..5 {
                        ui.add(
                            DragValue::new(&mut self.example_state[x][y])
                                .hexadecimal(16, false, false),
                        );
                    }
                });
            }
            ui.add_space(8.0);
            ui.subheading("Round Number (only alters the iota step)");
            ui.add(DragValue::new(&mut self.example_round).clamp_range(0..=23));
            ui.add_space(8.0);
            ui.subheading("Steps");
            ui.group(|ui| {
                if ui.button("θ theta").clicked() {
                    self.example_state.theta()
                }
                if ui.button("ρ rho").clicked() {
                    self.example_state.rho()
                }
                if ui.button("π pi").clicked() {
                    self.example_state.pi()
                }
                if ui.button("χ chi").clicked() {
                    self.example_state.chi()
                }
                if ui.button("ι iota").clicked() {
                    self.example_state.iota(self.example_round)
                }
            });
            ui.add_space(8.0);
            if ui.button("Single Round").clicked() {
                self.example_state.round(self.example_round);
                self.example_round += 1;
            }
            ui.add_space(8.0);
            if ui.button("Full Permutation (24 Rounds)").clicked() {
                self.example_state.keccak_f()
            }
        });

        ui.add_space(16.0);
    }

    fn hash_bytes_from_string(&self, text: &str) -> Result<String, HasherError> {
        self.hasher.hash_bytes_from_string(text)
    }
}
