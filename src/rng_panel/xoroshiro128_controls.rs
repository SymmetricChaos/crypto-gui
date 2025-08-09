use super::ClassicRngFrame;
use crate::ui_elements::{generate_randoms_box, UiElements};
use rand::{thread_rng, Rng};
use rngs::{
    xorshift::{xoroshiro128::Xoroshiro128, XoshiroScrambler},
    SimpleRng,
};
use strum::IntoEnumIterator;

pub struct XoshiroFrame {
    rng128: Xoroshiro128,
    randoms: String,
    n_random: usize,
}

impl Default for XoshiroFrame {
    fn default() -> Self {
        Self {
            rng128: Default::default(),
            randoms: String::new(),
            n_random: 1,
        }
    }
}

impl XoshiroFrame {}

impl ClassicRngFrame for XoshiroFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/rngs/src/xorshift/xoroshiro128.rs",
        );
        ui.add_space(16.0);

        ui.randomize_reset_rng(self);

        ui.add_space(16.0);
        ui.horizontal(|ui| {
            ui.subheading("State");
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                self.randomize();
            }
        });
        for i in 0..2 {
            ui.u64_hex_edit(&mut self.rng128.state[i]);
        }

        ui.add_space(16.0);
        ui.subheading("Scrambler");
        for variant in XoshiroScrambler::iter() {
            ui.selectable_value(&mut self.rng128.scrambler, variant, variant.to_string());
        }

        ui.collapsing("scrambler function", |ui| match self.rng128.scrambler {
            XoshiroScrambler::PlusPlus => ui.label("((state[0] + state[1]) <<< 17) + state[0]"),
            XoshiroScrambler::StarStar => ui.label("((state[1] × 5) <<< 7) × 9"),
            XoshiroScrambler::Plus => ui.label("state[0] + state[1]"),
        });

        ui.add_space(16.0);
        ui.horizontal(|ui| {
            if ui.button("step").clicked() {
                self.rng128.next_u32();
            }
            if ui
                .button("jump")
                .on_hover_text("move forward by 2^64 steps")
                .clicked()
            {
                self.rng128.jump();
            }
            if ui
                .button("long jump")
                .on_hover_text("move forward by 2^96 steps")
                .clicked()
            {
                self.rng128.long_jump();
            }
        });

        ui.add_space(16.0);
        generate_randoms_box(ui, &mut self.rng128, &mut self.n_random, &mut self.randoms);
        ui.add_space(16.0);
    }

    fn rng(&mut self) -> &mut dyn rngs::SimpleRng {
        &mut self.rng128
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        for word in self.rng128.state.iter_mut() {
            *word = rng.gen()
        }
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
