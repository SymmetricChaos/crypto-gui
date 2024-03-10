use crate::ui_elements::UiElements;

use super::{byte_formatting_io, HasherFrame};
use hashers::{blake::blake3::Blake3, errors::HasherError, traits::ClassicHasher};
use rand::{thread_rng, RngCore};
use utils::byte_formatting::ByteFormat;

pub struct Blake3Frame {
    hasher: Blake3,
    key_string: String,
    hash_len: String,
}

impl Default for Blake3Frame {
    fn default() -> Self {
        Self {
            hasher: Default::default(),
            key_string: String::new(),
            hash_len: String::from("32"),
        }
    }
}

impl Blake3Frame {
    fn key_control(ui: &mut egui::Ui, string: &mut String, bytes: &mut [u8; 32]) {
        ui.horizontal(|ui| {
            if ui.control_string(string).changed() {
                *string = string.chars().filter(|c| c.is_ascii_hexdigit()).collect();
                if ui.button("🎲").on_hover_text("randomize").clicked() {
                    let mut rng = thread_rng();
                    rng.fill_bytes(bytes);
                    *string = ByteFormat::Hex.byte_slice_to_text(&bytes)
                }
                if string.len() != 64 {
                    ui.error_text("the key must be exactly 64 hexadecimal digits");
                }
                if let Ok(new) = ByteFormat::Hex.text_to_bytes(string) {
                    match new.try_into() {
                        Ok(key) => *bytes = key,
                        Err(_) => todo!(),
                    }
                } else {
                    ui.error_text("unable to parse input");
                }
            };
        });
    }

    fn hash_len_control(ui: &mut egui::Ui, string: &mut String, value: &mut usize, max: usize) {
        ui.horizontal(|ui| {
            if ui.control_string(string).changed() {
                *string = string
                    .chars()
                    .filter(|c| c.is_ascii_digit())
                    .take(2)
                    .collect();
                match usize::from_str_radix(string, 10) {
                    Ok(new) => {
                        if new == 0 || new > max {
                            ui.error_text("invalid hash length_size");
                        } else {
                            *value = new
                        }
                    }
                    Err(_) => {
                        ui.error_text("unable to parse hash_len value");
                    }
                };
            }
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

        ui.subheading("Hash Length");
        ui.label("The BLAKE3 function allows a variety of output lengths.");
        // Self::hash_len_control(ui, &mut self.hash_len, &mut self.hasher.hash_len, 32);

        ui.add_space(16.0);
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.hasher.keyed_hash, true, "Keyed");
            ui.selectable_value(&mut self.hasher.keyed_hash, false, "Unkeyed");
        });
        if self.hasher.keyed_hash {
            Self::key_control(ui, &mut self.key_string, &mut self.hasher.key)
        }

        // ui.label("<<<EXPLANATION OF HASH FUNCTION CODE>>>");

        ui.add_space(16.0);
    }

    fn hash_bytes_from_string(&self, text: &str) -> Result<String, HasherError> {
        self.hasher.hash_bytes_from_string(text)
    }
}
