use egui::{DragValue, RichText};
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
}

impl Default for Sha3Frame {
    fn default() -> Self {
        Self {
            hasher: Default::default(),
            variant: Sha3Variant::Sha3_256,
            shake_output_len: 128,
            example_state: KeccackState::new(),
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
        ui.subheading("SHAKE Output Length");
        ui.add_enabled(
            self.variant == Sha3Variant::Shake128 || self.variant == Sha3Variant::Shake256,
            DragValue::new(&mut self.shake_output_len).clamp_range(1..=512),
        );

        ui.add_space(16.0);
        ui.subheading("Discussion");
        match self.variant {
            Sha3Variant::Sha3_224 => ui.label("SHA3-224 "),
            Sha3Variant::Sha3_256 => ui.label("SHA3-256 "),
            Sha3Variant::Sha3_384 => ui.label("SHA3-384 "),
            Sha3Variant::Sha3_512 => ui.label("SHA3-512 "),
            Sha3Variant::Shake128 => ui.label("SHAKE128 "),
            Sha3Variant::Shake256 => ui.label("SHAKE256 "),
        };

        ui.add_space(16.0);
        ui.collapsing("Interactive State", |ui| {
            for y in 0..5 {
                ui.horizontal(|ui| {
                    for x in 0..5 {
                        ui.add(DragValue::new(&mut self.example_state[x][y]));
                    }
                });
            }

            if ui.button("Theta").clicked() {
                self.example_state.theta()
            }
            if ui.button("Rho").clicked() {
                self.example_state.rho()
            }
            if ui.button("Pi").clicked() {
                self.example_state.pi()
            }
            if ui.button("Chi").clicked() {
                self.example_state.chi()
            }
            if ui.button("Iota").clicked() {
                self.example_state.chi()
            }
        });

        ui.add_space(16.0);
    }

    fn hash_bytes_from_string(&self, text: &str) -> Result<String, HasherError> {
        self.hasher.hash_bytes_from_string(text)
    }
}
