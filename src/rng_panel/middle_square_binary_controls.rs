use super::ClassicRngFrame;
use crate::ui_elements::{generate_randoms_box, UiElements};
use egui::RichText;
use rand::{thread_rng, Rng};
use rngs::middle_square_binary::{MSBSize, MiddleSquareBinary};

pub struct MiddleSquareBinaryFrame {
    rng: MiddleSquareBinary,
    seed_state: u64,
    randoms: String,
    n_random: usize,
}

impl Default for MiddleSquareBinaryFrame {
    fn default() -> Self {
        Self {
            rng: MiddleSquareBinary::default(),
            seed_state: 255,
            randoms: String::new(),
            n_random: 5,
        }
    }
}

impl MiddleSquareBinaryFrame {
    fn show_method(&self, ui: &mut egui::Ui) {
        let mut current_state = String::from("State:  ");
        current_state.push_str(&" ".repeat(self.rng.width.quarter_size() / 2));
        match self.rng.width {
            MSBSize::B64 => {
                current_state.push_str(&format!("{:016x}", self.rng.state));
            }
            MSBSize::B32 => {
                current_state.push_str(&format!("{:08x}", self.rng.state));
            }
            MSBSize::B16 => {
                current_state.push_str(&format!("{:04x}", self.rng.state));
            }
            MSBSize::B8 => {
                current_state.push_str(&format!("{:02x}", self.rng.state));
            }
        }
        ui.label(RichText::from(current_state).monospace().size(14.0));
        let mut square_string = String::from("Square: ");
        let next_state = self.rng.state * self.rng.state;
        let digit_string = match self.rng.width {
            MSBSize::B64 => format!("{:032x}", next_state),
            MSBSize::B32 => format!("{:016x}", next_state),
            MSBSize::B16 => format!("{:08x}", next_state),
            MSBSize::B8 => format!("{:04x}", next_state),
        };
        square_string.push_str(&digit_string);
        ui.label(RichText::from(&square_string).monospace().size(14.0));
        let mut next_string = String::from("Next:   ");
        next_string.push_str(&" ".repeat(self.rng.width.quarter_size() / 2));
        let next_val = self.rng.peek_next();
        let next = match self.rng.width {
            MSBSize::B64 => format!("{:016x}", next_val),
            MSBSize::B32 => format!("{:08x}", next_val),
            MSBSize::B16 => format!("{:04x}", next_val),
            MSBSize::B8 => format!("{:02x}", next_val),
        };
        next_string.push_str(&next);
        ui.label(RichText::from(next_string).monospace().size(14.0));
    }
}

impl ClassicRngFrame for MiddleSquareBinaryFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/rngs/src/middle_square_binary.rs",
        );
        ui.add_space(8.0);

        ui.subheading("Size");
        ui.label("Each routine uses a state of twice as many bits as the output. The 8-bit and 16-bit versions fall into repeating sequences almost immedaitely and are only of academic interest. The 32-bit version can be optimized to run extremely on modern hardware. Although the 64-bit version often has a very long period it is still a very weak PRNG.");
        if ui
            .selectable_value(&mut self.rng.width, MSBSize::B64, "64-Bit")
            .clicked()
            || ui
                .selectable_value(&mut self.rng.width, MSBSize::B32, "32-Bit")
                .clicked()
            || ui
                .selectable_value(&mut self.rng.width, MSBSize::B16, "16-Bit")
                .clicked()
            || ui
                .selectable_value(&mut self.rng.width, MSBSize::B8, "8-Bit")
                .clicked()
        {
            self.rng.state &= self.rng.width.mask();
        };

        ui.horizontal(|ui| {
            ui.subheading("Seed State");
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                self.randomize();
            }
        });
        if ui.u64_hex_edit(&mut self.seed_state).lost_focus() {
            self.rng.state = self.seed_state as u128;
            self.rng.state &= self.rng.width.mask();
        }

        ui.add_space(16.0);
        self.show_method(ui);
        ui.add_space(16.0);

        generate_randoms_box(ui, &mut self.rng, &mut self.n_random, &mut self.randoms);
    }

    fn rng(&self) -> &dyn rngs::ClassicRng {
        &self.rng
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.seed_state = rng.gen();
        self.rng.state = self.seed_state as u128;
        self.rng.state &= self.rng.width.mask();
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
