use super::ClassicRngFrame;
use crate::ui_elements::{generate_randoms_box, UiElements};

use rand::{thread_rng, Rng};
use rngs::{splitmix::Splitmix, SimpleRng};

pub struct SplitmixFrame {
    rng: Splitmix,
    randoms: String,
    n_random: usize,
}

impl Default for SplitmixFrame {
    fn default() -> Self {
        Self {
            rng: Default::default(),
            randoms: String::new(),
            n_random: 1,
        }
    }
}

impl SplitmixFrame {}

impl ClassicRngFrame for SplitmixFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/rngs/src/splitmix.rs",
        );
        ui.add_space(8.0);

        ui.randomize_reset_rng(self);
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.subheading("State Value");
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                let mut rng = thread_rng();
                self.rng.state = rng.gen::<u64>();
            }
        });
        ui.label("The 64-bit word that changes each time Splitmix is called.");
        ui.u64_hex_edit(&mut self.rng.state);

        ui.horizontal(|ui| {
            ui.subheading("Gamma Value");
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                let mut rng = thread_rng();
                self.rng.gamma = rng.gen::<u64>();
            }
        });
        ui.label("The 64-bit word that is added to the state each time Splitmix is called.");
        ui.u64_hex_edit(&mut self.rng.gamma);

        ui.add_space(16.0);
        ui.label("Call Splitmix once, advancing the state without producing a value.");
        if ui.button("step").clicked() {
            self.rng.next_u64();
        }

        ui.add_space(16.0);
        ui.label("Produce a new Splitmix instance from the existing one. This generates both a new state and a new value of gamma. With high probability the values produce by the new Splitmix will appear unrelated to the original. In actual usage the original Splitmix is still available, only changed by having stepped twice without producing an output, causing the PRNG to \"split\" in the way it is named for.");
        if ui.button("split").clicked() {
            self.rng = self.rng.split();
        }

        ui.add_space(16.0);
        ui.collapsing("calculations", |ui| {

            ui.subheading("Internal State");
            ui.label(format!("{:016X}", self.rng.state));

            ui.monospace(format!(
                "{:016X}  +  {:016X}  =  {:016X}    (the constant value \"gamma\" is added to the state, producing a simple Weyl sequence)",
                self.rng.state,
                self.rng.gamma,
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
        generate_randoms_box(ui, &mut self.rng, &mut self.n_random, &mut self.randoms);
        ui.add_space(16.0);
    }

    fn rng(&mut self) -> &mut dyn rngs::SimpleRng {
        &mut self.rng
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.rng.state = rng.gen::<u64>();
        self.rng.gamma = rng.gen::<u64>();
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
