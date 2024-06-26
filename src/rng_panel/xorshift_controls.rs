use super::ClassicRngFrame;
use crate::ui_elements::{generate_random_u32s_box, UiElements};
use rand::{thread_rng, Rng};
use rngs::{xorshift::Xorshift64, ClassicRng};

pub struct XorshiftFrame {
    rng: Xorshift64,
    key: String,
    randoms: String,
    n_random: usize,
}

impl Default for XorshiftFrame {
    fn default() -> Self {
        Self {
            rng: Default::default(),
            key: String::from("0BAD5EED0BAD5EED"),
            randoms: String::new(),
            n_random: 5,
        }
    }
}

impl XorshiftFrame {}

impl ClassicRngFrame for XorshiftFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.add_space(16.0);

        ui.horizontal(|ui| {
            ui.subheading("Seed Value");
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                self.randomize();
            }
        });
        ui.horizontal(|ui| {
            ui.label("Seed should be provided as a string of hexadecimal digits.");
            if ui.button("set").clicked() {
                self.rng.state = u64::from_str_radix(&self.key, 16)
                    .expect("filtering should force this to be valid");
            }
        });
        if ui.text_edit_singleline(&mut self.key).changed() {
            self.key = self
                .key
                .chars()
                .filter(|c| c.is_ascii_hexdigit())
                .take(16)
                .collect();
            self.rng.state = u64::from_str_radix(&self.key, 16)
                .expect("filtering should force this to be valid");
        }

        ui.add_space(16.0);
        ui.subheading("Internal State");
        ui.label(format!("{:016X}", self.rng.state));

        ui.add_space(16.0);
        if ui.button("step").clicked() {
            self.rng.next_u32();
        }
        ui.collapsing("calculations", |ui| {

            let mut t = self.rng.state;
            ui.monospace(format!(
                "{:016X}  ⊕  {:016X}  =  {:016X}    (XOR the state with itself shifted left by 13 bits)",
                t,
                t << 13,
                t ^ (t << 13)
            ));
            t ^= t << 13;


            ui.monospace(format!(
                "{:016X}  ⊕  {:016X}  =  {:016X}    (XOR the state with itself shifted right by 17 bits)",
                t,
                t >> 7,
                t ^ (t >> 7)
            ));
            t ^= t >> 7;

            ui.monospace(format!(
                "{:016X}  ⊕  {:016X}  =  {:016X}    (XOR the state with itself shifted left by 5 bits)",
                t,
                t << 13,
                t ^ (t << 13)
            ));
        });

        ui.add_space(16.0);
        generate_random_u32s_box(ui, &mut self.rng, &mut self.n_random, &mut self.randoms);
        ui.add_space(16.0);
    }

    fn rng(&self) -> &dyn rngs::ClassicRng {
        &self.rng
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.rng.state = rng.gen::<u64>();
        self.key = format!("{:016X}", self.rng.state);
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
