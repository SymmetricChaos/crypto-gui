use super::ClassicRngFrame;
use crate::ui_elements::{generate_random_nums_box, UiElements};

use rand::{thread_rng, Rng};
use rngs::splitmix::Splitmix;

pub struct SplitmixFrame {
    rng: Splitmix,
    key: String,
    randoms: String,
    n_random: usize,
}

impl Default for SplitmixFrame {
    fn default() -> Self {
        Self {
            rng: Default::default(),
            key: String::from("DEADBEEF42"),
            randoms: String::new(),
            n_random: 5,
        }
    }
}

impl SplitmixFrame {}

impl ClassicRngFrame for SplitmixFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.add_space(16.0);

        ui.horizontal(|ui| {
            ui.subheading("Seed Value");
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                self.randomize();
            }
        });
        ui.label("Seed should be provided as a string of hexadecimal digits.");
        if ui.text_edit_multiline(&mut self.key).changed() {
            self.key = self
                .key
                .chars()
                .filter(|c| c.is_ascii_hexdigit())
                .take(16)
                .collect();
            self.rng.state = u64::from_str_radix(&self.key, 16)
                .expect("filtering should force Splitmix64 seed to be valid");
        }

        ui.add_space(16.0);
        ui.subheading("Internal State");
        ui.label(format!("{:016X}", self.rng.state));

        ui.add_space(16.0);
        if ui.button("step").clicked() {
            self.rng.next_u64();
        }
        ui.collapsing("calculations", |ui| {
            ui.monospace(format!(
                "{:016X}  +  9e3779b97f4a7c15  =  {:016X}    (a constant is added to the state)",
                self.rng.state,
                self.rng.state.wrapping_add(0x9e3779b97f4a7c15)
            ));
            let mut t = self.rng.state.wrapping_add(0x9e3779b97f4a7c15);
            ui.add_space(4.0);
            ui.monospace("(the state is then copied to a variable, the following calculations change the output but not the state)");
            ui.add_space(4.0);
            ui.monospace(format!(
                "{:016X}  ⊕  {:016X}  =  {:016X}    (XOR the value with itself shifted right by 30 bits)",
                t,
                t >> 30,
                t ^ (t>>30)
            ));
            t ^= t >> 30;
            ui.add_space(4.0);
            ui.monospace(format!(
                "{:016X}  ×  {:016X}  =  {:016X}    (multiply the value by a constant)",
                t,
                0xbf58476d1ce4e5b9_u64,
                t.wrapping_mul(0xbf58476d1ce4e5b9)
            ));
            t = t.wrapping_mul(0xbf58476d1ce4e5b9);
            ui.add_space(4.0);
            ui.monospace(format!(
                "{:016X}  ⊕  {:016X}  =  {:016X}    (XOR the value with itself shifted right by 27 bits)",
                t,
                t >> 27,
                t ^ (t>>27)
            ));
            t ^= t >> 27;
            ui.add_space(4.0);
            ui.monospace(format!(
                "{:016X}  ×  {:016X}  =  {:016X}    (multiply the value by a constant)",
                t,
                0x94d049bb133111eb_u64,
                t.wrapping_mul(0x94d049bb133111eb)
            ));
            t = t.wrapping_mul(0x94d049bb133111eb);
            ui.add_space(4.0);
            ui.monospace(format!(
                "{:016X}  ⊕  {:016X}  =  {:016X}    (XOR the value with itself shifted right by 31 bits)",
                t,
                t >> 31,
                t ^ (t>>31)
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
        self.rng.state = rng.gen::<u64>();
        self.key = format!("{:016X}", self.rng.state);
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
