use super::ClassicRngFrame;
use crate::ui_elements::{generate_random_nums_box, UiElements};
use egui::{Button, DragValue, RichText};
use rand::{thread_rng, Rng};
use rngs::{middle_square::MiddleSquare, ClassicRng};
use utils::text_functions::filter_string;

pub struct MiddleSquareFrame {
    rng: MiddleSquare,
    state_string: String,
    position: usize,
    randoms: String,
    n_random: usize,
}

impl Default for MiddleSquareFrame {
    fn default() -> Self {
        let rng = MiddleSquare::default();
        let state_string = rng.state.to_string();
        Self {
            rng,
            state_string,
            position: 3,
            randoms: String::new(),
            n_random: 5,
        }
    }
}

impl MiddleSquareFrame {
    fn show_method(&self, ui: &mut egui::Ui) {
        let mut display_state = String::from("State:  ");
        display_state.push_str(&" ".repeat(self.rng.width / 2));
        display_state.push_str(&format!("{:0w$}", self.rng.state, w = self.rng.width));
        ui.label(RichText::from(display_state).monospace().size(14.0));
        let mut square_string = String::from("Square: ");
        let digit_string = format!(
            "{:0w$}",
            self.rng.state * self.rng.state,
            w = self.rng.width * 2
        );
        square_string.push_str(&digit_string);
        ui.label(RichText::from(&square_string).monospace().size(14.0));
        let mut next_string = String::from("Next:   ");
        next_string.push_str(&" ".repeat(self.rng.width / 2));
        next_string
            .push_str(&digit_string[self.rng.width / 2..self.rng.width + self.rng.width / 2]);
        ui.label(RichText::from(next_string).monospace().size(14.0));
    }
}

impl ClassicRngFrame for MiddleSquareFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.subheading("Choose Width");
        if ui
            .add(
                DragValue::new(&mut self.position)
                    .clamp_range(1..=4)
                    .custom_formatter(|n, _| (2.0 * n).to_string())
                    .speed(0.2),
            )
            .changed()
        {
            self.rng.width = self.position * 2;
            self.rng.state = self.rng.state % (10_u64.pow((self.rng.width) as u32));
        }

        ui.subheading("Seed Value");
        if ui.control_string(&mut self.state_string).changed() {
            filter_string(&mut self.state_string, &"0123456789");
            self.rng.state = self.state_string.parse().unwrap_or(0);
            self.rng.state = self.rng.state % (10_u64.pow((self.rng.width) as u32));
        }

        ui.add_space(16.0);
        self.show_method(ui);
        ui.add_space(16.0);

        if self.rng.width % 2 == 0 {
            if ui.button("step").clicked() {
                self.rng.next_u32();
            };
            ui.error_text("");
        } else {
            ui.add_enabled(false, Button::new("step"));
            ui.error_text("width must be even");
        }

        generate_random_nums_box(ui, &mut self.rng, &mut self.n_random, &mut self.randoms);
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
