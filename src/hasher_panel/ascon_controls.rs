use super::HasherFrame;
use crate::ui_elements::UiElements;
use hashers::{
    ascon::{hash::Ascon, Variant},
    errors::HasherError,
};
use rand::{thread_rng, Rng};
use utils::byte_formatting::{u64s_to_bytes_be, ByteFormat};

pub struct AsconFrame {
    input_format: ByteFormat,
    output_format: ByteFormat,
    variant: Variant,
    hash_len: usize,
    key: [u64; 2],
}

impl Default for AsconFrame {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            variant: Variant::Hash,
            hash_len: 32,
            key: [0; 2],
        }
    }
}

impl HasherFrame for AsconFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/ascon",
        );
        ui.add_space(8.0);

        ui.byte_io_mode_hasher(&mut self.input_format, &mut self.output_format);
        ui.add_space(4.0);

        ui.subheading("Variant");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.variant, Variant::Hash, Variant::Hash.to_string());
            ui.selectable_value(
                &mut self.variant,
                Variant::Hasha,
                Variant::Hasha.to_string(),
            );
        });
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.variant, Variant::Xof, Variant::Xof.to_string());
            ui.selectable_value(&mut self.variant, Variant::Xofa, Variant::Xofa.to_string());
        });
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.variant, Variant::Mac, Variant::Mac.to_string());
            ui.selectable_value(&mut self.variant, Variant::Maca, Variant::Maca.to_string());
        });
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.variant, Variant::Prf, Variant::Prf.to_string());
            ui.selectable_value(&mut self.variant, Variant::Prfa, Variant::Prfa.to_string());
        });

        ui.add_space(4.0);

        match self.variant {
            Variant::Hash => {
                ui.label("Ascon-Hash absorbs 64 bits of input at a time and returns a 256-bit hash. There are 12 rounds for initialization, absorbing, and squeezing.");
            }
            Variant::Hasha => {
                ui.label("Ascon-Hasha absorbs 64 bits of input at a time and returns a 256-bit hash. There are 12 initialization rounds and 8 rounds for all other steps.");
            }
            Variant::Xof => {
                ui.label("Ascon-XOF absorbs 64 bits of input at a time and returns any amount of data. There are 12 rounds for initialization, absorbing, and squeezing.");
                ui.label("Output Length");
                ui.add(egui::DragValue::new(&mut self.hash_len).range(1..=2048));
            }
            Variant::Xofa => {
                ui.label("Ascon-XOFa absorbs 64 bits of input at a time and returns any amount of data. There are 12 initialization rounds and 8 rounds for all other steps.");
                ui.label("Output Length");
                ui.add(egui::DragValue::new(&mut self.hash_len).range(1..=2048));
            }
            Variant::Mac => {
                ui.label("Ascon-MAC returns a 128-bit Message Authentication Code on the key and message. It applies 12 rounds for initialization, absorbing, and squeezing. It absorbs 256-bits at a time.");
            }
            Variant::Maca => {
                ui.label("Ascon-MACa returns a 128-bit Message Authentication Code based on the key and message. It applies 12 initialization rounds but only 8 rounds for absorbing and squeezing. It absorbs 320-bits at a time.");
            }
            Variant::Prf => {
                ui.label("Ascon-PRF returns any amount of data based on the key and message. It applies 12 rounds for initialization, absorbing, and squeezing. It absorbs 256-bits at a time.");
                ui.label("Output Length");
                ui.add(egui::DragValue::new(&mut self.hash_len).range(1..=2048));
            }
            Variant::Prfa => {
                ui.label("Ascon-PRFa returns any amount of data based on the key and message. It applies 12 initialization rounds but only 8 rounds for absorbing and squeezing. Output length is unlimited. It absorbs 320-bits at a time.");
                ui.label("Output Length");
                ui.add(egui::DragValue::new(&mut self.hash_len).range(1..=2048));
            } // Variant::AsconPrfShort => {
              //     ui.label("Ascon-PRFshort is significantly unlike the other variants in order to process small inputs very quickly. Inputs are limited to 128-bits and the initial state incorporates the input itself. Both message length and the chosen output length are used for domain separation. Most unusually no absorbing rounds are applied. Instead after the 12 initialization rounds the key is XORed into the state and the hash (limited to 128-bits) is immediately extracted.");
              //     self.hasher.hash_len = self.hasher.hash_len.clamp(1, 16);
              // }
        }

        ui.add_space(8.0);

        if [Variant::Mac, Variant::Maca, Variant::Prf, Variant::Prfa].contains(&self.variant) {
            ui.horizontal(|ui| {
                ui.label(format!(
                    "{} requires a 128-bit key which is treated as a pair of 64-bit words.",
                    self.variant.to_string()
                ));
                if ui.button("🎲").on_hover_text("randomize").clicked() {
                    let mut rng = thread_rng();
                    rng.fill(&mut self.key);
                };
            });
            ui.u64_hex_edit(&mut self.key[0]);
            ui.u64_hex_edit(&mut self.key[1]);
        }

        ui.add_space(16.0);
    }

    fn hash_string(&self, text: &str) -> Result<String, HasherError> {
        let bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| hashers::errors::HasherError::general("byte format error"))?;

        let mut key = [0; 16];
        u64s_to_bytes_be(&mut key, &self.key);

        let h = match self.variant {
            Variant::Hash => Ascon::hash(&bytes),
            Variant::Hasha => Ascon::hasha(&bytes),
            Variant::Xof => Ascon::xof(&bytes, self.hash_len),
            Variant::Xofa => Ascon::xofa(&bytes, self.hash_len),
            Variant::Mac => Ascon::mac(&bytes, key),
            Variant::Maca => Ascon::mac(&bytes, key),
            Variant::Prf => Ascon::prf(&bytes, key, self.hash_len),
            Variant::Prfa => Ascon::prfa(&bytes, key, self.hash_len),
        };

        Ok(self.output_format.byte_slice_to_text(&h))
    }
}
