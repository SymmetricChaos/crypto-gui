use super::HasherFrame;
use crate::ui_elements::{validate_string_hex_bytes, UiElements};
use egui::DragValue;
use hashers::{
    errors::HasherError,
    sha::{Keccack, KeccackState},
    traits::StatefulHasher,
};
use rand::{thread_rng, RngCore};
use utils::byte_formatting::ByteFormat;

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
    Kmac128,
    Kmac256,
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
            Variant::Kmac128 => true,
            Variant::Kmac256 => true,
        }
    }

    fn function_name(&self) -> bool {
        match self {
            Variant::Sha3_224 => false,
            Variant::Sha3_256 => false,
            Variant::Sha3_384 => false,
            Variant::Sha3_512 => false,
            Variant::Shake128 => false,
            Variant::Shake256 => false,
            Variant::CShake128 => true,
            Variant::CShake256 => true,
            Variant::Kmac128 => false,
            Variant::Kmac256 => false,
        }
    }

    fn customization(&self) -> bool {
        match self {
            Variant::Sha3_224 => false,
            Variant::Sha3_256 => false,
            Variant::Sha3_384 => false,
            Variant::Sha3_512 => false,
            Variant::Shake128 => false,
            Variant::Shake256 => false,
            Variant::CShake128 => true,
            Variant::CShake256 => true,
            Variant::Kmac128 => true,
            Variant::Kmac256 => true,
        }
    }

    fn keyed(&self) -> bool {
        match self {
            Variant::Sha3_224 => false,
            Variant::Sha3_256 => false,
            Variant::Sha3_384 => false,
            Variant::Sha3_512 => false,
            Variant::Shake128 => false,
            Variant::Shake256 => false,
            Variant::CShake128 => false,
            Variant::CShake256 => false,
            Variant::Kmac128 => true,
            Variant::Kmac256 => true,
        }
    }
}

pub struct Sha3Frame {
    input_format: ByteFormat,
    output_format: ByteFormat,
    variant: Variant,
    hash_len: u64,
    example_state: KeccackState,
    example_round: usize,
    function_name: String,
    customization: String,
    key: Vec<u8>,
    key_string: String,
}

impl Default for Sha3Frame {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            variant: Variant::Sha3_256,
            hash_len: 32,
            example_state: KeccackState::new(),
            example_round: 0,
            function_name: String::new(),
            customization: String::new(),
            key: Vec::new(),
            key_string: String::new(),
        }
    }
}

impl Sha3Frame {
    fn validate_key(&mut self) {
        validate_string_hex_bytes(&mut self.key_string, Some(16));
        self.key = ByteFormat::Hex
            .text_to_bytes(&self.key_string)
            .expect("unable to parse salt input");
    }

    fn key_control(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.control_string(&mut self.key_string).lost_focus() {
                self.validate_key();
            };
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                let mut rng = thread_rng();
                self.key = vec![0; 32];
                rng.fill_bytes(&mut self.key);
                self.key_string = ByteFormat::Hex.byte_slice_to_text(&self.key)
            };
        });
    }
}

impl HasherFrame for Sha3Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/sha/sha3.rs",
        );

        ui.add_space(8.0);
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
        ui.add_space(8.0);
        ui.subheading("Message Authentication Codes");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.variant, Variant::Kmac128, "KMAC128");
            ui.selectable_value(&mut self.variant, Variant::Kmac256, "KMAC256");
        });

        ui.add_space(16.0);
        ui.subheading("Discussion");
        match self.variant {
            Variant::Sha3_224 => ui.label("SHA3-224 absorbs 1152 bits at a time and returns a 224-bit hash."),
            Variant::Sha3_256 => ui.label("SHA3-256 absorbs 1088 bits at a time and returns a 256-bit hash."),
            Variant::Sha3_384 => ui.label("SHA3-384 absorbs 832 bits at a time and returns a 384-bit hash."),
            Variant::Sha3_512 => ui.label("SHA3-512 absorbs 576 bits at a time. and returns a 512-bit hash"),
            Variant::Shake128 => ui.label("SHAKE128 absorbs 1344 bits at a time. It can be set to return any number of bits but only claims 128 bits of security."),
            Variant::Shake256 => ui.label("SHAKE256 absorbs 1088 bits at a time. It can be set to return any number of bits but only claims 256 bits of security.."),
            Variant::CShake128 => ui.label("cSHAKE128 is similar to SHAKE128 but allows both a function name and customization string. This functions is intended to be used by NIST in the creation of new Keccack based functions. It absorbs 1344 bits at a time and offers 128-bit security."),
            Variant::CShake256 => ui.label("cSHAKE256 is similar to SHAKE256 but allows both a function name and customization string. This functions is intended to be used by NIST in the creation of new Keccack based functions. It absorbs 1088 bits at a time and offers 256-bit security."),
            Variant::Kmac128 => ui.label("KMAC128 is a cSHAKE128 derived Message Authentical Code. It accepts a key and customization string. Output length is adjustsable but in order to meet the security claim the output length and key must both be sufficient."),
            Variant::Kmac256 => ui.label("KMAC256 is a cSHAKE256 derived Message Authentical Code. It accepts a key and customization string. Output length is adjustsable but in order to meet the security claim the output length and key must both be sufficient."),
        };

        ui.add_space(12.0);
        if self.variant.adjustable_length() {
            ui.add_space(4.0);
            ui.subheading("Output Length (bytes)");
            ui.add(DragValue::new(&mut self.hash_len).range(1..=1024));
        }
        if self.variant.function_name() {
            ui.add_space(4.0);
            ui.subheading("cSHAKE Function Name (UTF-8)");
            ui.control_string(&mut self.function_name);
        }
        if self.variant.customization() {
            ui.add_space(4.0);
            ui.subheading("Customization String (UTF-8)");
            ui.control_string(&mut self.customization);
        }
        if self.variant.keyed() {
            ui.add_space(4.0);
            ui.subheading("Key (Hexadecimal)");
            self.key_control(ui);
        }

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
                self.example_round = (self.example_round + 1) % 24;
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
            Variant::Sha3_224 => Keccack::sha3_224(),
            Variant::Sha3_256 => Keccack::sha3_256(),
            Variant::Sha3_384 => Keccack::sha3_384(),
            Variant::Sha3_512 => Keccack::sha3_512(),
            Variant::Shake128 => Keccack::shake_128(self.hash_len),
            Variant::Shake256 => Keccack::shake_256(self.hash_len),
            Variant::CShake128 => Keccack::cshake_128(
                self.hash_len,
                self.function_name.as_bytes(),
                self.customization.as_bytes(),
            ),
            Variant::CShake256 => Keccack::cshake_256(
                self.hash_len,
                self.function_name.as_bytes(),
                self.customization.as_bytes(),
            ),
            Variant::Kmac128 => {
                Keccack::kmac_128(&self.key, self.hash_len, self.customization.as_bytes())
            }
            Variant::Kmac256 => {
                Keccack::kmac_256(&self.key, self.hash_len, self.customization.as_bytes())
            }
        }
        .update_and_finalize(&bytes);

        Ok(self.output_format.byte_slice_to_text(&h))
    }
}
