use super::ClassicRngFrame;
use crate::ui_elements::{generate_random_u32s_box, UiElements};
use rand::{thread_rng, Rng};
use rngs::{xorshift::xoshiro256::Xoshiro256, ClassicRng};

pub struct XoshiroFrame {
    rng: Xoshiro256,
    randoms: String,
    n_random: usize,
}

impl Default for XoshiroFrame {
    fn default() -> Self {
        Self {
            rng: Default::default(),
            randoms: String::new(),
            n_random: 5,
        }
    }
}

impl XoshiroFrame {}

impl ClassicRngFrame for XoshiroFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/rngs/src/xoshiro.rs",
        );
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.subheading("Seed Values");
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                self.randomize();
            }
        });
        for i in 0..4 {
            ui.u64_hex_edit(&mut self.rng.state[i]);
        }

        // ui.add_space(16.0);
        // ui.subheading("Scrambler");
        // for variant in Scrambler::iter() {
        //     ui.selectable_value(&mut self.rng.scrambler, variant, variant.to_string());
        // }

        // ui.collapsing("scrambler function", |ui| match self.rng.scrambler {
        //     Scrambler::PlusPlus => ui.label("rotate_left_23(state[0] + state[3]) + state[0]"),
        //     Scrambler::StarStar => ui.label("rotate_left_7(state[1] × 5) × 9"),
        //     Scrambler::Plus => ui.label("state[0] + state[3"),
        // });

        ui.add_space(16.0);
        ui.horizontal(|ui| {
            if ui.button("step").clicked() {
                self.rng.next_u32();
            }
            if ui
                .button("jump")
                .on_hover_text("move forward by 2^128 steps")
                .clicked()
            {
                self.rng.jump();
            }
            if ui
                .button("long jump")
                .on_hover_text("move forward by 2^192 steps")
                .clicked()
            {
                self.rng.long_jump();
            }
        });

        ui.add_space(16.0);
        ui.subheading("Internal State");
        ui.monospace(format!(
            "{:016X} {:016X} {:016X} {:016X}",
            self.rng.state[0], self.rng.state[1], self.rng.state[2], self.rng.state[3],
        ));

        // ui.collapsing("calculations", |ui| {
        //     ui.label(
        //         "let output = scrambler_function(state)\nlet t = shift_left_17(state[1])\nstate[2] ^= state[0]\nstate[3] ^= state[1]\nstate[1] ^= state[2]\nstate[0] ^= state[3]\nstate[2] ^= t\nstate[3] = rotate_left_45(state[3])\nreturn output",
        //     );
        // });

        ui.add_space(16.0);
        generate_random_u32s_box(ui, &mut self.rng, &mut self.n_random, &mut self.randoms);
        ui.add_space(16.0);
    }

    fn rng(&self) -> &dyn rngs::ClassicRng {
        &self.rng
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        for word in self.rng.state.iter_mut() {
            *word = rng.gen()
        }
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
