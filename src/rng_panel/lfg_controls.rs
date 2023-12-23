use std::collections::VecDeque;

use egui::{DragValue, TextStyle};
use rngs::{
    lfg::{FibOp, Lfg},
    ClassicRng,
};

use crate::ui_elements::{filter_and_parse_u32, generate_random_nums_box, UiElements};

use super::ClassicRngFrame;

pub struct LfgFrame {
    rng: Lfg,
    vector_length: usize,
    state_strings: VecDeque<String>,
    randoms: String,
}

impl Default for LfgFrame {
    fn default() -> Self {
        let mut s = Self {
            rng: Default::default(),
            vector_length: 16,
            state_strings: VecDeque::from([]),
            randoms: String::new(),
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
        ui.subheading("Vector Length");
        if ui
            .add(DragValue::new(&mut self.vector_length).clamp_range(2..=20))
            .changed()
        {
            self.rng.state.truncate(self.vector_length);
            while self.rng.state.len() < self.vector_length {
                self.rng.state.push_back(1)
            }
            self.rng.tap = self.rng.tap.min(self.rng.state.len() - 1);
            self.set_state_strings();
        };
        ui.add_space(8.0);

        ui.subheading("Tap Location");
        ui.add(DragValue::new(&mut self.rng.tap).clamp_range(1..=(self.vector_length - 1)));

        ui.add_space(8.0);
        ui.subheading("Operation");
        ui.selectable_value(&mut self.rng.op, FibOp::Add, "Addition");
        ui.selectable_value(&mut self.rng.op, FibOp::Mul, "Multiplication");
        ui.selectable_value(&mut self.rng.op, FibOp::Xor, "Bitwise XOR");

        ui.add_space(8.0);
        ui.subheading("State");
        ui.label("Numbers stored in the vector");
        for (s, n) in self.state_strings.iter_mut().zip(self.rng.state.iter_mut()) {
            Self::input_control(ui, s, n);
        }

        ui.add_space(8.0);
        if ui.button("step").clicked() {
            self.rng.next_u32();
        }

        ui.add_space(8.0);
        generate_random_nums_box(ui, &mut self.rng, 10, &mut self.randoms);
        self.set_state_strings();
    }

    fn rng(&self) -> &dyn rngs::ClassicRng {
        &self.rng
    }

    fn randomize(&mut self) {
        todo!()
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
