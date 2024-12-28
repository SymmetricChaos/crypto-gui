use egui::DragValue;
use hashers::{
    errors::HasherError,
    sha::{Keccack, KeccackState},
    traits::StatefulHasher,
};
use utils::byte_formatting::ByteFormat;

use crate::ui_elements::{control_string, UiElements};

use super::HasherFrame;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Variant {
    Sha3_224,
    Sha3_256,
    Sha3_384,
    Sha3_512,
    Shake128,
    Shake256,
    CShake128,
    CShake256,
    // Kmac_128,
    // Kmac_256,
    // TupleHash_128,
    // TupleHash_256,
}

impl Variant {
    fn adjustable_length(&self) -> bool {
        match self {
            Variant::Sha3_224 => false,
            Variant::Sha3_256 => false,
            Variant::Sha3_384 => false,
            Variant::Sha3_512 => false,
            Variant::Shake128 => true,
            Variant::Shake256 => true,
            Variant::CShake128 => true,
            Variant::CShake256 => true,
        }
    }

    fn cshake(&self) -> bool {
        match self {
            Variant::Sha3_224 => false,
            Variant::Sha3_256 => false,
            Variant::Sha3_384 => false,
            Variant::Sha3_512 => false,
            Variant::Shake128 => false,
            Variant::Shake256 => false,
            Variant::CShake128 => true,
            Variant::CShake256 => true,
        }
    }
}

pub struct Sha3Frame {
    input_format: ByteFormat,
    output_format: ByteFormat,
    variant: Variant,
    shake_hash_len: u64,
    example_state: KeccackState,
    example_round: usize,
    function_name: String,
    customization: String,
}

impl Default for Sha3Frame {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            variant: Variant::Sha3_256,
            shake_hash_len: 128,
            example_state: KeccackState::new(),
            example_round: 0,
            function_name: String::new(),
            customization: String::new(),
        }
    }
}

impl HasherFrame for Sha3Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/sha/sha3.rs",
        );

        ui.byte_io_mode_hasher(&mut self.input_format, &mut self.output_format);

        ui.add_space(16.0);
        ui.subheading("SHA-3 Hash Algorithms");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.variant, Variant::Sha3_224, "SHA3-224");
            ui.selectable_value(&mut self.variant, Variant::Sha3_256, "SHA3-256");
            ui.selectable_value(&mut self.variant, Variant::Sha3_384, "SHA3-384");
            ui.selectable_value(&mut self.variant, Variant::Sha3_512, "SHA3-512");
        });
        ui.add_space(8.0);
        ui.subheading("SHA-3 Extensible Output Functions");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.variant, Variant::Shake128, "SHAKE128");
            ui.selectable_value(&mut self.variant, Variant::Shake256, "SHAKE256");
        });
        ui.add_space(8.0);
        ui.subheading("NIST Special Functions");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.variant, Variant::CShake128, "cSHAKE128");
            ui.selectable_value(&mut self.variant, Variant::CShake256, "cSHAKE256");
        });

        ui.add_space(16.0);
        ui.subheading("Discussion");
        match self.variant {
            Variant::Sha3_224 => ui.label("SHA3-224 keeps 448 bits of state reserved and absorbs 1152 bits at a time. It returns a 224 bit hash."),
            Variant::Sha3_256 => ui.label("SHA3-256 keeps 512 bits of state reserved and absorbs 1088 bits at a time. It returns a 256 bit hash."),
            Variant::Sha3_384 => ui.label("SHA3-384 keeps 768 bits of state reserved and absorbs 832 bits at a time. It returns a 384 bit hash."),
            Variant::Sha3_512 => ui.label("SHA3-512 keeps 1024 bits of state reserved and absorbs 576 bits at a time. It returns a 512 bit hash."),
            Variant::Shake128 => ui.label("SHAKE128 keeps 256 bits of state reserved and absorbs 1344 bits at a time. It can be set to return any number of bits."),
            Variant::Shake256 => ui.label("SHAKE256 keeps 512 bits of state reserved and absorbs 1088 bits at a time. It can be set to return any number of bits."),
            Variant::CShake128 => ui.label("cSHAKE128 is similar to SHAKE128 but allows both a function name and customization string."),
            Variant::CShake256 => ui.label("cSHAKE256 is similar to SHAKE256 but allows both a function name and customization string."),
        };

        ui.add_space(8.0);
        ui.subheading("Output Length (in bytes)");
        ui.add_enabled(
            self.variant.adjustable_length(),
            DragValue::new(&mut self.shake_hash_len).range(1..=1024),
        );

        ui.add_space(12.0);
        ui.subheading("cSHAKE Function Name");
        control_string(ui, &mut self.function_name, self.variant.cshake());
        ui.add_space(4.0);
        ui.subheading("cSHAKE Customization String");
        control_string(ui, &mut self.customization, self.variant.cshake());

        ui.add_space(16.0);
        ui.collapsing("Interactive State", |ui| {
            if ui.button("Reset").clicked() {
                self.example_state = KeccackState::new();
            };
            for y in 0..5 {
                ui.horizontal(|ui| {
                    for x in 0..5 {
                        ui.u64_hex_edit(&mut self.example_state[x][y]);
                    }
                });
            }
            ui.add_space(8.0);
            ui.subheading("Round Number (only alters the iota step)");
            ui.add(DragValue::new(&mut self.example_round).range(0..=23));
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
                self.example_state.keccak_f_1600()
            }
        });

        ui.add_space(16.0);
    }

    fn hash_string(&self, text: &str) -> Result<String, HasherError> {
        let bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| hashers::errors::HasherError::general("byte format error"))?;

        let h = match self.variant {
            Variant::Sha3_224 => Keccack::sha3_224().hash(&bytes),
            Variant::Sha3_256 => Keccack::sha3_256().hash(&bytes),
            Variant::Sha3_384 => Keccack::sha3_384().hash(&bytes),
            Variant::Sha3_512 => Keccack::sha3_512().hash(&bytes),
            Variant::Shake128 => Keccack::shake_128(self.shake_hash_len).hash(&bytes),
            Variant::Shake256 => Keccack::shake_256(self.shake_hash_len).hash(&bytes),
            Variant::CShake128 => Keccack::cshake_128(
                self.shake_hash_len,
                self.function_name.as_bytes(),
                self.customization.as_bytes(),
            )
            .hash(&bytes),
            Variant::CShake256 => Keccack::cshake_256(
                self.shake_hash_len,
                self.function_name.as_bytes(),
                self.customization.as_bytes(),
            )
            .hash(&bytes),
        };

        Ok(self.output_format.byte_slice_to_text(&h))
    }
}
