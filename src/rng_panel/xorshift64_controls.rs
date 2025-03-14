use super::ClassicRngFrame;
use crate::ui_elements::{generate_random_u32s_box, UiElements};
use rand::{thread_rng, Rng};
use rngs::{
    xorshift::{
        xorshift64_generic::{Xorshift64, XorshiftRule, XorshiftScrambler},
        TRIPLES_64,
    },
    ClassicRng,
};
use strum::IntoEnumIterator;

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
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/rngs/src/xorshift/xorshift_generic.rs",
        );

        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.subheading("Seed Value");
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                let mut rng = thread_rng();
                self.rng.state = rng.gen::<u64>();
            }
        });
        ui.label("Any value other than zero is a valid state.");
        ui.u64_hex_edit(&mut self.rng.state);

        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.subheading("Triple");
            if ui
                .button("🎲")
                .on_hover_text("random maximum length triple")
                .clicked()
            {
                let mut rng = thread_rng();
                let t = TRIPLES_64[rng.gen_range(0..TRIPLES_64.len())];
                self.rng.triple.0 = t.0 as u64;
                self.rng.triple.1 = t.1 as u64;
                self.rng.triple.2 = t.2 as u64;
            }
        });
        ui.add_space(4.0);
        ui.label("There are 275 triples that produce a maximum length sequence of 2^64-1 values.");
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.label("(");
            ui.u64_drag_value_dec(&mut self.rng.triple.0);
            ui.label(",");
            ui.u64_drag_value_dec(&mut self.rng.triple.1);
            ui.label(",");
            ui.u64_drag_value_dec(&mut self.rng.triple.2);
            ui.label(")");
        });

        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.subheading("Rule");
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                let mut rng = thread_rng();
                match rng.gen_range(0..8) {
                    0 => self.rng.rule = XorshiftRule::A0,
                    1 => self.rng.rule = XorshiftRule::A1,
                    2 => self.rng.rule = XorshiftRule::A2,
                    3 => self.rng.rule = XorshiftRule::A3,
                    4 => self.rng.rule = XorshiftRule::A4,
                    5 => self.rng.rule = XorshiftRule::A5,
                    6 => self.rng.rule = XorshiftRule::A6,
                    7 => self.rng.rule = XorshiftRule::A7,
                    _ => unreachable!("integer not in range 0..8 was generated"),
                }
            }
        });
        ui.add_space(4.0);
        ui.label("There are eight shift rules that can be used. These are named follwing Vigna (2016) and the shifts used can be seen below.");
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            for variant in XorshiftRule::iter() {
                ui.selectable_value(&mut self.rng.rule, variant, variant.to_string());
            }
        });
        ui.add_space(4.0);
        ui.monospace(self.rng.rule.rule());

        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.subheading("Scrambler");
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                let mut rng = thread_rng();
                match rng.gen_range(0..3) {
                    0 => self.rng.scrambler = XorshiftScrambler::None,
                    1 => self.rng.scrambler = XorshiftScrambler::Plus,
                    2 => self.rng.scrambler = XorshiftScrambler::Star32,
                    3 => self.rng.scrambler = XorshiftScrambler::Star8,
                    4 => self.rng.scrambler = XorshiftScrambler::Star2,
                    _ => unreachable!("integer not in range 0..5 was generated"),
                }
            }
        });
        ui.add_space(4.0);
        ui.label("The raw outputs of an xorshift generator have easily detectable patterns. A scrambling step improves the output.");
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            for variant in XorshiftScrambler::iter() {
                ui.selectable_value(&mut self.rng.scrambler, variant, variant.to_string());
            }
        });
        ui.add_space(4.0);
        match self.rng.scrambler {
            XorshiftScrambler::None => ui.label("No scrambling step."),
            XorshiftScrambler::Plus => ui.label("For a 32-bit output the top and bottom half of the state are added together. For a 64-bit output two consecutive values are added together."),
            XorshiftScrambler::Star32 => ui.label("Performs multiplication by 2685821657736338717. This is invertible so equidistribution is preserved."),
            XorshiftScrambler::Star8 =>  ui.label("Performs multiplication by 1181783497276652981. This is invertible so equidistribution is preserved."),
            XorshiftScrambler::Star2 =>  ui.label("Performs multiplication by 8372773778140471301. This is invertible so equidistribution is preserved."),
        };

        ui.add_space(16.0);
        if ui.button("step").clicked() {
            self.rng.next_u32();
        }

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
