use crate::ui_elements::UiElements;

use super::HasherFrame;
use egui::DragValue;
use hashers::siphash::SipHash;
use rand::{thread_rng, Rng};

pub struct SipHashFrame {
    hasher: SipHash,
    k0_string: String,
    k1_string: String,
}

impl Default for SipHashFrame {
    fn default() -> Self {
        Self {
            hasher: Default::default(),
            k0_string: String::new(),
            k1_string: String::new(),
        }
    }
}

impl SipHashFrame {
    fn key_control(ui: &mut egui::Ui, string: &mut String, key: &mut u64) {
        ui.horizontal(|ui| {
            if ui.control_string(string).changed() {
                *string = string
                    .chars()
                    .filter(|c| c.is_ascii_hexdigit())
                    .take(16)
                    .collect();

                if ui.button("🎲").on_hover_text("randomize").clicked() {
                    let mut rng = thread_rng();
                    *key = rng.gen();
                    *string = format!("{:016x}", key);
                }
                match u64::from_str_radix(string, 16) {
                    Ok(new) => *key = new.to_be(),
                    Err(_) => {
                        ui.error_text("unable to parse key");
                    }
                };
            }
        });
    }
}

impl HasherFrame for SipHashFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/siphash.rs",
        );

        ui.byte_io_mode_hasher(
            &mut self.hasher.input_format,
            &mut self.hasher.output_format,
        );

        ui.add_space(16.0);
        ui.subheading("Key0 (hexadecimal)");
        Self::key_control(ui, &mut self.k0_string, &mut self.hasher.k0);
        ui.add_space(8.0);
        ui.subheading("Key1 (hexadecimal)");
        Self::key_control(ui, &mut self.k1_string, &mut self.hasher.k1);

        ui.add_space(16.0);
        ui.subheading("Compression Rounds");
        ui.add(DragValue::new(&mut self.hasher.compression_rounds).range(0..=8));

        ui.add_space(8.0);
        ui.subheading("Finalization Rounds");
        ui.add(DragValue::new(&mut self.hasher.finalization_rounds).range(1..=10));

        ui.add_space(16.0);
    }

    crate::hash_string! {}
}
