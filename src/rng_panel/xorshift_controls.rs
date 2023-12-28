use super::ClassicRngFrame;
use crate::ui_elements::{generate_random_nums_box, UiElements};
use rand::{thread_rng, Rng};
use rngs::{xorshift::Xorshift32, ClassicRng};

pub struct XorshiftFrame {
    rng: Xorshift32,
    key: String,
    randoms: String,
    n_random: usize,
    s0: u32,
    s1: u32,
    s2: u32,
}

impl Default for XorshiftFrame {
    fn default() -> Self {
        Self {
            rng: Default::default(),
            key: String::new(),
            randoms: String::new(),
            n_random: 5,
            s0: 0,
            s1: 0,
            s2: 0,
        }
    }
}

impl XorshiftFrame {
    fn set_shifts(&mut self) {
        self.s0 = self.rng.state ^ (self.rng.state << 13);
        self.s1 = self.s0 ^ (self.s0 >> 17);
        self.s2 = self.s1 ^ (self.s1 << 5);
    }
}

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
                self.rng.state = u32::from_str_radix(&self.key, 16)
                    .expect("filtering should force this to be valid");
                self.set_shifts();
            }
        });
        if ui.text_edit_singleline(&mut self.key).changed() {
            self.key = self
                .key
                .chars()
                .filter(|c| c.is_ascii_hexdigit())
                .take(8)
                .collect();
            self.rng.state = u32::from_str_radix(&self.key, 16)
                .expect("filtering should force this to be valid");
            self.set_shifts();
        }

        ui.add_space(16.0);
        ui.subheading("Internal State");
        ui.label(format!("{:08X}", self.rng.state));

        ui.add_space(16.0);
        if ui.button("step").clicked() {
            self.rng.next_u32();
            self.set_shifts();
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
                t >> 17,
                t ^ (t >> 17)
            ));
            t ^= t >> 17;

            ui.monospace(format!(
                "{:016X}  ⊕  {:016X}  =  {:016X}    (XOR the state with itself shifted left by 5 bits)",
                t,
                t << 5,
                t ^ (t << 5)
            ));
        });

        ui.add_space(16.0);
        generate_random_nums_box(ui, &mut self.rng, &mut self.n_random, &mut self.randoms);
        ui.add_space(16.0);
    }

    fn rng(&self) -> &dyn rngs::ClassicRng {
        &self.rng
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.key = format!("{:08X}", rng.gen::<u32>());
        self.rng.state =
            u32::from_str_radix(&self.key, 16).expect("thread_rng should have provided a valid u32")
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
