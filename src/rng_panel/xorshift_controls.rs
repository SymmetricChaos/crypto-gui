use super::ClassicRngFrame;
use crate::ui_elements::UiElements;
use rand::{thread_rng, Rng};
use rngs::{xorshift::Xorshift, ClassicRng};

pub struct XorshiftFrame {
    rng: Xorshift,
    key: String,
    randoms: String,
}

impl Default for XorshiftFrame {
    fn default() -> Self {
        Self {
            rng: Default::default(),
            key: String::new(),
            randoms: String::new(),
        }
    }
}

impl XorshiftFrame {}

impl ClassicRngFrame for XorshiftFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.add_space(16.0);

        ui.horizontal(|ui| {
            ui.subheading("Key");
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                self.randomize();
            }
        });
        ui.label("Key should be provided as a string of hexadecimal digits.");
        if ui.button("set").clicked() {
            self.rng.state =
                u32::from_str_radix(&self.key, 8).expect("filtering should force this to be valid")
        }
        if ui.text_edit_multiline(&mut self.key).changed() {
            self.key = self
                .key
                .chars()
                .filter(|c| c.is_ascii_hexdigit())
                .take(16)
                .collect();
            self.rng.state =
                u32::from_str_radix(&self.key, 16).expect("filtering should force this to be valid")
        }

        ui.add_space(16.0);
        ui.subheading("Internal State");
        ui.label(format!("{:04X}", self.rng.state));

        ui.add_space(16.0);
        if ui.button("step").clicked() {
            self.rng.next_u32();
        }

        ui.add_space(16.0);
        if ui.button("Random Numbers").clicked() {
            for _ in 0..5 {
                if !self.randoms.is_empty() {
                    self.randoms.push_str(", ");
                }
                self.randoms.push_str(&self.rng.next_u32().to_string());
            }
        }
        ui.text_edit_multiline(&mut self.randoms);
        ui.add_space(16.0);
    }

    fn rng(&self) -> &dyn rngs::ClassicRng {
        &self.rng
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.key = format!("{:04X}", rng.gen::<u32>());
        self.rng.state =
            u32::from_str_radix(&self.key, 16).expect("thread_rng should have provided a valid u32")
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
