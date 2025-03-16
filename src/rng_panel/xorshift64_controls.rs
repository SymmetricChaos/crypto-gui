use super::ClassicRngFrame;
use crate::{
    integer_drag_value::EditU32,
    ui_elements::{generate_randoms_box, UiElements},
};
use egui::Label;
use rand::{thread_rng, Rng};
use rngs::{
    xorshift::{
        xorshift64_generic::{Xorshift64, XorshiftRule, XorshiftScrambler},
        TRIPLES_64,
    },
    ClassicRng,
};
use strum::IntoEnumIterator;

pub struct Xorshift64Frame {
    rng: Xorshift64,
    randoms: String,
    n_random: usize,
}

impl Default for Xorshift64Frame {
    fn default() -> Self {
        Self {
            rng: Default::default(),
            randoms: String::new(),
            n_random: 5,
        }
    }
}

impl Xorshift64Frame {
    fn random_triple(&mut self) {
        let mut rng = thread_rng();
        let t = TRIPLES_64[rng.gen_range(0..TRIPLES_64.len())];
        self.rng.triple.0 = t.0 as u64;
        self.rng.triple.1 = t.1 as u64;
        self.rng.triple.2 = t.2 as u64;
    }

    fn random_rule(&mut self) {
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

    fn random_scrambler(&mut self) {
        let mut rng = thread_rng();
        match rng.gen_range(0..7) {
            0 => self.rng.scrambler = XorshiftScrambler::None,
            1 => self.rng.scrambler = XorshiftScrambler::Plus,
            2 => self.rng.scrambler = XorshiftScrambler::Star32,
            3 => self.rng.scrambler = XorshiftScrambler::Star8,
            4 => self.rng.scrambler = XorshiftScrambler::Star2,
            5 => self.rng.scrambler = XorshiftScrambler::WowPlus,
            6 => self.rng.scrambler = XorshiftScrambler::WowXor,
            _ => unreachable!("integer not in range 0..7 was generated"),
        }
    }
}

impl ClassicRngFrame for Xorshift64Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/rngs/src/xorshift/xorshift64_generic.rs",
        );

        ui.randomize_reset_rng(self);

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
                self.random_triple();
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
                self.random_rule();
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
                self.random_scrambler();
            }
        });
        ui.add_space(4.0);
        ui.label("The raw outputs of an xorshift generator have easily detectable patterns. Many forms of scrambling are used to alter the output (not the state).");
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            for variant in XorshiftScrambler::iter() {
                ui.selectable_value(&mut self.rng.scrambler, variant, variant.to_string());
            }
        });
        ui.add_space(4.0);
        let is_wow = self.rng.scrambler == XorshiftScrambler::WowPlus
            || self.rng.scrambler == XorshiftScrambler::WowXor;
        ui.add_visible(is_wow, Label::new("Counter"));
        ui.add_visible(is_wow, EditU32::new(&mut self.rng.ctr));
        ui.add_visible(is_wow, Label::new("Weyl Constant"));
        ui.add_visible(is_wow, EditU32::new(&mut self.rng.weyl));
        ui.add_space(4.0);
        match self.rng.scrambler {
            XorshiftScrambler::None =>    ui.label("No scrambling step."),
            XorshiftScrambler::Plus =>    ui.label("The top and bottom half of the state are added together."),
            XorshiftScrambler::Star32 =>  ui.label("Performs multiplication by 2685821657736338717. This is invertible so equidistribution and period are preserved"),
            XorshiftScrambler::Star8 =>   ui.label("Performs multiplication by 1181783497276652981. This is invertible so equidistribution and period are preserved."),
            XorshiftScrambler::Star2 =>   ui.label("Performs multiplication by 8372773778140471301. This is invertible so equidistribution and period are preserved."),
            XorshiftScrambler::WowPlus => ui.label("Adds a counter value to the output. The counter is stepped by a constant each time, producing a Weyl sequence so long as the constant is odd. This results in a longer period."),
            XorshiftScrambler::WowXor =>  ui.label("XORs a counter value into the output. The counter is stepped by a constant each time, producing a Weyl sequence so long as the constant is odd. This results in a longer period."),
        };

        ui.add_space(16.0);
        if ui.button("step").clicked() {
            self.rng.next_u32();
        }

        ui.add_space(16.0);
        generate_randoms_box(ui, &mut self.rng, &mut self.n_random, &mut self.randoms);
        ui.add_space(16.0);
    }

    fn rng(&self) -> &dyn rngs::ClassicRng {
        &self.rng
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.rng.state = rng.gen::<u64>();
        self.random_rule();
        self.random_scrambler();
        self.random_triple();
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
