use crate::ui_elements::{control_string, UiElements};

use super::{byte_formatting_io, HasherFrame};
use egui::Button;
use hashers::{
    blake::blake3::{Blake3, Blake3Mode},
    errors::HasherError,
    traits::ClassicHasher,
};
use rand::{thread_rng, RngCore};
use utils::byte_formatting::ByteFormat;

pub struct Blake3Frame {
    hasher: Blake3,
    key_string: String,
    context_string: String,
    valid_key: bool,
}

impl Default for Blake3Frame {
    fn default() -> Self {
        Self {
            hasher: Default::default(),
            key_string: String::new(),
            context_string: String::new(),
            valid_key: false,
        }
    }
}

impl Blake3Frame {
    fn key_control(&mut self, ui: &mut egui::Ui) {
        let string = &mut self.key_string;
        let enabled = self.hasher.mode == Blake3Mode::Keyed;
        ui.subheading("Key");
        ui.label(
            "Exactly 64 hexadecimal digits must be supplied, specifying all 256 bits of the key.",
        );
        ui.horizontal(|ui| {
            if control_string(ui, string, enabled).changed() {
                *string = string
                    .chars()
                    .filter(|c| c.is_ascii_hexdigit())
                    .take(64)
                    .collect();
            };
            if string.len() != 64 {
                self.valid_key = false;
            } else {
                self.valid_key = true;
            }
            if ui
                .add_enabled(enabled, Button::new("🎲"))
                .on_hover_text("randomize")
                .clicked()
            {
                let mut rng = thread_rng();
                rng.fill_bytes(&mut self.hasher.key);
                *string = ByteFormat::Hex.byte_slice_to_text(&mut self.hasher.key)
            }
        });

        if self.valid_key {
            if let Ok(new) = ByteFormat::Hex.text_to_bytes(string) {
                match new.try_into() {
                    Ok(key) => self.hasher.key = key,
                    Err(_) => unreachable!(),
                }
            } else {
                unreachable!("unable to parse input");
            }
        } else {
            if self.hasher.mode == Blake3Mode::Keyed {
                ui.error_text("invalid key");
            } else {
                ui.error_text("");
            }
        }
    }

    fn context_control(&mut self, ui: &mut egui::Ui) {
        let string = &mut self.context_string;
        let enabled = self.hasher.mode == Blake3Mode::KeyDerivation;
        ui.subheading("Context String");
        ui.label("For key derivation a static globally unique string must be provided. The suggested format is [application] [creation timestamp] [purpose].");
        ui.horizontal(|ui| {
            control_string(ui, string, enabled);
        });
    }
}

impl HasherFrame for Blake3Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.add_space(16.0);

        byte_formatting_io(
            ui,
            &mut self.hasher.input_format,
            &mut self.hasher.output_format,
        );
        ui.add_space(16.0);

        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.hasher.mode, Blake3Mode::Unkeyed, "Unkeyed");
            ui.selectable_value(&mut self.hasher.mode, Blake3Mode::Keyed, "Keyed");
            ui.selectable_value(
                &mut self.hasher.mode,
                Blake3Mode::KeyDerivation,
                "Key Derivation",
            );
        });

        ui.add_space(16.0);

        self.key_control(ui);

        ui.add_space(16.0);

        self.context_control(ui);

        ui.add_space(16.0);

        ui.subheading("Hash Length");
        ui.label("BLAKE3 allows a variety of output lengths, with a default of 32 bytes (256 bits). While up to 2^64 bytes can be returned this interface limits output to 256 bytes (2048 bits). Unlike BLAKE2 there is no domain seperation for different lengths so short outputs are prefixes of long ones.");
        ui.add(egui::DragValue::new(&mut self.hasher.hash_len).clamp_range(1..=256));
    }

    fn hash_bytes_from_string(&self, text: &str) -> Result<String, HasherError> {
        if self.hasher.mode == Blake3Mode::Keyed && !self.valid_key {
            Err(HasherError::key("BLAKE3 keyed hash can only be called when exactly 64 hexadecimal digits (256 bits) of key are given"))
        } else {
            self.hasher.hash_bytes_from_string(text)
        }
    }
}
