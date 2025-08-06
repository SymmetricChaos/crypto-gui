use super::ClassicRngFrame;
use crate::ui_elements::{generate_randoms_box, UiElements};
use rand::{thread_rng, Rng};
use rngs::{
    xorshift::{
        xoshiro128::Xoshiro128, xoshiro256::Xoshiro256, xoshiro512::Xoshiro512, XoshiroScrambler,
    },
    ClassicRng,
};
use strum::IntoEnumIterator;

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::EnumIter)]
enum XoshiroSize {
    W128,
    W256,
    W512,
}

impl XoshiroSize {
    fn _string(&self) -> &str {
        match self {
            XoshiroSize::W128 => "Xoshiro128",
            XoshiroSize::W256 => "Xoshiro256",
            XoshiroSize::W512 => "Xoshiro512",
        }
    }
}

pub struct XoshiroFrame {
    _rng128: Xoshiro128,
    rng256: Xoshiro256,
    _rng512: Xoshiro512,
    _state_size: XoshiroSize,
    randoms: String,
    n_random: usize,
}

impl Default for XoshiroFrame {
    fn default() -> Self {
        Self {
            _rng128: Default::default(),
            rng256: Default::default(),
            _rng512: Default::default(),
            _state_size: XoshiroSize::W256,
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
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/rngs/src/xoshiro.rs",
        );
        ui.add_space(16.0);

        ui.randomize_reset_rng(self);

        // ui.subheading("State Size");
        // for variant in XoshiroSize::iter() {
        //     ui.selectable_value(&mut self.state_size, variant, variant.string());
        // }

        ui.add_space(16.0);
        ui.horizontal(|ui| {
            ui.subheading("State");
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                self.randomize();
            }
        });
        for i in 0..4 {
            ui.u64_hex_edit(&mut self.rng256.state[i]);
        }

        ui.add_space(16.0);
        ui.subheading("Scrambler");
        for variant in XoshiroScrambler::iter() {
            ui.selectable_value(&mut self.rng256.scrambler, variant, variant.to_string());
        }

        ui.collapsing("scrambler function", |ui| match self.rng256.scrambler {
            XoshiroScrambler::PlusPlus => {
                ui.label("rotate_left_23(state[0] + state[3]) + state[0]")
            }
            XoshiroScrambler::StarStar => ui.label("rotate_left_7(state[1] × 5) × 9"),
            XoshiroScrambler::Plus => ui.label("state[0] + state[3"),
        });

        ui.add_space(16.0);
        ui.horizontal(|ui| {
            if ui.button("step").clicked() {
                self.rng256.next_u32();
            }
            if ui
                .button("jump")
                .on_hover_text("move forward by 2^128 steps")
                .clicked()
            {
                self.rng256.jump();
            }
            if ui
                .button("long jump")
                .on_hover_text("move forward by 2^192 steps")
                .clicked()
            {
                self.rng256.long_jump();
            }
        });

        ui.add_space(16.0);
        generate_randoms_box(ui, &mut self.rng256, &mut self.n_random, &mut self.randoms);
        ui.add_space(16.0);
    }

    fn rng(&mut self) -> &mut dyn rngs::ClassicRng {
        &mut self.rng256
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        for word in self.rng256.state.iter_mut() {
            *word = rng.gen()
        }
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
