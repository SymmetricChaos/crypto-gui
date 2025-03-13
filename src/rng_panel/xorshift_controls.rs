use super::ClassicRngFrame;
use crate::ui_elements::{generate_random_u32s_box, UiElements};
use rand::{thread_rng, Rng};
use rngs::{
    xorshift::{
        xorshift64_generic::{Xorshift64, XorshiftMatrix, XorshiftScrambler},
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
            ui.subheading("Matrix");
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                let mut rng = thread_rng();
                match rng.gen_range(0..4) {
                    0 => self.rng.matrix = XorshiftMatrix::A0,
                    1 => self.rng.matrix = XorshiftMatrix::A1,
                    2 => self.rng.matrix = XorshiftMatrix::A4,
                    3 => self.rng.matrix = XorshiftMatrix::A5,
                    _ => unreachable!("invalid integer generated"),
                }
            }
        });
        for variant in XorshiftMatrix::iter() {
            ui.selectable_value(&mut self.rng.matrix, variant, variant.to_string());
        }

        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.subheading("Scrambler");
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                let mut rng = thread_rng();
                match rng.gen_range(0..3) {
                    0 => self.rng.scrambler = XorshiftScrambler::None,
                    1 => self.rng.scrambler = XorshiftScrambler::Plus,
                    2 => self.rng.scrambler = XorshiftScrambler::Star32,
                    _ => unreachable!("invalid integer generated"),
                }
            }
        });
        for variant in XorshiftScrambler::iter() {
            ui.selectable_value(&mut self.rng.scrambler, variant, variant.to_string());
        }

        ui.add_space(16.0);
        if ui.button("step").clicked() {
            self.rng.next_u32();
        }
        // ui.collapsing("calculations", |ui| {

        //     let mut t = self.rng.state;
        //     ui.monospace(format!(
        //         "{:016X}  ⊕  {:016X}  =  {:016X}    (XOR the state with itself shifted left by 13 bits)",
        //         t,
        //         t << 13,
        //         t ^ (t << 13)
        //     ));
        //     t ^= t << 13;

        //     ui.monospace(format!(
        //         "{:016X}  ⊕  {:016X}  =  {:016X}    (XOR the state with itself shifted right by 17 bits)",
        //         t,
        //         t >> 7,
        //         t ^ (t >> 7)
        //     ));
        //     t ^= t >> 7;

        //     ui.monospace(format!(
        //         "{:016X}  ⊕  {:016X}  =  {:016X}    (XOR the state with itself shifted left by 5 bits)",
        //         t,
        //         t << 13,
        //         t ^ (t << 13)
        //     ));
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
        self.rng.state = rng.gen::<u64>();
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
