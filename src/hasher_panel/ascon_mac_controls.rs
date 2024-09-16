use hashers::ascon::mac::{AsconMac, Variant};
use strum::IntoEnumIterator;

use super::HasherFrame;
use crate::ui_elements::UiElements;

pub struct AsconMacFrame {
    hasher: AsconMac,
}

impl Default for AsconMacFrame {
    fn default() -> Self {
        Self {
            hasher: AsconMac::default(),
        }
    }
}

impl HasherFrame for AsconMacFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.add_space(16.0);

        ui.byte_io_mode_hasher(
            &mut self.hasher.input_format,
            &mut self.hasher.output_format,
        );
        ui.add_space(4.0);

        for variant in Variant::iter() {
            ui.selectable_value(&mut self.hasher.variant, variant, variant.to_string());
        }
        match self.hasher.variant {
            Variant::AsconMac => {
                ui.label("Ascon-MAC absorbs 256-bits of input at a time. It applies 12 rounds for initialization, absorbing, and squeezing.");
                self.hasher.hash_len = self.hasher.hash_len.clamp(1, 16);
            }
            Variant::AsconMaca => {
                ui.label("Ascon-MACa absorbs 320-bits of input at a time. It applies 12 initialization rounds but only eight rounds for absorbing and squeezing.");
                self.hasher.hash_len = self.hasher.hash_len.clamp(1, 16);
            }
            Variant::AsconPrf => {
                ui.label("Ascon-PRF absorbs 256-bits of input at a time. It applies 12 rounds for initialization, absorbing, and squeezing. Unlike Ascon-MAC output length is unlimited.");
            }
            Variant::AsconPrfa => {
                ui.label("Ascon-PRF absorbs 256-bits of input at a time. It applies 12 initialization rounds but only eight rounds for absorbing and squeezing. Unlike Ascon-MACa output length is unlimited.");
            }
            Variant::AsconPrfShort => {
                ui.label("Ascon-PRFshort is significantly unlike the other variants in order to process small inputs very quickly. Inputs are limited to 128-bits and the initial state incorporates the input itself. Both message length and the chosen output length are used for domain separation. Most unusually no absorbing rounds are applied. Instead after the 12 initialization rounds the key is XORed into the state and the hash (limited to 128-bits) is immediately extracted.");
                self.hasher.hash_len = self.hasher.hash_len.clamp(1, 16);
            }
        }
        ui.add_space(4.0);

        ui.subheading("Key");
        ui.label("All variants on Ascon-MAC require a 128-bit key which is treated as a pair of 64-bit words.");
        ui.u64_hex_edit(&mut self.hasher.key[0]);
        ui.u64_hex_edit(&mut self.hasher.key[1]);
        ui.add_space(4.0);

        ui.subheading("Hash Length");
        match self.hasher.variant {
            Variant::AsconMac => {
                ui.label("Ascon-MAC can return a hash of 1 to 16 bytes (8 to 128 bits).");
                ui.add(egui::DragValue::new(&mut self.hasher.hash_len).range(1..=16));
            }
            Variant::AsconMaca => {
                ui.label("Ascon-MACa can return a hash of 1 to 16 bytes (8 to 128 bits).");
                ui.add(egui::DragValue::new(&mut self.hasher.hash_len).range(1..=16));
            }
            Variant::AsconPrf => {
                ui.label("Ascon-PRF can return an output of any length but here is limited to 512 bytes (4096 bits).");
                ui.add(egui::DragValue::new(&mut self.hasher.hash_len).range(1..=512));
            }
            Variant::AsconPrfa => {
                ui.label("Ascon-PRFa can return an output of any length but here is limited to 512 bytes (4096 bits).");
                ui.add(egui::DragValue::new(&mut self.hasher.hash_len).range(1..=512));
            }
            Variant::AsconPrfShort => {
                ui.label("Ascon-PRFshort returns a hash of 1 to 16 bytes (8 to 128 bits).");
                ui.add(egui::DragValue::new(&mut self.hasher.hash_len).range(1..=16));
            }
        }

        ui.add_space(16.0);
    }

    crate::hash_string! {}
}
