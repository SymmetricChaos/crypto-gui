use super::ClassicRngFrame;
use crate::ui_elements::{generate_random_u32s_box, UiElements};
use rand::{thread_rng, Rng};
use rngs::{xorshift::xorshift::Xorshift64, ClassicRng};

pub struct XorshiftFrame {
    rng: Xorshift64,
    randoms: String,
    n_random: usize,
}

impl Default for XorshiftFrame {
    fn default() -> Self {
        Self {
            rng: Default::default(),
            randoms: String::new(),
            n_random: 5,
        }
    }
}

impl XorshiftFrame {}

impl ClassicRngFrame for XorshiftFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/rngs/src/xorshift.rs",
        );
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.subheading("Seed Value");
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                self.randomize();
            }
        });
        ui.u64_hex_edit(&mut self.rng.state);

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
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
