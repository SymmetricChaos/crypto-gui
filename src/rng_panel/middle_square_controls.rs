use super::ClassicRngFrame;
use crate::ui_elements::UiElements;
use egui::{Button, DragValue};
use rand::{thread_rng, Rng};
use rngs::{middle_square::MiddleSquare, ClassicRng};

pub struct MiddleSquareFrame {
    rng: MiddleSquare,
}

impl Default for MiddleSquareFrame {
    fn default() -> Self {
        Self {
            rng: Default::default(),
        }
    }
}

impl MiddleSquareFrame {}

impl ClassicRngFrame for MiddleSquareFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.subheading("Width");
        if ui
            .add(DragValue::new(&mut self.rng.width).clamp_range(2..=8))
            .changed()
        {
            self.rng.state = self.rng.state % (10_u64.pow((self.rng.width + 1) as u32));
        };
        ui.label(format!("{:0w$}", self.rng.state, w = self.rng.width));
        ui.add_space(16.0);
        if self.rng.width % 2 == 0 {
            if ui.button("step").clicked() {
                self.rng.step();
            };
            ui.error_text("");
        } else {
            ui.add_enabled(false, Button::new("step"));
            ui.error_text("width must be even");
        }
    }

    fn rng(&self) -> &dyn rngs::ClassicRng {
        &self.rng
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.rng.state = rng.gen_range(0..10_u64.pow((self.rng.width + 2) as u32) - 1)
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
