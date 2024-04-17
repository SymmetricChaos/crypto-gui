use super::ClassicRngFrame;
use crate::ui_elements::{generate_random_u32s_box, UiElements};
use egui::{DragValue, FontId, RichText};
use rand::{thread_rng, Rng};
use rngs::vmpcr::Vmpcr;
use utils::byte_formatting::ByteFormat;

pub struct VmpcrFrame {
    rng: Vmpcr,
    key: String,
    iv: String,
    key_bytes: Option<Vec<u8>>,
    iv_bytes: Option<Vec<u8>>,
    random_bytes: String,
    randoms: String,
    n_random_bytes: usize,
    n_random: usize,
}

impl Default for VmpcrFrame {
    fn default() -> Self {
        let mut rng = Vmpcr::default();
        rng.ksa(
            &[0xDE_u8, 0xAD, 0xBE, 0xEF, 0x42],
            &[0xBA, 0xAD, 0xF0, 0x0D],
        );
        Self {
            rng: Default::default(),
            key: String::from("DEADBEEF42"),
            iv: String::from("BAADF00D"),
            key_bytes: Some(vec![0xDE_u8, 0xAD, 0xBE, 0xEF, 0x42]),
            iv_bytes: Some(vec![0xBA, 0xAD, 0xF0, 0x0D]),
            random_bytes: String::new(),
            randoms: String::new(),
            n_random_bytes: 5,
            n_random: 5,
        }
    }
}

impl VmpcrFrame {}

impl ClassicRngFrame for VmpcrFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.add_space(16.0);

        ui.horizontal(|ui| {
            ui.subheading("Key");
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                let mut rng = thread_rng();
                self.key = format!("{:08X}", rng.gen::<u64>());
                self.key_bytes = Some(ByteFormat::Hex.text_to_bytes(&self.key).unwrap());
            }
        });
        ui.label("Up to 256 bytes as hexadecimal.");
        if ui.text_edit_multiline(&mut self.key).changed() {
            self.key = self.key.chars().filter(|c| c.is_ascii_hexdigit()).collect();
            self.key_bytes = match ByteFormat::Hex.text_to_bytes(&self.key) {
                Ok(b) => Some(b),
                Err(_) => None,
            }
        }
        if self.key_bytes.is_none() {
            ui.error_text("Invalid Key Bytes");
        } else {
            ui.error_text("");
        }
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.subheading("Initialization Vector");
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                let mut rng = thread_rng();
                self.iv = format!("{:08X}", rng.gen::<u64>());
                self.iv_bytes = Some(ByteFormat::Hex.text_to_bytes(&self.iv).unwrap());
            }
        });
        ui.label("Up to 256 bytes as hexadecimal.");
        if ui.text_edit_multiline(&mut self.iv).changed() {
            self.iv = self.iv.chars().filter(|c| c.is_ascii_hexdigit()).collect();
            self.iv_bytes = match ByteFormat::Hex.text_to_bytes(&self.iv) {
                Ok(b) => Some(b),
                Err(_) => None,
            }
        }
        if self.iv_bytes.is_none() {
            ui.error_text("Invalid IV Bytes");
        } else {
            ui.error_text("");
        }

        ui.add_enabled_ui(self.key_bytes.is_some() && self.iv_bytes.is_some(), |ui| {
            if ui.button("Run Key Scheduling Algorithm").clicked() {
                self.rng.ksa(
                    self.key_bytes.as_ref().unwrap(),
                    self.iv_bytes.as_ref().unwrap(),
                )
            }
        });

        ui.add_space(16.0);

        ui.collapsing("Internal State", |ui| {
            ui.subheading("Array P");
            egui::Grid::new("vmpc_p_array")
                .num_columns(16)
                .striped(true)
                .show(ui, |ui| {
                    for (n, b) in self.rng.arr_p.into_iter().enumerate() {
                        if n % 16 == 0 && n != 0 {
                            ui.end_row()
                        }
                        ui.label(
                            RichText::from(format!("{:02X}", b)).font(FontId::monospace(15.0)),
                        );
                    }
                });
            ui.subheading("\n\nArray S");
            egui::Grid::new("vmpc_s_array")
                .num_columns(16)
                .striped(true)
                .show(ui, |ui| {
                    for (n, b) in self.rng.arr_s.into_iter().enumerate() {
                        if n % 16 == 0 && n != 0 {
                            ui.end_row()
                        }
                        ui.label(
                            RichText::from(format!("{:02X}", b)).font(FontId::monospace(15.0)),
                        );
                    }
                });
        });

        ui.add_space(16.0);
        if ui.button("step").clicked() {
            self.rng.next_byte();
        }
        // ui.collapsing("explain", |ui| {
        //     ui.label("To generate a value just two kinds of operations are used, addition of bytes and swapping of array elements. Note that addition \"wraps around\" after 255, this is also known as addition modulo 256. The procedure is as follows:\n\nAdd 1 to 'i'. Add the value that 'i' points to, to 'j'. Swap the values that 'i' and 'j' point to. Add the values that 'i' and 'j' point to. The byte at this location is the output.\n\nThis can be written more compactly. Here the notation A[x] means the byte at positionx in the array.\n\ni = i + 1\nj = j + A[i]\nswap A[i] with A[j]\nt = A[i] + A[j] (create a temporary value)\noutput A[t]")
        // });

        ui.add_space(16.0);
        ui.horizontal(|ui| {
            if ui.button("Random Bytes").clicked() {
                for _ in 0..self.n_random_bytes {
                    if !self.random_bytes.is_empty() {
                        self.random_bytes.push_str(", ");
                    }
                    self.random_bytes
                        .push_str(&format!("{:02X}", self.rng.next_byte()));
                }
            }
            ui.add(DragValue::new(&mut self.n_random_bytes).clamp_range(1..=10))
        });
        ui.text_edit_multiline(&mut self.random_bytes);

        ui.add_space(16.0);
        generate_random_u32s_box(ui, &mut self.rng, &mut self.n_random, &mut self.randoms);
        ui.add_space(16.0);
    }

    fn rng(&self) -> &dyn rngs::ClassicRng {
        &self.rng
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.key = format!("{:08X}", rng.gen::<u64>());
        self.iv = format!("{:08X}", rng.gen::<u64>());
        self.key_bytes = Some(ByteFormat::Hex.text_to_bytes(&self.key).unwrap());
        self.iv_bytes = Some(ByteFormat::Hex.text_to_bytes(&self.iv).unwrap());
        self.rng.ksa(
            self.key_bytes.as_ref().unwrap(),
            self.iv_bytes.as_ref().unwrap(),
        )
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
