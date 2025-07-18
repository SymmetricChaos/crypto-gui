use std::collections::VecDeque;

use egui::{DragValue, TextStyle};
use rngs::{
    lfg::{FibOp32, Lfg32},
    ClassicRng,
};

use crate::ui_elements::{filter_and_parse_u32, generate_randoms_box, UiElements};

use super::ClassicRngFrame;

pub struct LfgFrame {
    rng: Lfg32,
    vector_length: usize,
    state_strings: VecDeque<String>,
    randoms: String,
    n_random: usize,
}

impl Default for LfgFrame {
    fn default() -> Self {
        let rng = Lfg32::default();
        let vector_length = rng.state.len();
        let mut s = Self {
            rng,
            vector_length,
            state_strings: VecDeque::from([]),
            randoms: String::new(),
            n_random: 5,
        };
        s.set_state_strings();
        s
    }
}

impl LfgFrame {
    fn set_state_strings(&mut self) {
        self.state_strings = self.rng.state.iter().map(|n| n.to_string()).collect();
    }

    fn input_control(ui: &mut egui::Ui, string: &mut String, n: &mut u32) {
        if ui
            .add_sized(
                [40.0, 20.0],
                egui::TextEdit::singleline(string)
                    .font(TextStyle::Monospace)
                    .clip_text(false),
            )
            .changed()
        {
            filter_and_parse_u32(n, string);
        }
    }
}

impl ClassicRngFrame for LfgFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/rngs/src/lfg.rs",
        );
        ui.add_space(8.0);

        ui.subheading("Tap Location");
        ui.add(DragValue::new(&mut self.rng.tap).range(1..=(self.vector_length - 1)));

        ui.add_space(8.0);
        ui.subheading("Modulus");
        ui.add(DragValue::new(&mut self.rng.modulus));

        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.subheading("State");
            if ui
                .add(DragValue::new(&mut self.vector_length).range(2..=20))
                .changed()
            {
                self.rng.state.truncate(self.vector_length);
                while self.rng.state.len() < self.vector_length {
                    self.rng.state.push_back(1)
                }
                self.rng.tap = self.rng.tap.min(self.rng.state.len() - 1);
                self.set_state_strings();
            };
        });

        ui.label("Numbers stored in the vector");
        ui.horizontal(|ui| {
            for (i, (s, n)) in self
                .state_strings
                .iter_mut()
                .zip(self.rng.state.iter_mut())
                .enumerate()
            {
                if i == self.rng.tap || i == 0 {
                    ui.subheading("[");
                }
                Self::input_control(ui, s, n);
                if i == self.rng.tap || i == 0 {
                    ui.subheading("]");
                }
            }
        });

        ui.add_space(8.0);
        if ui.button("step").clicked() {
            self.rng.next_u32();
        }

        ui.add_space(8.0);
        ui.subheading("Operation");
        ui.selectable_value(&mut self.rng.op, FibOp32::Add, "Addition");
        ui.selectable_value(&mut self.rng.op, FibOp32::Mul, "Multiplication");
        ui.selectable_value(&mut self.rng.op, FibOp32::Xor, "Bitwise XOR");

        ui.add_space(8.0);
        generate_randoms_box(ui, &mut self.rng, &mut self.n_random, &mut self.randoms);
        self.set_state_strings();
    }

    fn rng(&mut self) -> &mut dyn rngs::ClassicRng {
        &mut self.rng
    }

    fn randomize(&mut self) {
        todo!()
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
